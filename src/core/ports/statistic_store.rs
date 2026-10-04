use crate::core::domain::agent::human::Human;
use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::domain::statistic::table::{StatisticalTableDefinition, StatisticalTableRelease};

pub trait StatisticStorePort: Send + Sync {
    fn record_release(&mut self, record: StatisticReleaseRecord) -> Result<(), String>;
    fn all_releases(&self) -> &[StatisticReleaseRecord];
    fn total_releases(&self) -> usize;
    fn get_latest(&self, statistic_id: &str) -> Option<&StatisticReleaseRecord>;
    fn get_accessible_for_agent<'a>(
        &'a self,
        agent: &Human,
        definitions: &[StatisticDefinition],
    ) -> Vec<&'a StatisticReleaseRecord>;

    // Tabular statistical bulletin releases
    fn record_table_release(&mut self, release: StatisticalTableRelease) -> Result<(), String>;
    fn all_table_releases(&self) -> &[StatisticalTableRelease];
    fn total_table_releases(&self) -> usize;
    fn get_latest_table(&self, table_id: &str) -> Option<&StatisticalTableRelease>;
    fn get_accessible_tables_for_agent<'a>(
        &'a self,
        agent: &Human,
        definitions: &[StatisticalTableDefinition],
    ) -> Vec<&'a StatisticalTableRelease>;
}
