use crate::abr::ABRStrategy;

pub struct BufferBased;

impl ABRStrategy for BufferBased {
    fn select_bitrate(
        &mut self,
        bandwidth_kbps: f32,
        buffer_level: f32,
    ) -> u32 {
        if buffer_level >= 4.0 {
            1500
        } else if buffer_level >= 2.0 {
            1000
        } else if buffer_level >= 1.0 {
            750
        } else {
            500
        }
    }
}