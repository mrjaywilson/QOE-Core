mod abr;
mod playback;
mod network;
mod metrics;
mod models;

use crate::playback::engine::run_simulation;
use crate::abr::{ABRType, create_strategy};

#[no_mangle]
pub extern "C" fn simulate_session() {
    let mut strategy = create_strategy(ABRType::ThroughputBased { window_size: 3 });
    let metrics = run_simulation(&mut *strategy);

    for m in metrics {
        println!(
            "Time: {}s, Bitrate: {} kbps, Buffer: {:.2} s, Stalled: {}, Switch: {}",
            m.timestamp, m.bitrate_kbps, m.buffer_level_secs, m.stalled, m.switch
        );
    }
}