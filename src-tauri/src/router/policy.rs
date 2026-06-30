use serde_json::Value;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct RouteDecision {
    pub provider_id: String,
    pub model_id: String,
    pub difficulty: String,
    pub reason: String,
}

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

pub fn choose_route(text: &str, providers: &Value, config: &Value) -> Option<RouteDecision> {
    let difficulty = classify_difficulty(text);

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

    // Prefer the first connected provider that has a default model.
    for provider_value in connected.iter() {
        let provider_id = provider_value.as_str().map(String::from)?;
        if let Some(model_id) = defaults.get(&provider_id).and_then(|m| m.as_str()) {
            return Some(RouteDecision {
                provider_id,
                model_id: model_id.to_string(),
                difficulty: difficulty.to_string(),
                reason: format!("Connected provider for {} task", difficulty),
            });
        }
    }

    // Fallback: pick the first default mapping available.
    defaults.iter().next().map(|(provider_id, model)| {
        let model_id = model.as_str().unwrap_or("default").to_string();
        RouteDecision {
            provider_id: provider_id.clone(),
            model_id,
            difficulty: difficulty.to_string(),
            reason: format!("Fallback default for {} task", difficulty),
        }
    })
}
