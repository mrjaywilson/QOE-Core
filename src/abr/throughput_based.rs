use crate::abr::{create_strategy, ABRStrategy};
use crate::playback::engine::run_simulation;

pub struct ThroughputBased {
    pub window_size: usize,
    pub history: Vec<f32>,
}

impl ThroughputBased {
    pub fn new(window_size: usize) -> Self {
        ThroughputBased {
            window_size,
            history: vec![],
        }
    }
}

impl ABRStrategy for ThroughputBased {
    fn select_bitrate(&mut self, bandwidth_kbps: f32, _buffer_level: f32) -> u32 {
        self.history.push(bandwidth_kbps);
        if self.history.len() > self.window_size {
            self.history.remove(0);
        }

        let avg_bw = self.history.iter().copied().sum::<f32>() / self.history.len() as f32;

        // Simulated tiers (can be improved later)
        if avg_bw >= 1800.0 {
            2000
        } else if avg_bw >= 1200.0 {
            1500
        } else if avg_bw >= 800.0 {
            1000
        } else {
            500
        }
    }
}

#[test]
fn test_throughput_based_strategy() {
    use crate::abr::{ABRType, ABRStrategy};

    let mut strategy = create_strategy(ABRType::ThroughputBased { window_size: 3 });
    let metrics = run_simulation(&mut *strategy);

    assert_eq!(metrics.len(), 10);
    assert!(metrics.iter().any(|m| m.bitrate_kbps < 2000));
}