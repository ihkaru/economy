use std::fs::{self, File};
use std::path::Path;

use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use crate::adapters::persistence::parquet_batch::ParquetBatchBuilder;
use crate::core::domain::agent::human::Human;
use crate::core::domain::environment::climate::ClimateState;
use crate::core::domain::environment::resource::ResourceNode;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::domain::statistic::table::StatisticalTableRelease;
use crate::core::domain::time::RunId;
use crate::core::ports::export_port::{ExportPort, ExportSummary};

pub struct ParquetExporter {
    base_output_dir: String,
}

impl ParquetExporter {
    pub fn new(base_output_dir: impl Into<String>) -> Self {
        Self {
            base_output_dir: base_output_dir.into(),
        }
    }

    fn write_record_batch_to_parquet(
        batch: RecordBatch,
        output_file_path: &Path,
    ) -> Result<(), String> {
        let file = File::create(output_file_path)
            .map_err(|e| format!("Failed to create Parquet file {}: {}", output_file_path.display(), e))?;

        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))
            .map_err(|e| format!("Failed to initialize ArrowWriter: {}", e))?;

        writer
            .write(&batch)
            .map_err(|e| format!("Failed to write RecordBatch to Parquet: {}", e))?;

        writer
            .close()
            .map_err(|e| format!("Failed to close ArrowWriter: {}", e))?;

        Ok(())
    }
}

impl ExportPort for ParquetExporter {
    fn export_all(
        &mut self,
        run_id: &RunId,
        ledger_entries: &[LedgerEntry],
        agents: &[Human],
        _climate: &ClimateState,
        nodes: &[ResourceNode],
        statistics: &[StatisticReleaseRecord],
        tables: &[StatisticalTableRelease],
    ) -> Result<ExportSummary, String> {
        let target_dir = Path::new(&self.base_output_dir).join(format!("run_id={}", run_id.as_str()));
        fs::create_dir_all(&target_dir)
            .map_err(|e| format!("Failed to create directory {}: {}", target_dir.display(), e))?;

        // 1. Export Ledger
        let ledger_batch = ParquetBatchBuilder::create_ledger_batch(ledger_entries)?;
        let ledger_file = target_dir.join("ledger.parquet");
        Self::write_record_batch_to_parquet(ledger_batch, &ledger_file)?;

        // 2. Export Agents
        let agents_batch = ParquetBatchBuilder::create_agents_batch(agents)?;
        let agents_file = target_dir.join("agents.parquet");
        Self::write_record_batch_to_parquet(agents_batch, &agents_file)?;

        // 3. Export Environment Nodes
        let env_batch = ParquetBatchBuilder::create_environment_batch(nodes)?;
        let env_file = target_dir.join("environment.parquet");
        Self::write_record_batch_to_parquet(env_batch, &env_file)?;

        // 4. Export Statistical Releases
        let stats_batch = ParquetBatchBuilder::create_statistics_batch(statistics)?;
        let stats_file = target_dir.join("statistics.parquet");
        Self::write_record_batch_to_parquet(stats_batch, &stats_file)?;

        // 5. Export Tabular Statistical Releases (if any)
        if !tables.is_empty() {
            let tables_batch = ParquetBatchBuilder::create_tables_batch(tables)?;
            let tables_file = target_dir.join("tables.parquet");
            Self::write_record_batch_to_parquet(tables_batch, &tables_file)?;
        }

        Ok(ExportSummary {
            ledger_records_written: ledger_entries.len(),
            agents_recorded: agents.len(),
            resource_nodes_recorded: nodes.len(),
            statistics_recorded: statistics.len(),
            tables_recorded: tables.len(),
            output_directory: target_dir.display().to_string(),
        })
    }
}
