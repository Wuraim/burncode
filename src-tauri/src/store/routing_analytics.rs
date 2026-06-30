use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingAnalytics {
    #[serde(default)]
    pub models: HashMap<String, ModelUsageStats>,
    #[serde(default)]
    pub provider_quotas: HashMap<String, ProviderQuotaConfig>,
    #[serde(default)]
    pub total_routes: u64,
    #[serde(default)]
    pub total_failures: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelUsageStats {
    pub requests: u64,
    pub successes: u64,
    pub failures: u64,
    pub total_cost: f64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_reasoning_tokens: u64,
    pub total_latency_ms: u64,
    #[serde(default)]
    pub by_difficulty: HashMap<String, u64>,
    pub last_used_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderQuotaConfig {
    #[serde(default)]
    pub requests_cap: Option<u64>,
    #[serde(default)]
    pub budget_cap: Option<f64>,
    #[serde(default)]
    pub tokens_cap: Option<u64>,
    #[serde(default)]
    pub source: QuotaSource,
    #[serde(default)]
    pub used_requests: u64,
    #[serde(default)]
    pub used_budget: f64,
    #[serde(default)]
    pub used_tokens: u64,
    #[serde(default)]
    pub period_type: Option<QuotaPeriod>,
    #[serde(default)]
    pub period_start_ms: Option<u64>,
    #[serde(default)]
    pub period_cap_requests: Option<u64>,
    #[serde(default)]
    pub period_cap_budget: Option<f64>,
    #[serde(default)]
    pub period_cap_tokens: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaSource {
    #[default]
    Estimated,
    Manual,
    Live,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaPeriod {
    Daily,
    Monthly,
}

impl RoutingAnalytics {
    pub fn key(provider_id: &str, model_id: &str) -> String {
        format!("{}/{}", provider_id, model_id)
    }

    pub fn get_model(&self, provider_id: &str, model_id: &str) -> ModelUsageStats {
        self.models
            .get(&Self::key(provider_id, model_id))
            .cloned()
            .unwrap_or_default()
    }

    pub fn check_period_reset(&mut self) {
        let now = now_ms();
        for quota in self.provider_quotas.values_mut() {
            let Some(ref ptype) = quota.period_type else { continue };
            let Some(start) = quota.period_start_ms else { continue };
            let period_ms: u64 = match ptype {
                QuotaPeriod::Daily => 24 * 3600 * 1000,
                QuotaPeriod::Monthly => 30 * 24 * 3600 * 1000,
            };
            if now >= start.saturating_add(period_ms) {
                quota.used_requests = 0;
                quota.used_budget = 0.0;
                quota.used_tokens = 0;
                quota.period_start_ms = Some(now);
                // Reset period caps to the current manual caps
                quota.period_cap_requests = quota.requests_cap;
                quota.period_cap_budget = quota.budget_cap;
                quota.period_cap_tokens = quota.tokens_cap;
                quota.source = QuotaSource::Manual;
            }
        }
    }

    pub fn record_outcome(
        &mut self,
        provider_id: &str,
        model_id: &str,
        success: bool,
        cost: f64,
        input_tokens: u64,
        output_tokens: u64,
        reasoning_tokens: u64,
        latency_ms: u64,
        difficulty: &str,
    ) {
        self.check_period_reset();
        let key = Self::key(provider_id, model_id);
        let stats = self.models.entry(key).or_default();
        stats.requests += 1;
        if success {
            stats.successes += 1;
        } else {
            stats.failures += 1;
        }
        stats.total_cost += cost;
        stats.total_input_tokens += input_tokens;
        stats.total_output_tokens += output_tokens;
        stats.total_reasoning_tokens += reasoning_tokens;
        stats.total_latency_ms += latency_ms;
        *stats.by_difficulty.entry(difficulty.to_string()).or_default() += 1;
        stats.last_used_at_ms = Some(now_ms());

        self.total_routes += 1;
        if !success {
            self.total_failures += 1;
        }

        let quota = self.provider_quotas.entry(provider_id.to_string()).or_default();
        quota.used_requests += 1;
        quota.used_budget += cost;
        quota.used_tokens += input_tokens + output_tokens + reasoning_tokens;
    }

    pub fn set_quota(
        &mut self,
        provider_id: &str,
        requests_cap: Option<u64>,
        budget_cap: Option<f64>,
        tokens_cap: Option<u64>,
        period_type: Option<QuotaPeriod>,
    ) {
        let quota = self.provider_quotas.entry(provider_id.to_string()).or_default();
        quota.requests_cap = requests_cap.or(quota.requests_cap);
        quota.budget_cap = budget_cap.or(quota.budget_cap);
        quota.tokens_cap = tokens_cap.or(quota.tokens_cap);
        quota.source = QuotaSource::Manual;
        if let Some(pt) = period_type {
            quota.period_type = Some(pt);
            quota.period_start_ms = Some(now_ms());
            quota.period_cap_requests = quota.requests_cap;
            quota.period_cap_budget = quota.budget_cap;
            quota.period_cap_tokens = quota.tokens_cap;
        }
    }
}

pub fn analytics_path(app_config_dir: PathBuf) -> PathBuf {
    app_config_dir.join("routing-analytics.json")
}

#[derive(Clone)]
pub struct RoutingAnalyticsStore {
    pub config_dir: PathBuf,
}

impl RoutingAnalyticsStore {
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub fn path(&self) -> PathBuf {
        analytics_path(self.config_dir.clone())
    }

    pub async fn load(&self) -> Result<RoutingAnalytics, String> {
        let path = self.path();
        if !path.exists() {
            return Ok(RoutingAnalytics::default());
        }
        let bytes = tokio::fs::read(&path).await.map_err(|e| e.to_string())?;
        let analytics = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        Ok(analytics)
    }

    pub async fn save(&self, analytics: &RoutingAnalytics) -> Result<(), String> {
        tokio::fs::create_dir_all(&self.config_dir)
            .await
            .map_err(|e| e.to_string())?;
        let bytes = serde_json::to_vec_pretty(analytics).map_err(|e| e.to_string())?;
        tokio::fs::write(self.path(), bytes)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
