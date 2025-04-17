mod abr;
mod playback;
mod network;
mod metrics;
mod models;
mod build;

use std::ffi::c_float;
use std::thread::scope;
use crate::playback::engine::run_simulation;
use crate::metrics::logger::write_to_csv;
use crate::models::{ABRType, SimulationConfig};
use crate::metrics::qoe::evaluate_qoe;

#[no_mangle]
pub extern "C" fn simulate_session() {

    let config = SimulationConfig {
        segment_duration_secs: 1.0,
        stall_threshold_secs: 0.5,
        buffer_size_max_secs: 10.0,
        abr_type: ABRType::ThroughputBased { window_size: 3 },
    };

    let metrics = run_simulation(&config);
    let _ = write_to_csv(&metrics, "data/session_log.csv");

    for m in metrics {
        println!(
            "Time: {}s, Bitrate: {} kbps, Buffer: {:.2} s, Stalled: {}, Switch: {}",
            m.timestamp, m.bitrate_kbps, m.buffer_level_secs, m.stalled, m.switch
        );
    }
}

#[no_mangle]
pub extern "C" fn simulate_and_get_score() -> c_float {
    let config = SimulationConfig {
        abr_type: ABRType::ThroughputBased { window_size: 3},
        segment_duration_secs: 1.0,
        buffer_size_max_secs: 10.0,
        stall_threshold_secs: 0.5,
    };

    let metrics = run_simulation(&config);
    let score = evaluate_qoe(&metrics);

    score.final_score
}