use crate::app::state::AppState;
use crate::opencode::types::{ModelRef, PromptPart, SendPromptRequest};
use crate::router::policy::{choose_route, RouteDecision};
use serde_json::Value;

#[tauri::command]
pub async fn list_sessions(state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_sessions().await
}

#[tauri::command]
pub async fn create_session(
    title: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.create_session(title).await
}

#[tauri::command]
pub async fn get_session(id: String, state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_session(&id).await
}

#[tauri::command]
pub async fn get_session_messages(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_messages(&id).await
}

#[tauri::command]
pub async fn suggest_route(
    text: String,
    state: tauri::State<'_, AppState>,
) -> Result<Option<RouteDecision>, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    let providers = client.get_providers().await?;
    let config = client.get_config_providers().await?;
    Ok(choose_route(&text, &providers, &config))
}

#[tauri::command]
pub async fn send_prompt(
    id: String,
    text: String,
    provider_id: Option<String>,
    model_id: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    let model = provider_id.zip(model_id).map(|(provider_id, model_id)| ModelRef {
        provider_id,
        model_id,
    });
    let request = SendPromptRequest {
        parts: vec![PromptPart {
            part_type: "text".into(),
            text,
        }],
        model,
    };
    client.send_prompt(&id, request).await
}

#[tauri::command]
pub async fn get_todos(id: String, state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_todos(&id).await
}

#[tauri::command]
pub async fn get_diff(id: String, state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_diff(&id).await
}
