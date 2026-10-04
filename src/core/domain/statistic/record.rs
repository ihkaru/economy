use serde::{Deserialize, Serialize};
use crate::core::domain::time::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticReleaseRecord {
    pub release_id: u64,
    pub statistic_id: String,
    pub name: String,
    pub release_tick: Tick,
    pub schedule_desc: String,
    pub primary_value: Option<f64>,
    /// Full multidimensional statistical payload in flexible JSON
    pub payload: serde_json::Value,
    /// Encoded data openness / access requirement rules at time of release
    pub access_requirement: serde_json::Value,
}
