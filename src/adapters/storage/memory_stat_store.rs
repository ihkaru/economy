use std::collections::HashMap;

use crate::core::domain::agent::human::Human;
use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::domain::statistic::table::{StatisticalTableDefinition, StatisticalTableRelease};
use crate::core::ports::statistic_store::StatisticStorePort;

#[derive(Default)]
pub struct MemoryStatisticStore {
    records: Vec<StatisticReleaseRecord>,
    table_releases: Vec<StatisticalTableRelease>,
    latest_records: HashMap<String, usize>,
    latest_tables: HashMap<String, usize>,
}

impl MemoryStatisticStore {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            table_releases: Vec::new(),
            latest_records: HashMap::new(),
            latest_tables: HashMap::new(),
        }
    }
}

impl StatisticStorePort for MemoryStatisticStore {
    fn record_release(&mut self, record: StatisticReleaseRecord) -> Result<(), String> {
        let idx = self.records.len();
        self.latest_records.insert(record.statistic_id.clone(), idx);
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
        self.latest_records.get(statistic_id).and_then(|&idx| self.records.get(idx))
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
        let idx = self.table_releases.len();
        self.latest_tables.insert(release.table_id.clone(), idx);
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
        self.latest_tables.get(table_id).and_then(|&idx| self.table_releases.get(idx))
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
