use crate::router::scoring::build_decision;
use crate::store::routing_analytics::RoutingAnalytics;
use serde_json::Value;

pub use crate::router::types::RouteDecision;

pub fn classify_difficulty(text: &str) -> &'static str {
    let chars = text.chars().count();
    let word_count = text.split_whitespace().count();
    if chars > 4000 || word_count > 600 {
        "long_context"
    } else if word_count > 200 || text.contains("refactor") || text.contains("architect") {
        "deep"
    } else if word_count > 60 || text.contains("implement") || text.contains("debug") {
        "balanced"
    } else {
        "fast"
    }
}

pub fn choose_route(
    text: &str,
    providers: &Value,
    config: &Value,
    analytics: &RoutingAnalytics,
) -> Option<RouteDecision> {
    build_decision(text, providers, config, analytics)
}
