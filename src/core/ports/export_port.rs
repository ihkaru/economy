use crate::core::domain::agent::human::Human;
use crate::core::domain::environment::climate::ClimateState;
use crate::core::domain::environment::resource::ResourceNode;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::domain::statistic::table::StatisticalTableRelease;
use crate::core::domain::time::RunId;

#[derive(Debug, Clone)]
pub struct ExportSummary {
    pub ledger_records_written: usize,
    pub agents_recorded: usize,
    pub resource_nodes_recorded: usize,
    pub statistics_recorded: usize,
    pub tables_recorded: usize,
    pub output_directory: String,
}

pub trait ExportPort: Send + Sync {
    fn export_all(
        &mut self,
        run_id: &RunId,
        ledger_entries: &[LedgerEntry],
        agents: &[Human],
        climate: &ClimateState,
        nodes: &[ResourceNode],
        statistics: &[StatisticReleaseRecord],
        tables: &[StatisticalTableRelease],
    ) -> Result<ExportSummary, String>;
}
