mod abr;
mod playback;
mod network;
mod metrics;
mod models;

use crate::playback::engine::run_simulation;

#[no_mangle]
pub extern "C" fn simulate_session() {
    let metrics = run_simulation();

    for m in metrics {
        println!(
            "Time: {}s, Bitrate: {} kbps, Buffer: {:.2} s, Stalled: {}, Switch: {}",
            m.timestamp, m.bitrate_kbps, m.buffer_level_secs, m.stalled, m.switch
        );
    }
}