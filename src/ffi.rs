use std::ffi::{c_char, c_float, CString};
use std::thread::scope;
use crate::playback::engine::run_simulation;
use crate::metrics::logger::write_to_csv;
use crate::models::{ABRType, SimulationConfig};
use crate::metrics::qoe::evaluate_qoe;

#[repr(C)]
pub struct SimConfig {
    pub abr_type: u32,
    pub abr_window_size: u32,
    pub buffer_size_max: f32,
    pub segment_duration: f32,
    pub stall_threshold: f32,
}

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

#[no_mangle]
pub extern "C" fn simulate_with_config(config: SimConfig) -> c_float {
    let native_config = convert_config(config);
    let metrics = run_simulation(&native_config);
    let score = evaluate_qoe(&metrics);

    score.final_score
}

#[no_mangle]
pub extern "C" fn simulate_and_get_json(config: SimConfig) -> *const c_char {
    let native_config = convert_config(config);
    let metrics = run_simulation(&native_config);

    let json = crate::metrics::serialize::metrics_to_json(&metrics);
    let c_str = CString::new(json).unwrap();

    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn free_simulation_string(s: *mut c_char)
{
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

fn convert_config(c_config: SimConfig) -> SimulationConfig {
    use crate::models::ABRType;

    let abr_type = match c_config.abr_type {
        0 => ABRType::Fixed,
        1 => ABRType::BufferBased,
        2 => ABRType::ThroughputBased { window_size: c_config.abr_window_size as usize },
        _ => ABRType::Fixed,
    };

    SimulationConfig {
        abr_type,
        buffer_size_max_secs: c_config.buffer_size_max,
        segment_duration_secs: c_config.segment_duration,
        stall_threshold_secs: c_config.stall_threshold,
    }
}

#[test]
fn test_simulate_with_config_via_ffi() {
    let config = SimConfig {
        abr_type: 2,
        abr_window_size: 3,
        buffer_size_max: 10.0,
        segment_duration: 1.0,
        stall_threshold: 0.5,
    };

    let score = simulate_with_config(config);
    assert!(score > 0.0);
}