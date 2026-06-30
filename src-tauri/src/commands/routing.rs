use crate::app::state::AppState;
use crate::store::routing_analytics::RoutingAnalytics;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct RouteBucketMeta {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoutePolicyMeta {
    pub buckets: Vec<RouteBucketMeta>,
    pub selection_rule: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteConfig {
    pub settings: crate::store::settings::RoutingSettings,
    pub policy: RoutePolicyMeta,
    pub providers: Value,
    pub defaults: Value,
    pub analytics: RoutingAnalytics,
    pub pinned_providers: Vec<String>,
}

#[tauri::command]
pub async fn get_route_config(state: tauri::State<'_, AppState>) -> Result<RouteConfig, String> {
    let guard = state.lock().await;
    let client = guard.client.as_ref().ok_or("Not connected")?;
    let providers = client.get_providers().await?;
    let defaults = client.get_config_providers().await?;
    Ok(RouteConfig {
        settings: guard.settings.routing.clone(),
        policy: RoutePolicyMeta {
            buckets: vec![
                RouteBucketMeta {
                    name: "fast",
                    description: "Short, simple prompts.",
                },
                RouteBucketMeta {
                    name: "balanced",
                    description: "Moderate complexity or implementation/debug tasks.",
                },
                RouteBucketMeta {
                    name: "deep",
                    description: "Large refactor or architectural questions.",
                },
                RouteBucketMeta {
                    name: "long_context",
                    description: "Very long prompts.",
                },
            ],
            selection_rule: "Score every eligible connected provider/model using historical success rate, cost, latency, difficulty fit, and quota pressure. Pick the highest score.".into(),
        },
        providers,
        defaults,
        analytics: guard.analytics.clone(),
        pinned_providers: vec!["openai".into(), "opencode".into()],
    })
}

#[tauri::command]
pub async fn set_provider_quota(
    provider_id: String,
    requests_cap: Option<u64>,
    budget_cap: Option<f64>,
    tokens_cap: Option<u64>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let analytics_store = {
        let guard = state.lock().await;
        guard.analytics_store.clone()
    };
    let mut guard = state.lock().await;
    guard
        .analytics
        .set_quota(&provider_id, requests_cap, budget_cap, tokens_cap);
    analytics_store.save(&guard.analytics).await
}
