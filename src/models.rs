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

#[derive(Debug, Clone)]
pub enum ABRType {
    Fixed,
    ThroughputBased { window_size: usize },
}

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub abr_type: ABRType,
    pub buffer_size_max_secs: f32,
    pub segment_duration_secs: f32,
    pub stall_threshold_secs: f32,
}