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
            .filter(|p| {
                p.is_dir()
                    && p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.starts_with("run_id=century_seed42") || s.starts_with("run_id=millennium_seed42"))
                        .unwrap_or(false)
            })
            .collect();
        matching.sort();
        matching.pop().unwrap_or_else(|| std::path::PathBuf::from("output"))
    };

    if !run_dir.exists() {
        eprintln!("Run directory not found: {}", run_dir.display());
        return Ok(());
    }

    println!("======================================================================");
    println!("🏛️  SIMULATION EMPIRICAL AUDIT & HISTORICAL REALITY EVALUATION");
    println!("Target Directory: {}", run_dir.display());
    println!("======================================================================\n");

    // 1. Analyze Tables Parquet to determine Max Tick and Duration
    let mut max_tick: u64 = 0;
    let tables_path = run_dir.join("tables.parquet");
    if tables_path.exists() {
        let tables_file = File::open(&tables_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(tables_file)?;
        let mut reader = builder.build()?;
        while let Some(batch) = reader.next() {
            let batch = batch?;
            let tick_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
            for i in 0..batch.num_rows() {
                let t = tick_col.value(i);
                if t > max_tick {
                    max_tick = t;
                }
            }
        }
    }

    let simulated_years = if max_tick > 0 { max_tick as f64 / 365.0 } else { 100.0 };
    println!("⏱️  Duration Detected: {:.1} years ({} ticks)\n", simulated_years, max_tick);

    // 2. Analyze Agents
    let agents_file = File::open(run_dir.join("agents.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(agents_file)?;
    let mut reader = builder.build()?;

    let mut total_agents = 0;
    let mut alive_count = 0;
    let mut deceased_count = 0;
    let mut max_age_years: f64 = 0.0;
    let mut ages = Vec::new();
    let mut alive_ages = Vec::new();
    let mut max_generation: u64 = 1;
    let mut living_inventory_totals: BTreeMap<String, u64> = BTreeMap::new();
    let mut perishable_items_held: u64 = 0;

    while let Some(batch) = reader.next() {
        let batch = batch?;
        total_agents += batch.num_rows();

        let age_ticks_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
        let is_alive_col = batch.column(4).as_any().downcast_ref::<arrow::array::BooleanArray>().unwrap();
        let inv_col = batch.column(15).as_any().downcast_ref::<StringArray>().unwrap();
        let attr_col = batch.column(16).as_any().downcast_ref::<StringArray>().unwrap();

        for i in 0..batch.num_rows() {
            let age_years = age_ticks_col.value(i) as f64 / 365.0;
            ages.push(age_years);
            if age_years > max_age_years {
                max_age_years = age_years;
            }

            let is_alive = is_alive_col.value(i);
            let attr_str = attr_col.value(i);
            if let Ok(attr_val) = serde_json::from_str::<serde_json::Value>(attr_str) {
                if let Some(g_num) = attr_val.get("generation").and_then(|g| g.as_u64()) {
                    if g_num > max_generation {
                        max_generation = g_num;
                    }
                }
            }

            if is_alive {
                alive_count += 1;
                alive_ages.push(age_years);

                let inv_str = inv_col.value(i);
                if let Ok(inv_map) = serde_json::from_str::<BTreeMap<String, u64>>(inv_str) {
                    for (item_id, qty) in inv_map {
                        *living_inventory_totals.entry(item_id.clone()).or_insert(0) += qty;
                        // Item 102 (Fish) and 104 (Berries) are perishable
                        if item_id == "102" || item_id == "104" {
                            perishable_items_held += qty;
                        }
                    }
                }
            } else {
                deceased_count += 1;
            }
        }
    }

    println!("👥 DEMOGRAPHIC LIFECYCLE AUDIT:");
    println!("  Total Agents Spawned/Born : {}", total_agents);
    println!("  Living Agents at Final Year: {}", alive_count);
    println!("  Deceased Cumulative        : {}", deceased_count);
    println!("  Deepest Generation Reached : Gen {}", max_generation);
    println!("  Max Age Reached            : {:.1} years", max_age_years);
    if !alive_ages.is_empty() {
        let avg_alive_age: f64 = alive_ages.iter().sum::<f64>() / alive_ages.len() as f64;
        println!("  Average Age of Survivors  : {:.1} years", avg_alive_age);
    }

    // 3. Analyze Ledger Transactions
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

    println!("\n📜 ULTIMATE LEDGER ECONOMIC ACTIVITY:");
    println!("  Total Transactions Recorded: {}", total_transactions);
    println!("  Transaction Types:");
    for (t, count) in &trx_breakdown {
        println!("    - {:<30} : {} ({:.1}%)", t, count, (*count as f64 / total_transactions as f64) * 100.0);
    }
    println!("  Capital Goods Fabricated Cumulative:");
    let mut total_tools_fabricated: u64 = 0;
    for (tool, count) in &tools_crafted {
        println!("    - {:<20} : {} units", tool, count);
        total_tools_fabricated += *count;
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

    // 4. Environment Nodes Status
    let env_file = File::open(run_dir.join("environment.parquet"))?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(env_file)?;
    let mut reader = builder.build()?;

    println!("\n🌲 ECOLOGICAL CARRYING CAPACITY STATUS AT FINAL TICK:");
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

    // 5. Historical Reality Evaluation (Initial vs Final)
    let initial_agents = 50.0;
    let cagr = ((alive_count as f64 / initial_agents).powf(1.0 / simulated_years.max(1.0)) - 1.0) * 100.0;
    let axes_held = *living_inventory_totals.get("108").unwrap_or(&0);
    let nets_held = *living_inventory_totals.get("109").unwrap_or(&0);
    let rafts_held = *living_inventory_totals.get("106").unwrap_or(&0);
    let total_tools_held = axes_held + nets_held + rafts_held;
    let tools_per_capita = if alive_count > 0 { total_tools_held as f64 / alive_count as f64 } else { 0.0 };

    println!("\n======================================================================");
    println!("🏛️  EVALUASI KOMPARATIF KONDISI AWAL VS AKHIR TERHADAP REALITA SEJARAH");
    println!("======================================================================");
    println!("┌────────────────────────────┬────────────────────┬────────────────────┬──────────────────────────────────────────────────┐");
    println!("│ Dimensi Evaluasi           │ Kondisi Awal (T_0) │ Kondisi Akhir (T_f)│ Tolok Ukur Realita Sejarah & Status              │");
    println!("├────────────────────────────┼────────────────────┼────────────────────┼──────────────────────────────────────────────────┤");
    println!("│ Populasi Hidup             │ 50 jiwa (Pionir)   │ {:<18} │ CAGR: {:+.2}%/tahun (Pra-industri: -0.2% s/d +0.3%)│", format!("{} jiwa", alive_count), cagr);
    println!("│ Kedalaman Generasi         │ Gen 1 (Pionir)     │ {:<18} │ Suksesi Biologis (Est. ~3-4 gen/abad)            │", format!("Gen {}", max_generation));
    println!("│ Total Kelahiran Historis   │ 0 kelahiran        │ {:<18} │ Regenerasi Demografi Alami                        │", format!("{} bayi", total_agents.saturating_sub(50)));
    println!("│ Alat Modal Beredar         │ 0 unit             │ {:<18} │ Akumulasi Fisik ({:.2} alat/kapita)              │", format!("{} unit", total_tools_held), tools_per_capita);
    println!("│ Total Alat Diproduksi      │ 0 unit             │ {:<18} │ Fabrikasi Modal Kumulatif                        │", format!("{} unit", total_tools_fabricated));
    println!("│ Makanan Perishable Beredar │ 0 unit             │ {:<18} │ Risiko Pembusukan Pangan Segar                    │", format!("{} unit", perishable_items_held));
    println!("│ Transaksi Ekonomi          │ 0 transaksi        │ {:<18} │ Aktivitas Ledger dan Pembagian Kerja             │", format!("{} transaksi", total_transactions));
    println!("└────────────────────────────┴────────────────────┴────────────────────┴──────────────────────────────────────────────────┘");

    // 6. Reality Anomaly Detection & Diagnostics
    println!("\n⚠️  DETEKSI ANOMALI REALITA & DIAGNOSA AKAR MASALAH:");
    let mut anomalies_found = 0;

    // Check Anomaly 1: Capital Durability / Depreciation
    if total_tools_fabricated > 0 && total_tools_held >= (total_tools_fabricated * 3 / 10) {
        anomalies_found += 1;
        println!("  1. 🔴 ANOMALI: MODAL ABADI (Immortal Capital Trap)");
        println!("     - Observasi : {} alat dibuat, {} alat masih beredar ({:.1} alat/kapita).", total_tools_fabricated, total_tools_held, tools_per_capita);
        println!("     - Realita   : Kapak batu dan anyaman purba mengalami aus, patah, atau lapuk seiring pemakaian.");
        println!("     - Diagnosa  : Belum ada mekanisme keausan fisik (durability / wear-and-tear) per kali panen.");
    }

    // Check Anomaly 2: Perishability Spoilage
    if perishable_items_held > 0 {
        anomalies_found += 1;
        println!("  2. 🟡 ANOMALI: PANGAN SEGAR TIDAK MEMBUSUK (Perishability Immunity)");
        println!("     - Observasi : {} unit makanan segar (ikan/beri) disimpan di tas tanpa pengawetan garam.", perishable_items_held);
        println!("     - Realita   : Protein basah membusuk dalam beberapa hari jika tidak diasinkan/dikeringkan.");
        println!("     - Diagnosa  : `is_perishable: true` hanya metadata pasif, belum dieksekusi pembusukan harian di MetabolismSystem.");
    }

    // Check Anomaly 3: Population Trajectory
    if alive_count == 0 {
        anomalies_found += 1;
        println!("  3. 🔴 ANOMALI: KEPUNAHAN TOTAL (Extinction Collapse)");
        println!("     - Observasi : Populasi punah 0 jiwa.");
        println!("     - Diagnosa  : Kegagalan keseimbangan fertilitas vs mortalitas atau malnutrisi massal.");
    } else if cagr < -0.4 {
        anomalies_found += 1;
        println!("  3. 🟡 ANOMALI: PENYUSUTAN DEMOGRAFI LAMBAT (Slow Population Attrition)");
        println!("     - Observasi : Populasi menyusut dari 50 ke {} (CAGR {:+.2}%/tahun).", alive_count, cagr);
        println!("     - Realita   : Komunitas perintis dengan sumber pangan berlimpah seharusnya stabil atau bertumbuh perlahan.");
        println!("     - Diagnosa  : Batas kalori fertilitas atau jangkauan mobilitas agen ke node makanan masih terlalu membatasi.");
    }

    if anomalies_found == 0 {
        println!("  ✅ TIDAK DITEMUKAN ANOMALI SIGNIFIKAN: Semua trajektori sesuai tolok ukur sejarah manusia!");
    } else {
        println!("\n  💡 Rekomendasi: Pertimbangkan menyempurnakan mekanisme mikro di atas untuk mencapai realitas sejarah penuh.");
    }

    // 7. Render Final Tables
    if tables_path.exists() {
        let tables_file = File::open(tables_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(tables_file)?;
        let mut reader = builder.build()?;
        let renderer = AsciiTableRenderer::new();

        println!("\n======================================================================");
        println!("📊 FINAL STRUCTURED STATISTICAL RELEASES (LATEST PUBLISHED TABLES)");
        println!("======================================================================\n");

        while let Some(batch) = reader.next() {
            let batch = batch?;
            let tick_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
            let table_json_col = batch.column(5).as_any().downcast_ref::<StringArray>().unwrap();

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
