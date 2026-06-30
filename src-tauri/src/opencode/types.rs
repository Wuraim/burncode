use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub healthy: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPromptRequest {
    pub parts: Vec<PromptPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRef {
    pub provider_id: String,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpencodeEvent {
    pub event_type: String,
    #[serde(default)]
    pub data: Value,
}

impl OpencodeEvent {
    pub fn from_sse(event: &str, data: Value) -> Self {
        let event_type = if event.is_empty() { "message".into() } else { event.into() };
        Self { event_type, data }
    }
}
