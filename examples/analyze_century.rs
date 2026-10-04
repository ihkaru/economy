use std::collections::BTreeMap;
use std::fs::File;
use arrow::array::{Float64Array, StringArray, UInt64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use economy::core::domain::statistic::renderer::{AsciiTableRenderer, TableRenderer};
use economy::core::domain::statistic::table::StatisticalTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let run_dir = if args.len() > 1 {
        std::path::PathBuf::from(&args[1])
    } else {
        let mut matching: Vec<std::path::PathBuf> = std::fs::read_dir("output")?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir() && p.file_name().and_then(|n| n.to_str()).map(|s| s.starts_with("run_id=century_seed42")).unwrap_or(false))
            .collect();
        matching.sort();
        matching.pop().unwrap_or_else(|| std::path::PathBuf::from("output"))
    };

    if !run_dir.exists() {
        eprintln!("Run directory not found: {}", run_dir.display());
        return Ok(());
    }

    println!("======================================================================");
    println!("🏛️  100-YEAR SIMULATION EMPIRICAL AUDIT (36,500 TICKS)");
    println!("Target Directory: {}", run_dir.display());
    println!("======================================================================\n");

    // 1. Analyze Agents
    let agents_file = File::open(run_dir.join("agents.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(agents_file)?;
    let mut reader = builder.build()?;

    let mut total_agents = 0;
    let mut alive_count = 0;
    let mut deceased_count = 0;
    let mut max_age_years: f64 = 0.0;
    let mut ages = Vec::new();
    let mut alive_ages = Vec::new();

    while let Some(batch) = reader.next() {
        let batch = batch?;
        total_agents += batch.num_rows();

        let age_ticks_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
        let is_alive_col = batch.column(4).as_any().downcast_ref::<arrow::array::BooleanArray>().unwrap();

        for i in 0..batch.num_rows() {
            let age_years = age_ticks_col.value(i) as f64 / 365.0;
            ages.push(age_years);
            if age_years > max_age_years {
                max_age_years = age_years;
            }

            let is_alive = is_alive_col.value(i);
            if is_alive {
                alive_count += 1;
                alive_ages.push(age_years);
            } else {
                deceased_count += 1;
            }
        }
    }

    println!("👥 DEMOGRAPHIC LIFECYCLE AUDIT (100 YEARS):");
    println!("  Total Agents Spawned/Born : {}", total_agents);
    println!("  Living Agents at Year 100  : {}", alive_count);
    println!("  Deceased Cumulative        : {}", deceased_count);
    println!("  Max Age Reached            : {:.1} years", max_age_years);
    if !alive_ages.is_empty() {
        let avg_alive_age: f64 = alive_ages.iter().sum::<f64>() / alive_ages.len() as f64;
        println!("  Average Age of Survivors  : {:.1} years", avg_alive_age);
    }

    // 2. Analyze Ledger Transactions
    let ledger_file = File::open(run_dir.join("ledger.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(ledger_file)?;
    let mut reader = builder.build()?;

    let mut total_transactions = 0;
    let mut trx_breakdown = BTreeMap::new();
    let mut tools_crafted = BTreeMap::new();
    let mut discoveries = BTreeMap::new();
    let mut total_harvests = 0;
    let mut tool_multiplied_harvests = 0;
    let mut barter_informed_count = 0;
    let mut barter_uninformed_count = 0;

    while let Some(batch) = reader.next() {
        let batch = batch?;
        total_transactions += batch.num_rows();

        let meta_col = batch.column(11).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..batch.num_rows() {
            let meta_str = meta_col.value(i);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(meta_str) {
                if let Some(t) = val.get("transaction_type").and_then(|v| v.as_str()) {
                    *trx_breakdown.entry(t.to_string()).or_insert(0) += 1;

                    if t == "capital_tool_production" {
                        if let Some(tool) = val.get("tool_crafted").and_then(|v| v.as_str()) {
                            *tools_crafted.entry(tool.to_string()).or_insert(0) += 1;
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
                    } else if t == "bilateral_barter" {
                        let a_inf = val.get("agent_a_informed").and_then(|v| v.as_bool()).unwrap_or(false);
                        let b_inf = val.get("agent_b_informed").and_then(|v| v.as_bool()).unwrap_or(false);
                        if a_inf || b_inf {
                            barter_informed_count += 1;
                        } else {
                            barter_uninformed_count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\n📜 ULTIMATE LEDGER ECONOMIC ACTIVITY (100 YEARS):");
    println!("  Total Transactions Recorded: {}", total_transactions);
    println!("  Transaction Types:");
    for (t, count) in &trx_breakdown {
        println!("    - {:<30} : {} ({:.1}%)", t, count, (*count as f64 / total_transactions as f64) * 100.0);
    }
    println!("  Capital Goods Fabricated:");
    for (tool, count) in &tools_crafted {
        println!("    - {:<20} : {} units", tool, count);
    }
    println!("  Eureka Breakthroughs Discovered:");
    for (k, count) in &discoveries {
        println!("    - {:<30} : {} breakthroughs", k, count);
    }
    println!("  Natural Resource Harvesting:");
    println!("    - Total Harvest Events     : {}", total_harvests);
    println!("    - Tool-Enhanced (3.0x Yield): {} ({:.1}%)", tool_multiplied_harvests, (tool_multiplied_harvests as f64 / total_harvests.max(1) as f64) * 100.0);
    println!("  Hayekian Market Intelligence Barter:");
    println!("    - Informed Market Arbitrage : {} trades", barter_informed_count);
    println!("    - Uninformed Blind Barter   : {} trades", barter_uninformed_count);

    // 3. Environment Nodes Status
    let env_file = File::open(run_dir.join("environment.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(env_file)?;
    let mut reader = builder.build()?;

    println!("\n🌲 ECOLOGICAL CARRYING CAPACITY STATUS AT YEAR 100:");
    while let Some(batch) = reader.next() {
        let batch = batch?;
        let name_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let curr_col = batch.column(5).as_any().downcast_ref::<arrow::array::UInt32Array>().unwrap();
        let max_col = batch.column(6).as_any().downcast_ref::<arrow::array::UInt32Array>().unwrap();
        let maturity_col = batch.column(9).as_any().downcast_ref::<Float64Array>().unwrap();

        for i in 0..batch.num_rows() {
            println!("  - {:<28} : Stock {}/{} | Maturity {:.1}%",
                name_col.value(i), curr_col.value(i), max_col.value(i), maturity_col.value(i) * 100.0);
        }
    }

    // 4. Render Latest Tables
    let tables_path = run_dir.join("tables.parquet");
    if tables_path.exists() {
        let tables_file = File::open(tables_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(tables_file)?;
        let mut reader = builder.build()?;
        let renderer = AsciiTableRenderer::new();

        println!("\n======================================================================");
        println!("📊 FINAL STRUCTURED STATISTICAL RELEASES (YEAR 100 / TICK 36,500)");
        println!("======================================================================\n");

        while let Some(batch) = reader.next() {
            let batch = batch?;
            let tick_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
            let table_json_col = batch.column(5).as_any().downcast_ref::<StringArray>().unwrap();

            let total_rows = batch.num_rows();
            let start_idx = total_rows.saturating_sub(3);

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
