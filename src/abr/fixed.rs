use crate::abr::ABRStrategy;

pub struct FixedBitrate {
    pub bitrate_kbps: u32,
}

impl ABRStrategy for FixedBitrate {
    fn select_bitrate(&mut self, _bandwidth_kbps: f32, _buffer_level: f32) -> u32 {
        self.bitrate_kbps
    }
}