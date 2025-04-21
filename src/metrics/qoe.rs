use crate::models::{SessionMetrics, QoEScore};

pub fn evaluate_qoe(metrics: &[SessionMetrics]) -> QoEScore {
    let total_time = metrics.len() as f32;
    let total_bitrate: u32 = metrics.iter().map(|m| m.bitrate_kbps).sum();
    let switch_count = metrics.iter().filter(|m| m.switch).count();
    let stall_count = metrics.iter().filter(|m| m.stalled).count();

    let average_bitrate = total_bitrate as f32 / total_time;
    let stall_ratio = stall_count as f32 / total_time;

    let score = (average_bitrate / 2000.0)
        * (1.0 - stall_ratio)
        * (1.0 - (switch_count as f32 / total_time).min(1.0));

    QoEScore {
        average_bitrate,
        stall_ratio,
        stall_count: stall_count as u32,
        switch_count: switch_count as u32,
        final_score: (score * 100.0).round(),
    }
}