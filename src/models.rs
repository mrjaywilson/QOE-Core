use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Segment {
    pub duration_secs: f32,
    pub bitrate_kbps: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionMetrics {
    pub timestamp: u32,
    pub bitrate_kbps: u32,
    pub buffer_level_secs: f32,
    pub stalled:  bool,
    pub switch: bool
}

#[derive(Debug, Clone, Default)]
pub enum ABRType {
    #[default]
    Fixed,
    BufferBased,
    ThroughputBased { window_size: usize },
}

#[derive(Debug, Clone, Default)]
pub struct SimulationConfig {
    pub abr_type: ABRType,
    pub buffer_size_max_secs: f32,
    pub segment_duration_secs: f32,
    pub stall_threshold_secs: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            abr_type: ABRType::Fixed,
            buffer_size_max_secs: 10.0,
            segment_duration_secs: 1.0,
            stall_threshold_secs: 0.5,
        }
    }
}

#[derive(Debug)]
pub struct QoEScore {
    pub average_bitrate: f32,
    pub stall_ratio: f32,
    pub switch_count: u32,
    pub final_score: f32,
}