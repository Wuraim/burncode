use crate::router::types::{RouteCandidate, RouteDecision};
use crate::store::routing_analytics::{ProviderQuotaConfig, RoutingAnalytics};
use serde_json::Value;

pub fn score_candidates(
    _text: &str,
    providers: &Value,
    config: &Value,
    analytics: &RoutingAnalytics,
    difficulty: &str,
) -> Vec<RouteCandidate> {
    let connected = providers
        .get("connected")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let defaults = providers
        .get("default")
        .or_else(|| config.get("default"))
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    let mut candidates = Vec::new();

    for provider_value in connected.iter() {
        let Some(provider_id) = provider_value.as_str() else { continue };
        let Some(model_id) = defaults.get(provider_id).and_then(|m| m.as_str()) else { continue };

        let score = compute_score(provider_id, model_id, analytics, difficulty);
        candidates.push(RouteCandidate {
            provider_id: provider_id.to_string(),
            model_id: model_id.to_string(),
            score,
            reason: format!("score {:.2}", score),
        });
    }

    // If no connected candidate, use configured defaults as emergency fallback.
    if candidates.is_empty() {
        for (provider_id, model) in defaults.iter() {
            let model_id = model.as_str().unwrap_or("default").to_string();
            let score = compute_score(provider_id, &model_id, analytics, difficulty);
            candidates.push(RouteCandidate {
                provider_id: provider_id.clone(),
                model_id,
                score,
                reason: format!("fallback score {:.2}", score),
            });
        }
    }

    candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    candidates
}

fn compute_score(
    provider_id: &str,
    model_id: &str,
    analytics: &RoutingAnalytics,
    difficulty: &str,
) -> f64 {
    let stats = analytics.get_model(provider_id, model_id);
    let mut score = 100.0;

    // Success rate.
    if stats.requests > 0 {
        let success_rate = stats.successes as f64 / stats.requests as f64;
        score += success_rate * 50.0;
    } else {
        // Unknown models start slightly lower to prefer proven ones.
        score -= 10.0;
    }

    // Cost efficiency.
    if stats.requests > 0 && stats.total_cost > 0.0 {
        let avg_cost = stats.total_cost / stats.requests as f64;
        score -= avg_cost * 20.0;
    }

    // Latency efficiency.
    if stats.requests > 0 && stats.total_latency_ms > 0 {
        let avg_latency = stats.total_latency_ms as f64 / stats.requests as f64;
        score -= avg_latency * 0.01;
    }

    // Difficulty fit.
    if stats.by_difficulty.get(difficulty).copied().unwrap_or(0) > 0 {
        score += 15.0;
    }

    // Quota pressure.
    if let Some(quota) = analytics.provider_quotas.get(provider_id) {
        if is_quota_exhausted(quota) {
            score = 0.0;
        } else if let Some(cap) = quota.requests_cap {
            if cap > 0 {
                let used_ratio = quota.used_requests as f64 / cap as f64;
                score -= used_ratio * 40.0;
            }
        }
    }

    score.max(0.0)
}

fn is_quota_exhausted(quota: &ProviderQuotaConfig) -> bool {
    if let Some(cap) = quota.requests_cap {
        if quota.used_requests >= cap {
            return true;
        }
    }
    if let Some(cap) = quota.budget_cap {
        if quota.used_budget >= cap {
            return true;
        }
    }
    if let Some(cap) = quota.tokens_cap {
        if quota.used_tokens >= cap {
            return true;
        }
    }
    false
}

pub fn build_decision(
    text: &str,
    providers: &Value,
    config: &Value,
    analytics: &RoutingAnalytics,
) -> Option<RouteDecision> {
    use crate::router::policy::classify_difficulty;
    let difficulty = classify_difficulty(text);
    let mut candidates = score_candidates(text, providers, config, analytics, difficulty);
    if candidates.is_empty() {
        return None;
    }
    let chosen = candidates.remove(0);
    let source = if candidates.is_empty() {
        "fallback"
    } else if analytics.get_model(&chosen.provider_id, &chosen.model_id).requests > 0 {
        "learned"
    } else {
        "default"
    };
    Some(RouteDecision {
        provider_id: chosen.provider_id,
        model_id: chosen.model_id,
        difficulty: difficulty.to_string(),
        reason: format!("{} ({:.0})", chosen.reason, chosen.score),
        candidates,
        source: source.to_string(),
    })
}
