use crate::core::domain::agent::human::Human;
use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::domain::statistic::table::{StatisticalTableDefinition, StatisticalTableRelease};
use crate::core::ports::statistic_store::StatisticStorePort;

#[derive(Default)]
pub struct MemoryStatisticStore {
    records: Vec<StatisticReleaseRecord>,
    table_releases: Vec<StatisticalTableRelease>,
}

impl MemoryStatisticStore {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            table_releases: Vec::new(),
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

    fn record_table_release(&mut self, release: StatisticalTableRelease) -> Result<(), String> {
        self.table_releases.push(release);
        Ok(())
    }

    fn all_table_releases(&self) -> &[StatisticalTableRelease] {
        &self.table_releases
    }

    fn total_table_releases(&self) -> usize {
        self.table_releases.len()
    }

    fn get_latest_table(&self, table_id: &str) -> Option<&StatisticalTableRelease> {
        self.table_releases.iter().rev().find(|r| r.table_id == table_id)
    }

    fn get_accessible_tables_for_agent<'a>(
        &'a self,
        agent: &Human,
        definitions: &[StatisticalTableDefinition],
    ) -> Vec<&'a StatisticalTableRelease> {
        self.table_releases
            .iter()
            .filter(|release| {
                if let Some(def) = definitions.iter().find(|d| d.id == release.table_id) {
                    def.access.is_eligible(agent)
                } else {
                    false
                }
            })
            .collect()
    }
}
