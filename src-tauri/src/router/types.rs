#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct RouteDecision {
    pub provider_id: String,
    pub model_id: String,
    pub difficulty: String,
    pub reason: String,
    pub source: String,
    #[serde(default)]
    pub candidates: Vec<RouteCandidate>,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct RouteCandidate {
    pub provider_id: String,
    pub model_id: String,
    pub score: f64,
    pub reason: String,
}
