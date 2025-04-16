#[derive(Debug, Clone)]
pub struct Segment {
    pub duration_secs: f32,
    pub bitrate_kbps: u32,
}

#[derive(Debug, Clone)]
pub struct SessionMetrics {
    pub timestamp: u32,
    pub bitrate_kbps: u32,
    pub buffer_level_secs: f32,
    pub stalled:  bool,
    pub switch: bool
}