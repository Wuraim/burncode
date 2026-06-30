use crate::app::state::AppState;
use crate::opencode::types::{ExecuteCommandRequest, ModelRef, PromptPart, SendPromptRequest};
use crate::router::policy::{choose_route, RouteDecision};
use serde_json::Value;

fn extract_assistant_outcome(value: &Value) -> Option<(String, String, f64, u64, u64, u64, bool)> {
    let info = value.get("info")?;
    if info.get("role")?.as_str()? != "assistant" {
        return None;
    }
    let provider_id = info.get("providerID")?.as_str()?.to_string();
    let model_id = info.get("modelID")?.as_str()?.to_string();
    let cost = info.get("cost").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let tokens = info.get("tokens").cloned().unwrap_or_default();
    let input_tokens = tokens.get("input").and_then(|v| v.as_u64()).unwrap_or(0);
    let output_tokens = tokens.get("output").and_then(|v| v.as_u64()).unwrap_or(0);
    let reasoning_tokens = tokens
        .get("reasoning")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let failed = info.get("error").is_some();
    Some((provider_id, model_id, cost, input_tokens, output_tokens, reasoning_tokens, !failed))
}

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
    Ok(choose_route(&text, &providers, &config, &guard.analytics))
}

#[tauri::command]
pub async fn send_prompt(
    id: String,
    text: String,
    provider_id: Option<String>,
    model_id: Option<String>,
    agent: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let analytics_store = {
        let guard = state.lock().await;
        guard.analytics_store.clone()
    };
    let difficulty = crate::router::policy::classify_difficulty(&text).to_string();
    let response = {
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
            agent,
        };
        client.send_prompt(&id, request).await?
    };

    if let Some((provider_id, model_id, cost, input, output, reasoning, success)) =
        extract_assistant_outcome(&response)
    {
        let mut guard = state.lock().await;
        guard.analytics.record_outcome(
            &provider_id,
            &model_id,
            success,
            cost,
            input,
            output,
            reasoning,
            0,
            &difficulty,
        );
        let _ = analytics_store.save(&guard.analytics).await;
    }

    Ok(response)
}

#[tauri::command]
pub async fn send_prompt_async(
    id: String,
    text: String,
    provider_id: Option<String>,
    model_id: Option<String>,
    agent: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
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
        agent,
    };
    client.prompt_async(&id, request).await
}

#[tauri::command]
pub async fn record_assistant_outcome(
    provider_id: String,
    model_id: String,
    cost: f64,
    input_tokens: u64,
    output_tokens: u64,
    reasoning_tokens: u64,
    difficulty: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let store = {
        let guard = state.lock().await;
        guard.analytics_store.clone()
    };
    let mut guard = state.lock().await;
    guard.analytics.record_outcome(
        &provider_id,
        &model_id,
        true,
        cost,
        input_tokens,
        output_tokens,
        reasoning_tokens,
        0,
        &difficulty,
    );
    store.save(&guard.analytics).await
}

#[tauri::command]
pub async fn respond_to_permission(
    session_id: String,
    permission_id: String,
    response: String,
    remember: Option<bool>,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.respond_to_permission(&session_id, &permission_id, &response, remember).await
}

#[tauri::command]
pub async fn update_session(
    id: String,
    title: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.update_session(&id, title).await
}

#[tauri::command]
pub async fn delete_session(id: String, state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.delete_session(&id).await
}

#[tauri::command]
pub async fn fork_session(
    id: String,
    message_id: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.fork_session(&id, message_id.as_deref()).await
}

#[tauri::command]
pub async fn list_commands(state: tauri::State<'_, AppState>) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.get_commands().await
}

#[tauri::command]
pub async fn execute_command(
    id: String,
    command: String,
    arguments: Option<String>,
    agent: Option<String>,
    model_id: Option<String>,
    provider_id: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    let model = provider_id.zip(model_id).map(|(p, m)| crate::opencode::types::ModelRef {
        provider_id: p,
        model_id: m,
    });
    let request = ExecuteCommandRequest {
        command,
        arguments,
        agent,
        model,
    };
    client.execute_command(&id, request).await
}

#[tauri::command]
pub async fn abort_session(id: String, state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    client.abort_session(&id).await
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
