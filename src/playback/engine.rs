use crate::abr::{fixed, ABRStrategy};
use crate::network::traces::generate_fake_bandwidth_trace;
use crate::models::{Segment, SessionMetrics};

pub fn run_simulation(abr: &mut dyn ABRStrategy) -> Vec<SessionMetrics> {
    let trace = generate_fake_bandwidth_trace();
    let mut metrics: Vec<SessionMetrics> = Vec::new();

    let mut buffer_level = 0.0;
    let mut stalled = false;
    let segment_duration = 1.0; // 1 second
    let mut last_bitrate = 0;

    for (timestamp, bandwidth) in trace.iter().enumerate() {
        // let bitrate = fixed::select_bitrate(*bandwidth, buffer_level);
        let bitrate = abr.select_bitrate(*bandwidth, buffer_level);

        let download_time = (bitrate as f32) / bandwidth;
        let switch = bitrate != last_bitrate;

        if buffer_level < 0.5 {
            stalled = true;
            buffer_level += segment_duration - 1.0; // penalize for stall
        } else {
            buffer_level += segment_duration - download_time;
            stalled = false;
        }

        if buffer_level > 10.0 {
            buffer_level = 10.0; // cap buffer level
        }

        metrics.push(SessionMetrics {
            timestamp: timestamp as u32,
            bitrate_kbps: bitrate,
            buffer_level_secs: buffer_level,
            stalled,
            switch,
        });

        last_bitrate = bitrate;
    }

    metrics
}

#[cfg(test)]
mod tests {
    use crate::abr::fixed::FixedBitrate;
    use super::*;

    #[test]
    fn test_run_simulation_generates_metrics() {
        let mut abr = FixedBitrate { bitrate_kbps: 800 };
        let metrics = run_simulation(&mut abr);
        assert_eq!(metrics.len(), 10);
        assert!(metrics.iter().any(|m| m.stalled == true || m.switch == true));
    }
}