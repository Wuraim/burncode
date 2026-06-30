use crate::app::state::AppState;
use serde_json::Value;

#[tauri::command]
pub async fn list_providers(state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_providers().await
}

#[tauri::command]
pub async fn list_provider_auth_methods(state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_provider_auth_methods().await
}

#[tauri::command]
pub async fn list_config_providers(state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_config_providers().await
}

#[tauri::command]
pub async fn set_provider_auth(
    provider_id: String,
    body: Value,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.set_provider_auth(&provider_id, body).await
}
