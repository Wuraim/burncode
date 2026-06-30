use crate::app::state::AppState;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Default)]
pub struct TelemetrySnapshot {
    pub request_count: u64,
    pub error_count: u64,
    pub avg_latency_ms: f64,
    pub last_event: Option<Value>,
}

#[tauri::command]
pub async fn get_telemetry(_state: tauri::State<'_, AppState>) -> Result<TelemetrySnapshot, String> {
    Ok(TelemetrySnapshot::default())
}
