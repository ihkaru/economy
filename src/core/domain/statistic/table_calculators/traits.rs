use crate::core::domain::statistic::calculator::StatisticContext;
use crate::core::domain::statistic::table::StatisticalTable;

/// Interface for generating tabular statistical releases
pub trait StatisticalTableCalculator: Send + Sync {
    fn calculate_table(&self, ctx: &StatisticContext) -> StatisticalTable;
}
