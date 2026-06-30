use crate::app::state::AppState;
use serde_json::Value;

#[tauri::command]
pub async fn list_agents(state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_agents().await
}
