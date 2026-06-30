use crate::app::events::{start_event_stream, stop_event_stream};
use crate::app::state::AppState;
use crate::opencode::client::OpencodeClient;
use crate::store::settings::{AppSettings, ConnectionProfile};
use serde_json::Value;

#[tauri::command]
pub async fn load_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    let guard = state.lock().await;
    Ok(guard.settings.clone())
}

#[tauri::command]
pub async fn save_settings(
    settings: AppSettings,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let store = {
        let guard = state.lock().await;
        guard.settings_store.clone()
    };
    store.save(&settings).await?;
    state.lock().await.settings = settings;
    Ok(())
}

#[tauri::command]
pub async fn connect(
    profile: ConnectionProfile,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let client = OpencodeClient::new(&profile.server_url, &profile.username, &profile.password);
    let health = client.health().await?;

    // Persist settings on successful connection.
    let settings = AppSettings {
        profile: Some(profile.clone()),
        ..state.lock().await.settings.clone()
    };
    let store = {
        let guard = state.lock().await;
        guard.settings_store.clone()
    };
    store.save(&settings).await?;

    {
        let mut guard = state.lock().await;
        guard.settings = settings;
        guard.client = Some(client.clone());
        if let Some(handle) = guard.event_handle.take() {
            handle.abort();
        }
    }

    start_event_stream(app, state.inner().clone());
    Ok(serde_json::to_value(health).unwrap())
}

#[tauri::command]
pub async fn disconnect(state: tauri::State<'_, AppState>) -> Result<(), String> {
    stop_event_stream(state.inner().clone());
    let mut guard = state.lock().await;
    guard.client = None;
    Ok(())
}

#[tauri::command]
pub async fn is_connected(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(state.lock().await.client.is_some())
}
