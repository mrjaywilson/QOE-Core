use crate::abr::{create_strategy, ABRStrategy};
use crate::network::traces::generate_fake_bandwidth_trace;
use crate::models::{Segment, SessionMetrics, SimulationConfig};

pub fn run_simulation(config: &SimulationConfig) -> Vec<SessionMetrics> {
    let trace = generate_fake_bandwidth_trace();
    let mut metrics: Vec<SessionMetrics> = Vec::new();

    let mut abr = create_strategy(config.abr_type.clone());

    let mut buffer_level = 0.0;
    let mut stalled = false;
    let segment_duration = config.segment_duration_secs;
    let mut last_bitrate = 0;

    for (timestamp, bandwidth) in trace.iter().enumerate() {
        let bitrate = abr.select_bitrate(*bandwidth, buffer_level);

        let download_time = (bitrate as f32) / bandwidth;
        let switch = bitrate != last_bitrate;

        if buffer_level < config.stall_threshold_secs {
            stalled = true;
            buffer_level += segment_duration - 1.0;
        } else {
            buffer_level += segment_duration - download_time;
            stalled = false;
        }

        if buffer_level > config.buffer_size_max_secs {
            buffer_level = config.buffer_size_max_secs;
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
    use crate::models::{ABRType, SimulationConfig};
    use super::*;

    #[test]
    fn test_run_simulation_with_custom_config() {
        let config = SimulationConfig {
            segment_duration_secs: 1.0,
            stall_threshold_secs: 0.5,
            buffer_size_max_secs: 10.0,
            abr_type: ABRType::Fixed,
        };

        let metrics = run_simulation(&config);

        assert_eq!(metrics.len(), 10);
        assert!(metrics.iter().any(|m| m.stalled == true || m.switch == true));
    }
}