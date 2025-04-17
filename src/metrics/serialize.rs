use crate::models::SessionMetrics;
use serde_json;

pub fn metrics_to_json(metrics: &[SessionMetrics]) -> String {
    serde_json::to_string_pretty(&metrics).unwrap_or_else(|_| "[]".to_string())
}