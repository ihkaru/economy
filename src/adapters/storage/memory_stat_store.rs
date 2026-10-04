use crate::core::domain::agent::human::Human;
use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::ports::statistic_store::StatisticStorePort;

#[derive(Default)]
pub struct MemoryStatisticStore {
    records: Vec<StatisticReleaseRecord>,
}

impl MemoryStatisticStore {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
}

impl StatisticStorePort for MemoryStatisticStore {
    fn record_release(&mut self, record: StatisticReleaseRecord) -> Result<(), String> {
        self.records.push(record);
        Ok(())
    }

    fn all_releases(&self) -> &[StatisticReleaseRecord] {
        &self.records
    }

    fn total_releases(&self) -> usize {
        self.records.len()
    }

    fn get_latest(&self, statistic_id: &str) -> Option<&StatisticReleaseRecord> {
        self.records.iter().rev().find(|r| r.statistic_id == statistic_id)
    }

    fn get_accessible_for_agent<'a>(
        &'a self,
        agent: &Human,
        definitions: &[StatisticDefinition],
    ) -> Vec<&'a StatisticReleaseRecord> {
        self.records
            .iter()
            .filter(|record| {
                if let Some(def) = definitions.iter().find(|d| d.id == record.statistic_id) {
                    def.access.is_eligible(agent)
                } else {
                    false
                }
            })
            .collect()
    }
}
