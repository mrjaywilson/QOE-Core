pub mod fixed;
pub mod buffer_based;
pub mod throughput_based;

use crate::abr::fixed::FixedBitrate;
use crate::abr::throughput_based::ThroughputBased;
use crate::abr::buffer_based::BufferBased;
use crate::models::ABRType;

pub trait ABRStrategy {
    fn select_bitrate(
        &mut self,
        bandwidth_kbps: f32,
        buffer_level: f32) -> u32;
}

pub fn create_strategy(abr_type: ABRType) -> Box<dyn ABRStrategy> {
    match abr_type {
        ABRType::Fixed => Box::new(FixedBitrate { bitrate_kbps: 1000 }),
        ABRType::BufferBased => Box::new(BufferBased),
        ABRType::ThroughputBased { window_size } => Box::new(ThroughputBased::new(window_size)),
    }
}