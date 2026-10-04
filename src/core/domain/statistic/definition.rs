use std::sync::Arc;
use crate::core::domain::statistic::access::AccessRequirement;
use crate::core::domain::statistic::calculator::StatisticCalculator;
use crate::core::domain::statistic::schedule::ReleaseSchedule;

pub struct StatisticDefinition {
    pub id: String,
    pub name: String,
    pub schedule: ReleaseSchedule,
    pub access: AccessRequirement,
    pub access_json: serde_json::Value,
    pub calculator: Arc<dyn StatisticCalculator>,
}

impl StatisticDefinition {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        schedule: ReleaseSchedule,
        access: AccessRequirement,
        calculator: impl StatisticCalculator + 'static,
    ) -> Self {
        let access_json = serde_json::to_value(&access).unwrap_or_default();
        Self {
            id: id.into(),
            name: name.into(),
            schedule,
            access,
            access_json,
            calculator: Arc::new(calculator),
        }
    }
}
