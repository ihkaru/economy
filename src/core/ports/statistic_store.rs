use crate::core::domain::agent::human::Human;
use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::statistic::record::StatisticReleaseRecord;

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
}
