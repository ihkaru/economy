use std::fs::{self, File};
use std::path::{Path, PathBuf};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::BTreeMap;
use arrow::array::{StringArray, UInt64Array, Float64Array};
use economy::core::domain::statistic::renderer::{AsciiTableRenderer, TableRenderer};
use economy::core::domain::statistic::table::StatisticalTable;

fn find_latest_run_dir(base: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut dirs = Vec::new();
    if base.exists() {
        for entry in fs::read_dir(base)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                dirs.push(entry.path());
            }
        }
    }
    dirs.sort();
    dirs.pop().ok_or_else(|| "No run directories found in output/".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("output");
    let run_dir = find_latest_run_dir(base_dir)?;

    println!("=======================================================");
    println!("🔍 ANALYZING PERSISTED PARQUET DATA");
    println!("Target Directory: {}", run_dir.display());
    println!("=======================================================");

    // 1. Analyze Ledger
    let ledger_file = File::open(run_dir.join("ledger.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(ledger_file)?;
    let mut reader = builder.build()?;

    let mut trx_types = BTreeMap::new();
    let mut capital_tools = BTreeMap::new();
    let mut discoveries = BTreeMap::new();
    let mut total_harvests = 0;
    let mut tool_multiplied_harvests = 0;

    while let Some(batch) = reader.next() {
        let batch = batch?;
        let meta_col = batch.column(11).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..batch.num_rows() {
            let meta_str = meta_col.value(i);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(meta_str) {
                if let Some(t) = val.get("transaction_type").and_then(|v| v.as_str()) {
                    *trx_types.entry(t.to_string()).or_insert(0) += 1;
                    if t == "capital_tool_production" {
                        if let Some(tool) = val.get("tool_crafted").and_then(|v| v.as_str()) {
                            *capital_tools.entry(tool.to_string()).or_insert(0) += 1;
                        }
                    } else if t == "scientific_discovery" {
                        if let Some(k) = val.get("knowledge_name").and_then(|v| v.as_str()) {
                            *discoveries.entry(k.to_string()).or_insert(0) += 1;
                        }
                    } else if t == "natural_resource_harvest" {
                        total_harvests += 1;
                        if let Some(mult) = val.get("tool_multiplier").and_then(|v| v.as_f64()) {
                            if mult > 1.0 {
                                tool_multiplied_harvests += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n--- Ultimate Ledger Transaction Breakdown ---");
    for (k, v) in &trx_types {
        println!("  • {:<30} : {:>5} records", k, v);
    }
    println!("\n--- Emergent Capital Goods Produced ---");
    for (k, v) in &capital_tools {
        println!("  • {:<30} : {:>5} units", k, v);
    }
    println!("\n--- Emergent Scientific Breakthroughs (Eureka) ---");
    for (k, v) in &discoveries {
        println!("  • {:<30} : {:>5} times", k, v);
    }
    println!("\n--- Tool Efficiency Multiplier in Harvests ---");
    println!("  Total Natural Harvests : {}", total_harvests);
    println!("  Boosted 3.0x Harvests  : {} ({:.1}%)",
        tool_multiplied_harvests,
        if total_harvests > 0 { tool_multiplied_harvests as f64 / total_harvests as f64 * 100.0 } else { 0.0 });

    // 2. Analyze Environment Node Status
    let env_file = File::open(run_dir.join("environment.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(env_file)?;
    let mut reader = builder.build()?;
    println!("\n--- Environmental Nodes Status & Carrying Capacity ---");
    while let Some(batch) = reader.next() {
        let batch = batch?;
        let name_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let stock_col = batch.column(5).as_any().downcast_ref::<arrow::array::UInt32Array>().unwrap();
        let max_stock_col = batch.column(6).as_any().downcast_ref::<arrow::array::UInt32Array>().unwrap();
        let maturity_col = batch.column(9).as_any().downcast_ref::<Float64Array>().unwrap();

        for i in 0..batch.num_rows() {
            println!("  Node: {:<28} Stock: {:>5}/{:<5} Maturity: {:>6.2}%",
                name_col.value(i), stock_col.value(i), max_stock_col.value(i), maturity_col.value(i) * 100.0);
        }
    }

    // 3. Analyze Latest Published Statistical Tables from tables.parquet
    let tables_path = run_dir.join("tables.parquet");
    if tables_path.exists() {
        let tables_file = File::open(tables_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(tables_file)?;
        let mut reader = builder.build()?;
        let renderer = AsciiTableRenderer::new();

        println!("\n=======================================================");
        println!("📊 PERSISTED OFFICIAL STATISTICAL TABLES (LATEST)");
        println!("=======================================================\n");

        while let Some(batch) = reader.next() {
            let batch = batch?;
            let tick_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
            let table_json_col = batch.column(5).as_any().downcast_ref::<StringArray>().unwrap();

            // Print the last 2 tables in batch (latest releases)
            let total_rows = batch.num_rows();
            let start_idx = total_rows.saturating_sub(2);

            for i in start_idx..total_rows {
                let json_str = table_json_col.value(i);
                if let Ok(table) = serde_json::from_str::<StatisticalTable>(json_str) {
                    println!("--- Release at Tick {} ---", tick_col.value(i));
                    println!("{}", renderer.render(&table));
                    println!();
                }
            }
        }
    }

    Ok(())
}
