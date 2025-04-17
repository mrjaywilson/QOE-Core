use crate::abr::{create_strategy, ABRStrategy};
use crate::metrics::qoe::evaluate_qoe;
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
            buffer_level += segment_duration;
        } else {
            stalled = false;
            buffer_level += segment_duration - download_time;

            if buffer_level < 0.0 {
                buffer_level = 0.0;
            }
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
    use crate::metrics::qoe::evaluate_qoe;
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
        let qoe = evaluate_qoe(&metrics);

        println!(
            "QoE Simulation Test Score: {:.1}/100 | Avg Bitrate: {:.1}kbps | Stall Ratio: {:.2} | Switches: {}",
            qoe.final_score, qoe.average_bitrate, qoe.stall_ratio, qoe.switch_count
        );

        assert_eq!(metrics.len(), 10);
        assert!(metrics.iter().any(|m| m.stalled == true || m.switch == true));
    }
}

#[test]
fn test_buffer_based_strategy_outputs_varied_bitrates() {
    use crate::models::{ABRType, SimulationConfig};
    use crate::playback::engine::run_simulation;

    let config = SimulationConfig {
        segment_duration_secs: 1.0,
        stall_threshold_secs: 0.2,
        buffer_size_max_secs: 12.0,
        abr_type: ABRType::BufferBased,
    };

    let metrics = run_simulation(&config);
    let qoe = evaluate_qoe(&metrics);

    println!(
        "QoE Buffer-Based Test Score: {:.1}/100 | Avg Bitrate: {:.1}kbps | Stall Ratio: {:.2} | Switches: {}",
        qoe.final_score, qoe.average_bitrate, qoe.stall_ratio, qoe.switch_count
    );


    let distinct_bitrates: std::collections::HashSet<u32> = metrics.iter().map(|m| m.bitrate_kbps).collect();

    assert!(distinct_bitrates.len() > 1);
}

#[test]
fn test_qoe_scoring_for_throughput_based()
{
    use crate::models::{ABRType, SimulationConfig};
    use crate::metrics::qoe::evaluate_qoe;
    use crate::playback::engine::run_simulation;

    let config = SimulationConfig {
        segment_duration_secs: 1.0,
        stall_threshold_secs: 0.5,
        buffer_size_max_secs: 10.0,
        abr_type: ABRType::ThroughputBased { window_size: 3 },
    };

    let metrics = run_simulation(&config);
    let qoe = evaluate_qoe(&metrics);

    println!(
        "QoE Score: {:.1}/100 | Bitrate: {:.1}kbps | Stalls: {:.2} | Switches: {}",
        qoe.final_score, qoe.average_bitrate, qoe.stall_ratio, qoe.switch_count
    );

    assert!(qoe.final_score > 10.0, "QoE score too low!");
}