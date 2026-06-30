#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    pub request_count: u64,
    pub error_count: u64,
    pub total_latency_ms: u64,
    pub recent_requests: VecDeque<RequestRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestRecord {
    pub provider_id: String,
    pub model_id: String,
    pub latency_ms: u64,
    pub success: bool,
}

impl TelemetryMetrics {
    pub fn record_request(&mut self, provider_id: String, model_id: String, latency_ms: u64, success: bool) {
        self.request_count += 1;
        if !success {
            self.error_count += 1;
        }
        self.total_latency_ms += latency_ms;
        self.recent_requests.push_back(RequestRecord {
            provider_id,
            model_id,
            latency_ms,
            success,
        });
        if self.recent_requests.len() > 50 {
            self.recent_requests.pop_front();
        }
    }

    pub fn avg_latency_ms(&self) -> f64 {
        if self.request_count == 0 {
            0.0
        } else {
            self.total_latency_ms as f64 / self.request_count as f64
        }
    }
}
