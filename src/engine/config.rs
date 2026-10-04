use serde::{Deserialize, Serialize};
use crate::core::domain::time::{RunId, TickDuration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub run_id: RunId,
    pub seed: u64,
    pub tick_duration: TickDuration,
    pub total_ticks: u64,
    /// Target Ticks Per Second (TPS). None = run at maximum CPU capability (uncapped headless).
    pub ticks_per_second: Option<u64>,
    pub output_dir: String,
}

impl SimulationConfig {
    pub fn new(
        run_id: RunId,
        seed: u64,
        tick_duration: TickDuration,
        total_ticks: u64,
        ticks_per_second: Option<u64>,
        output_dir: impl Into<String>,
    ) -> Self {
        Self {
            run_id,
            seed,
            tick_duration,
            total_ticks,
            ticks_per_second,
            output_dir: output_dir.into(),
        }
    }
}
