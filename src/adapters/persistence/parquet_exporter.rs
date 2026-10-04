use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;

use arrow::array::{
    ArrayRef, BooleanArray, Float64Array, StringArray, UInt32Array, UInt64Array,
};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use crate::core::domain::agent::human::{Human, Sex};
use crate::core::domain::agent::traits::HasLifecycle;
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

    fn create_ledger_batch(entries: &[LedgerEntry]) -> Result<RecordBatch, String> {
        let trx_id_arr = Arc::new(UInt64Array::from_iter_values(
            entries.iter().map(|e| e.trx_id),
        )) as ArrayRef;

        let run_id_arr = Arc::new(StringArray::from_iter_values(
            entries.iter().map(|e| e.run_id.as_str()),
        )) as ArrayRef;

        let tick_id_arr = Arc::new(UInt64Array::from_iter_values(
            entries.iter().map(|e| e.tick_id.0),
        )) as ArrayRef;

        let party_a_arr = Arc::new(UInt64Array::from_iter_values(
            entries.iter().map(|e| e.party_a.as_u64()),
        )) as ArrayRef;

        let party_b_arr = Arc::new(UInt64Array::from_iter_values(
            entries.iter().map(|e| e.party_b.as_u64()),
        )) as ArrayRef;

        let primary_item_a_arr = Arc::new(UInt64Array::from_iter(
            entries.iter().map(|e| e.items_from_a.first().map(|i| i.item_id.as_u64())),
        )) as ArrayRef;

        let primary_item_b_arr = Arc::new(UInt64Array::from_iter(
            entries.iter().map(|e| e.items_from_b.first().map(|i| i.item_id.as_u64())),
        )) as ArrayRef;

        let items_a_count_arr = Arc::new(UInt32Array::from_iter_values(
            entries.iter().map(|e| e.items_from_a.len() as u32),
        )) as ArrayRef;

        let items_b_count_arr = Arc::new(UInt32Array::from_iter_values(
            entries.iter().map(|e| e.items_from_b.len() as u32),
        )) as ArrayRef;

        let bundle_a_arr = Arc::new(StringArray::from_iter_values(entries.iter().map(|e| {
            serde_json::to_string(&e.items_from_a).unwrap_or_else(|_| "[]".to_string())
        }))) as ArrayRef;

        let bundle_b_arr = Arc::new(StringArray::from_iter_values(entries.iter().map(|e| {
            serde_json::to_string(&e.items_from_b).unwrap_or_else(|_| "[]".to_string())
        }))) as ArrayRef;

        let metadata_arr = Arc::new(StringArray::from_iter_values(
            entries.iter().map(|e| e.metadata.to_string()),
        )) as ArrayRef;

        let schema = Arc::new(Schema::new(vec![
            Field::new("trx_id", DataType::UInt64, false),
            Field::new("run_id", DataType::Utf8, false),
            Field::new("tick_id", DataType::UInt64, false),
            Field::new("party_a", DataType::UInt64, false),
            Field::new("party_b", DataType::UInt64, false),
            Field::new("primary_item_a", DataType::UInt64, true),
            Field::new("primary_item_b", DataType::UInt64, true),
            Field::new("items_a_count", DataType::UInt32, false),
            Field::new("items_b_count", DataType::UInt32, false),
            Field::new("bundle_a", DataType::Utf8, false),
            Field::new("bundle_b", DataType::Utf8, false),
            Field::new("metadata", DataType::Utf8, false),
        ]));

        RecordBatch::try_new(
            schema,
            vec![
                trx_id_arr,
                run_id_arr,
                tick_id_arr,
                party_a_arr,
                party_b_arr,
                primary_item_a_arr,
                primary_item_b_arr,
                items_a_count_arr,
                items_b_count_arr,
                bundle_a_arr,
                bundle_b_arr,
                metadata_arr,
            ],
        )
        .map_err(|e| format!("Failed to create Ledger RecordBatch: {}", e))
    }

    fn create_agents_batch(agents: &[Human]) -> Result<RecordBatch, String> {
        let agent_id_arr = Arc::new(UInt64Array::from_iter_values(
            agents.iter().map(|a| a.id.as_u64()),
        )) as ArrayRef;

        let sex_arr = Arc::new(StringArray::from_iter_values(agents.iter().map(|a| {
            match a.sex {
                Sex::Male => "Male",
                Sex::Female => "Female",
            }
        }))) as ArrayRef;

        let birth_tick_arr = Arc::new(UInt64Array::from_iter_values(
            agents.iter().map(|a| a.birth_tick.0),
        )) as ArrayRef;

        let age_ticks_arr = Arc::new(UInt64Array::from_iter_values(
            agents.iter().map(|a| a.age_ticks),
        )) as ArrayRef;

        let is_alive_arr = Arc::new(BooleanArray::from_iter(
            agents.iter().map(|a| Some(a.is_alive())),
        )) as ArrayRef;

        let spouse_arr = Arc::new(UInt64Array::from_iter(
            agents.iter().map(|a| a.spouse_id.map(|s| s.as_u64())),
        )) as ArrayRef;

        let mother_arr = Arc::new(UInt64Array::from_iter(
            agents.iter().map(|a| a.mother_id.map(|m| m.as_u64())),
        )) as ArrayRef;

        let father_arr = Arc::new(UInt64Array::from_iter(
            agents.iter().map(|a| a.father_id.map(|f| f.as_u64())),
        )) as ArrayRef;

        let children_count_arr = Arc::new(UInt32Array::from_iter_values(
            agents.iter().map(|a| a.children_ids.len() as u32),
        )) as ArrayRef;

        let total_assets_arr = Arc::new(UInt32Array::from_iter_values(
            agents.iter().map(|a| a.inventory.values().sum::<u32>()),
        )) as ArrayRef;

        let inventory_arr = Arc::new(StringArray::from_iter_values(agents.iter().map(|a| {
            serde_json::to_string(&a.inventory).unwrap_or_else(|_| "{}".to_string())
        }))) as ArrayRef;

        let attributes_arr = Arc::new(StringArray::from_iter_values(
            agents.iter().map(|a| a.attributes.to_string()),
        )) as ArrayRef;

        let pos_x_arr = Arc::new(UInt32Array::from_iter_values(
            agents.iter().map(|a| a.location.x),
        )) as ArrayRef;

        let pos_y_arr = Arc::new(UInt32Array::from_iter_values(
            agents.iter().map(|a| a.location.y),
        )) as ArrayRef;

        let calorie_reserve_arr = Arc::new(Float64Array::from_iter_values(
            agents.iter().map(|a| a.calorie_reserve),
        )) as ArrayRef;

        let days_starving_arr = Arc::new(UInt32Array::from_iter_values(
            agents.iter().map(|a| a.days_starving),
        )) as ArrayRef;

        let has_vessel_arr = Arc::new(BooleanArray::from_iter(
            agents.iter().map(|a| Some(a.inventory.contains_key(&crate::core::domain::item::id::ItemId::RAFT))),
        )) as ArrayRef;

        let schema = Arc::new(Schema::new(vec![
            Field::new("agent_id", DataType::UInt64, false),
            Field::new("sex", DataType::Utf8, false),
            Field::new("birth_tick", DataType::UInt64, false),
            Field::new("age_ticks", DataType::UInt64, false),
            Field::new("is_alive", DataType::Boolean, false),
            Field::new("pos_x", DataType::UInt32, false),
            Field::new("pos_y", DataType::UInt32, false),
            Field::new("calorie_reserve", DataType::Float64, false),
            Field::new("days_starving", DataType::UInt32, false),
            Field::new("has_vessel", DataType::Boolean, false),
            Field::new("spouse_id", DataType::UInt64, true),
            Field::new("mother_id", DataType::UInt64, true),
            Field::new("father_id", DataType::UInt64, true),
            Field::new("children_count", DataType::UInt32, false),
            Field::new("total_assets", DataType::UInt32, false),
            Field::new("inventory_summary", DataType::Utf8, false),
            Field::new("attributes", DataType::Utf8, false),
        ]));

        RecordBatch::try_new(
            schema,
            vec![
                agent_id_arr,
                sex_arr,
                birth_tick_arr,
                age_ticks_arr,
                is_alive_arr,
                pos_x_arr,
                pos_y_arr,
                calorie_reserve_arr,
                days_starving_arr,
                has_vessel_arr,
                spouse_arr,
                mother_arr,
                father_arr,
                children_count_arr,
                total_assets_arr,
                inventory_arr,
                attributes_arr,
            ],
        )
        .map_err(|e| format!("Failed to create Agents RecordBatch: {}", e))
    }

    fn create_environment_batch(
        nodes: &[ResourceNode],
    ) -> Result<RecordBatch, String> {
        let node_id_arr = Arc::new(UInt64Array::from_iter_values(
            nodes.iter().map(|n| n.id),
        )) as ArrayRef;

        let name_arr = Arc::new(StringArray::from_iter_values(
            nodes.iter().map(|n| n.name.as_str()),
        )) as ArrayRef;

        let item_id_arr = Arc::new(UInt64Array::from_iter_values(
            nodes.iter().map(|n| n.item_id.as_u64()),
        )) as ArrayRef;

        let pos_x_arr = Arc::new(UInt32Array::from_iter_values(
            nodes.iter().map(|n| n.location.x),
        )) as ArrayRef;

        let pos_y_arr = Arc::new(UInt32Array::from_iter_values(
            nodes.iter().map(|n| n.location.y),
        )) as ArrayRef;

        let stock_arr = Arc::new(UInt32Array::from_iter_values(
            nodes.iter().map(|n| n.current_stock),
        )) as ArrayRef;

        let max_stock_arr = Arc::new(UInt32Array::from_iter_values(
            nodes.iter().map(|n| n.max_stock),
        )) as ArrayRef;

        let calories_arr = Arc::new(Float64Array::from_iter_values(
            nodes.iter().map(|n| n.calories_per_unit),
        )) as ArrayRef;

        let is_edible_arr = Arc::new(BooleanArray::from_iter(
            nodes.iter().map(|n| Some(n.is_edible)),
        )) as ArrayRef;

        let maturity_arr = Arc::new(Float64Array::from_iter_values(
            nodes.iter().map(|n| n.maturity),
        )) as ArrayRef;

        let pace_arr = Arc::new(StringArray::from_iter_values(
            nodes.iter().map(|n| format!("{:?}", n.pace)),
        )) as ArrayRef;

        let metadata_arr = Arc::new(StringArray::from_iter_values(
            nodes.iter().map(|n| n.metadata.to_string()),
        )) as ArrayRef;

        let schema = Arc::new(Schema::new(vec![
            Field::new("node_id", DataType::UInt64, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("item_id", DataType::UInt64, false),
            Field::new("pos_x", DataType::UInt32, false),
            Field::new("pos_y", DataType::UInt32, false),
            Field::new("current_stock", DataType::UInt32, false),
            Field::new("max_stock", DataType::UInt32, false),
            Field::new("calories_per_unit", DataType::Float64, false),
            Field::new("is_edible", DataType::Boolean, false),
            Field::new("maturity", DataType::Float64, false),
            Field::new("pace", DataType::Utf8, false),
            Field::new("metadata", DataType::Utf8, false),
        ]));

        RecordBatch::try_new(
            schema,
            vec![
                node_id_arr,
                name_arr,
                item_id_arr,
                pos_x_arr,
                pos_y_arr,
                stock_arr,
                max_stock_arr,
                calories_arr,
                is_edible_arr,
                maturity_arr,
                pace_arr,
                metadata_arr,
            ],
        )
        .map_err(|e| format!("Failed to create Environment RecordBatch: {}", e))
    }

    fn create_statistics_batch(
        statistics: &[StatisticReleaseRecord],
    ) -> Result<RecordBatch, String> {
        let release_id_arr = Arc::new(UInt64Array::from_iter_values(
            statistics.iter().map(|s| s.release_id),
        )) as ArrayRef;

        let stat_id_arr = Arc::new(StringArray::from_iter_values(
            statistics.iter().map(|s| s.statistic_id.as_str()),
        )) as ArrayRef;

        let name_arr = Arc::new(StringArray::from_iter_values(
            statistics.iter().map(|s| s.name.as_str()),
        )) as ArrayRef;

        let tick_arr = Arc::new(UInt64Array::from_iter_values(
            statistics.iter().map(|s| s.release_tick.0),
        )) as ArrayRef;

        let schedule_desc_arr = Arc::new(StringArray::from_iter_values(
            statistics.iter().map(|s| s.schedule_desc.as_str()),
        )) as ArrayRef;

        let primary_value_arr = Arc::new(Float64Array::from_iter(
            statistics.iter().map(|s| s.primary_value),
        )) as ArrayRef;

        let payload_arr = Arc::new(StringArray::from_iter_values(
            statistics.iter().map(|s| s.payload.to_string()),
        )) as ArrayRef;

        let access_req_arr = Arc::new(StringArray::from_iter_values(
            statistics.iter().map(|s| s.access_requirement.to_string()),
        )) as ArrayRef;

        let schema = Arc::new(Schema::new(vec![
            Field::new("release_id", DataType::UInt64, false),
            Field::new("statistic_id", DataType::Utf8, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("release_tick", DataType::UInt64, false),
            Field::new("schedule_desc", DataType::Utf8, false),
            Field::new("primary_value", DataType::Float64, true),
            Field::new("payload", DataType::Utf8, false),
            Field::new("access_requirement", DataType::Utf8, false),
        ]));

        RecordBatch::try_new(
            schema,
            vec![
                release_id_arr,
                stat_id_arr,
                name_arr,
                tick_arr,
                schedule_desc_arr,
                primary_value_arr,
                payload_arr,
                access_req_arr,
            ],
        )
        .map_err(|e| format!("Failed to create Statistics RecordBatch: {}", e))
    }

    fn create_tables_batch(
        tables: &[StatisticalTableRelease],
    ) -> Result<RecordBatch, String> {
        let release_id_arr = Arc::new(UInt64Array::from_iter_values(
            tables.iter().map(|s| s.release_id),
        )) as ArrayRef;

        let table_id_arr = Arc::new(StringArray::from_iter_values(
            tables.iter().map(|s| s.table_id.as_str()),
        )) as ArrayRef;

        let title_arr = Arc::new(StringArray::from_iter_values(
            tables.iter().map(|s| s.title.as_str()),
        )) as ArrayRef;

        let tick_arr = Arc::new(UInt64Array::from_iter_values(
            tables.iter().map(|s| s.release_tick.0),
        )) as ArrayRef;

        let schedule_desc_arr = Arc::new(StringArray::from_iter_values(
            tables.iter().map(|s| s.schedule_desc.as_str()),
        )) as ArrayRef;

        let table_json_arr = Arc::new(StringArray::from_iter_values(
            tables.iter().map(|s| serde_json::to_string(&s.table).unwrap_or_default()),
        )) as ArrayRef;

        let access_req_arr = Arc::new(StringArray::from_iter_values(
            tables.iter().map(|s| s.access_requirement.to_string()),
        )) as ArrayRef;

        let schema = Arc::new(Schema::new(vec![
            Field::new("release_id", DataType::UInt64, false),
            Field::new("table_id", DataType::Utf8, false),
            Field::new("title", DataType::Utf8, false),
            Field::new("release_tick", DataType::UInt64, false),
            Field::new("schedule_desc", DataType::Utf8, false),
            Field::new("table_json", DataType::Utf8, false),
            Field::new("access_requirement", DataType::Utf8, false),
        ]));

        RecordBatch::try_new(
            schema,
            vec![
                release_id_arr,
                table_id_arr,
                title_arr,
                tick_arr,
                schedule_desc_arr,
                table_json_arr,
                access_req_arr,
            ],
        )
        .map_err(|e| format!("Failed to create Tables RecordBatch: {}", e))
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
        let ledger_batch = Self::create_ledger_batch(ledger_entries)?;
        let ledger_file = target_dir.join("ledger.parquet");
        Self::write_record_batch_to_parquet(ledger_batch, &ledger_file)?;

        // 2. Export Agents
        let agents_batch = Self::create_agents_batch(agents)?;
        let agents_file = target_dir.join("agents.parquet");
        Self::write_record_batch_to_parquet(agents_batch, &agents_file)?;

        // 3. Export Environment Nodes
        let env_batch = Self::create_environment_batch(nodes)?;
        let env_file = target_dir.join("environment.parquet");
        Self::write_record_batch_to_parquet(env_batch, &env_file)?;

        // 4. Export Statistical Releases
        let stats_batch = Self::create_statistics_batch(statistics)?;
        let stats_file = target_dir.join("statistics.parquet");
        Self::write_record_batch_to_parquet(stats_batch, &stats_file)?;

        // 5. Export Tabular Statistical Releases (if any)
        if !tables.is_empty() {
            let tables_batch = Self::create_tables_batch(tables)?;
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
