use std::collections::BTreeMap;
use std::fs::File;
use arrow::array::{Array, Float64Array, StringArray, UInt64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use economy::core::domain::statistic::renderer::{AsciiTableRenderer, TableRenderer};
use economy::core::domain::statistic::table::StatisticalTable;

fn item_meta(id: u64) -> (&'static str, &'static str) {
    match id {
        101 => ("Timber / Firewood", "Good (Raw Material)"),
        102 => ("Fresh River Fish", "Good (Perishable Food)"),
        103 => ("Cultivated Grain", "Good (Staple Food)"),
        104 => ("Wild Forest Berries", "Good (Perishable Food/Flora)"),
        105 => ("Mineral Rock Salt", "Good (Preservative Mineral)"),
        106 => ("Maritime Timber Raft", "Capital (Water Transport)"),
        107 => ("Sea Cowrie Shells", "Good (Ornament/Currency)"),
        108 => ("Polished Stone Axe", "Capital (Forestry Tool)"),
        109 => ("Woven Fishing Net", "Capital (Marine Harvesting Tool)"),
        110 => ("Herbal Medicine", "Good (Therapeutic Pharmacopoeia)"),
        111 => ("Woven Carrying Basket", "Capital (Logistics Container)"),
        112 => ("Salt-Cured Preserved Fish", "Good (Preserved Food)"),
        113 => ("Sun-Dried Desiccated Berries", "Good (Preserved Food)"),
        114 => ("Wood-Smoked Preserved Fish", "Good (Preserved Food)"),
        115 => ("Fine Alluvial Clay", "Good (Raw Material)"),
        116 => ("Ceramic Storage Pottery Jar", "Capital (Granary Storage Container)"),
        117 => ("Quarried Lithic Stone", "Good (Raw Material)"),
        118 => ("Terrestrial Raw Meat", "Good (Perishable Food)"),
        119 => ("Wild Raw Hide", "Good (Raw Material)"),
        120 => ("Warm Leather Clothing", "Capital (Thermoregulation Apparel)"),
        121 => ("Wood-Smoked Preserved Meat", "Good (Preserved Food)"),
        201 => ("Raft Building Blueprint", "Knowledge (Non-Rival Blueprint)"),
        202 => ("Fish Curing Preservation", "Knowledge (Non-Rival Technique)"),
        203 => ("Fire-Making Technique", "Knowledge (Non-Rival Technique)"),
        204 => ("Tool Crafting Blueprint", "Knowledge (Non-Rival Blueprint)"),
        205 => ("Herbal Medicine Blueprint", "Knowledge (Non-Rival Blueprint)"),
        206 => ("Basket Weaving Blueprint", "Knowledge (Non-Rival Blueprint)"),
        207 => ("Pottery Making Blueprint", "Knowledge (Non-Rival Blueprint)"),
        208 => ("Leather Working & Tailoring", "Knowledge (Non-Rival Blueprint)"),
        301 => ("Fishing Right Permit", "Permit (Institutional Right)"),
        302 => ("Forestry Right Permit", "Permit (Institutional Right)"),
        401 => ("Manual Labor Service", "Service (Intangible Man-Hour)"),
        402 => ("Apprenticeship Tuition", "Service (Intangible Education)"),
        403 => ("Maritime Transport", "Service (Intangible Transport)"),
        404 => ("Medical Caregiving", "Service (Intangible Healthcare)"),
        _ => ("Custom Artifact", "Other"),
    }
}

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
    let mut living_sick_count: u64 = 0;
    let mut disease_related_deaths: u64 = 0;
    let mut starvation_deaths: u64 = 0;
    let mut old_age_deaths: u64 = 0;

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
            let attr_val: serde_json::Value = serde_json::from_str(attr_str).unwrap_or(serde_json::Value::Null);

            if let Some(g_num) = attr_val.get("generation").and_then(|g| g.as_u64()) {
                if g_num > max_generation {
                    max_generation = g_num;
                }
            }

            if is_alive {
                alive_count += 1;
                alive_ages.push(age_years);

                let is_sick = attr_val.get("is_sick").and_then(|v| v.as_bool()).unwrap_or(false);
                if is_sick {
                    living_sick_count += 1;
                }

                let inv_str = inv_col.value(i);
                if let Ok(inv_map) = serde_json::from_str::<BTreeMap<String, u64>>(inv_str) {
                    for (item_id, qty) in inv_map {
                        *living_inventory_totals.entry(item_id.clone()).or_insert(0) += qty;
                        // Item 102 (Fish), 104 (Berries), and 118 (Raw Meat) are perishable
                        if item_id == "102" || item_id == "104" || item_id == "118" {
                            perishable_items_held += qty;
                        }
                    }
                }
            } else {
                deceased_count += 1;
                if let Some(reason) = attr_val.get("death_reason").and_then(|v| v.as_str()) {
                    if reason.contains("Illness") || reason.contains("fever") {
                        disease_related_deaths += 1;
                    } else if reason.contains("Starvation") {
                        starvation_deaths += 1;
                    } else {
                        old_age_deaths += 1;
                    }
                }
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

    // 3. Analyze Ledger Transactions & Item Emergence Chronology
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
    let mut item_chronology: BTreeMap<u64, (u64, &'static str, &'static str, String)> = BTreeMap::new();

    let record_item = |chronology: &mut BTreeMap<u64, (u64, &'static str, &'static str, String)>, item_id: u64, tick: u64, context: &str| {
        let (name, cat) = item_meta(item_id);
        let entry = chronology.entry(item_id).or_insert((tick, name, cat, context.to_string()));
        if tick < entry.0 {
            entry.0 = tick;
            entry.3 = context.to_string();
        }
    };

    while let Some(batch) = reader.next() {
        let batch = batch?;
        total_transactions += batch.num_rows();

        let tick_col = batch.column(2).as_any().downcast_ref::<UInt64Array>().unwrap();
        let item_a_col = batch.column(5).as_any().downcast_ref::<UInt64Array>();
        let item_b_col = batch.column(6).as_any().downcast_ref::<UInt64Array>();
        let meta_col = batch.column(11).as_any().downcast_ref::<StringArray>().unwrap();

        for i in 0..batch.num_rows() {
            let tick = tick_col.value(i);

            if let Some(col_a) = item_a_col {
                if !col_a.is_null(i) {
                    record_item(&mut item_chronology, col_a.value(i), tick, "Ledger Outflow / Direct Transfer");
                }
            }
            if let Some(col_b) = item_b_col {
                if !col_b.is_null(i) {
                    record_item(&mut item_chronology, col_b.value(i), tick, "Ledger Inflow / Counterparty Transfer");
                }
            }

            let meta_str = meta_col.value(i);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(meta_str) {
                if let Some(t) = val.get("transaction_type").and_then(|v| v.as_str()) {
                    *trx_breakdown.entry(t.to_string()).or_insert(0) += 1;

                    match t {
                        "capital_tool_production" | "container_crafting" | "food_preservation" | "ceramic_storage" | "clothing_tailoring" => {
                            if let Some(tool) = val.get("tool_crafted").or_else(|| val.get("product_name")).and_then(|v| v.as_str()) {
                                *tools_crafted.entry(tool.to_string()).or_insert(0) += 1;
                                let tid = match tool {
                                    "Stone Axe" | "Polished Stone Axe" => 108,
                                    "Fishing Net" | "Woven Fishing Net" => 109,
                                    "Maritime Raft" | "Maritime Timber Raft" => 106,
                                    "Woven Basket" | "Woven Carrying Basket" => 111,
                                    "Salt-Cured Preserved Fish" | "Salt-Cured Fish" => 112,
                                    "Sun-Dried Desiccated Berries" | "Sun-Dried Berries" => 113,
                                    "Wood-Smoked Preserved Fish" | "Wood-Smoked Fish" => 114,
                                    "Wood-Smoked Preserved Meat" | "Wood-Smoked Meat" => 121,
                                    "Warm Leather Garment" | "Warm Leather Clothing" | "Leather Clothing" => 120,
                                    "Ceramic Storage Pottery Jar" | "Ceramic Pottery Jar" | "Pottery Jar" => 116,
                                    _ => 0,
                                };
                                if tid > 0 {
                                    record_item(&mut item_chronology, tid, tick, "Autonomous Production / Crafting");
                                }
                            }
                        }
                        "pharmacopoeia_preparation" => {
                            if let Some(prod) = val.get("product_name").or_else(|| val.get("tool_crafted")).and_then(|v| v.as_str()) {
                                *tools_crafted.entry(prod.to_string()).or_insert(0) += 1;
                            }
                            record_item(&mut item_chronology, 110, tick, "Herbal Pharmacopoeia Preparation");
                        }
                        "scientific_discovery" => {
                            if let Some(k) = val.get("knowledge_name").and_then(|v| v.as_str()) {
                                *discoveries.entry(k.to_string()).or_insert(0) += 1;
                                let kid = match k {
                                    "Raft Construction Blueprint" => 201,
                                    "Salting & Fish Curing Preservation" => 202,
                                    "Fire-Making Technique" => 203,
                                    "Tool Crafting Blueprint" => 204,
                                    "Herbal Medicine Blueprint" => 205,
                                    "Basket Weaving Blueprint" => 206,
                                    "Ceramic Pottery Firing Technique" | "Pottery Firing Blueprint" => 207,
                                    "Leather Working & Tailoring Blueprint" | "Leather Working Blueprint" => 208,
                                    _ => 0,
                                };
                                if kid > 0 {
                                    record_item(&mut item_chronology, kid, tick, "Spontaneous Eureka Discovery");
                                }
                            }
                        }
                        "medical_care_service" => {
                            record_item(&mut item_chronology, 404, tick, "Medical Consultation & Caregiving");
                        }
                        "knowledge_service_trade" => {
                            record_item(&mut item_chronology, 402, tick, "Apprenticeship Knowledge Tuition");
                        }
                        "natural_resource_harvest" => {
                            total_harvests += 1;
                            if let Some(mult) = val.get("tool_multiplier").and_then(|v| v.as_f64()) {
                                if mult > 1.0 {
                                    tool_multiplied_harvests += 1;
                                }
                            }
                        }
                        "bilateral_barter" => {
                            let a_inf = val.get("agent_a_informed").and_then(|v| v.as_bool()).unwrap_or(false);
                            let b_inf = val.get("agent_b_informed").and_then(|v| v.as_bool()).unwrap_or(false);
                            if a_inf || b_inf {
                                barter_informed_count += 1;
                            } else {
                                barter_uninformed_count += 1;
                            }
                        }
                        _ => {}
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

    // 4. Chronological Item Appearance in History
    println!("\n======================================================================");
    println!("⏳ DAFTAR KRONOLOGIS KEMUNCULAN & PENEMUAN ITEM SEPANJANG SEJARAH");
    println!("======================================================================");
    println!("┌─────┬──────────────────────────┬─────────────────────────────┬──────────┬──────────┬──────────────────────────────────────────┐");
    println!("│ ID  │ Nama Item                │ Kategori                    │ Tick     │ Tahun    │ Konteks Kemunculan / Mekanisme           │");
    println!("├─────┼──────────────────────────┼─────────────────────────────┼──────────┼──────────┼──────────────────────────────────────────┤");

    let mut sorted_chronology: Vec<(u64, u64, &'static str, &'static str, String)> = item_chronology
        .into_iter()
        .map(|(id, (tick, name, cat, ctx))| (tick, id, name, cat, ctx))
        .collect();
    sorted_chronology.sort_by_key(|(tick, id, _, _, _)| (*tick, *id));

    for (tick, id, name, cat, ctx) in &sorted_chronology {
        let yr = *tick as f64 / 365.0;
        println!("│ {:<3} │ {:<24} │ {:<27} │ {:<8} │ Thn {:<4.1} │ {:<40} │",
            id, name, cat, tick, yr, ctx);
    }
    println!("└─────┴──────────────────────────┴─────────────────────────────┴──────────┴──────────┴──────────────────────────────────────────┘");

    // 5. Epidemiology, Disease & Healthcare Sector Audit
    let medical_services_count = *trx_breakdown.get("medical_care_service").unwrap_or(&0);
    let pharmacopoeia_count = *trx_breakdown.get("pharmacopoeia_preparation").unwrap_or(&0);
    let medical_eureka_count = *discoveries.get("Herbal Medicine Blueprint").unwrap_or(&0);

    println!("\n======================================================================");
    println!("🏥 EVALUASI EPIDEMIOLOGI, PENYAKIT & PENGOBATAN (HEALTHCARE AUDIT)");
    println!("======================================================================");
    println!("  - Warga Hidup Sakit Saat Ini       : {} jiwa", living_sick_count);
    println!("  - Kematian Komplikasi Sakit/Demam  : {} jiwa", disease_related_deaths);
    println!("  - Kematian Kelaparan Murni         : {} jiwa", starvation_deaths);
    println!("  - Kematian Lanjut Usia / Alami     : {} jiwa", old_age_deaths);
    println!("  - Transaksi Jasa Medis (Dokter)    : {} konsultasi", medical_services_count);
    println!("  - Pembuatan Obat Herbal (Farmasi)  : {} batch obat", pharmacopoeia_count);
    println!("  - Terobosan Eureka Medis           : {} kali", medical_eureka_count);

    // 6. Historical Item Gap Analysis (Archaeological Matrix)
    println!("\n======================================================================");
    println!("🏛️ EVALUASI KESENJANGAN ITEM SEJARAH (ARCHAEOLOGICAL ITEM GAP ANALYSIS)");
    println!("======================================================================");
    println!("┌────────────────────────────┬──────────────────────────────────────┬──────────────────────────────┬──────────────────────────────┐");
    println!("│ Era / Periode Sejarah      │ Item Arkeologis Seharusnya Ada       │ Item Telah Ada di Model      │ Kesenjangan (Item Gaps)      │");
    println!("├────────────────────────────┼──────────────────────────────────────┼──────────────────────────────┼──────────────────────────────┤");
    println!("│ Paleolitik Bawah / Tengah  │ Kayu, Daging Liar, Kulit Hewan,      │ Kayu (101), Daging (118),    │ Bilah Batu Kasar (Chopper),  │");
    println!("│ (300.000 - 50.000 BP)      │ Batu Kuari, Api Unggun, Herba        │ Kulit (119), Batu Kuari(117) │ Pemantik Api Gesek           │");
    println!("├────────────────────────────┼──────────────────────────────────────┼──────────────────────────────┼──────────────────────────────┤");
    println!("│ Paleolitik Atas            │ Kapak Batu (Batu+Kayu), Rakit,       │ Kapak Batu (108), Rakit(106),│ Jarum Tulang Halus,          │");
    println!("│ (50.000 - 10.000 BP)       │ Pakaian Kulit Jahit, Daging Asap     │ Baju Kulit(120), Daging Asap │ Pigmen/Oker Merah Purba      │");
    println!("├────────────────────────────┼──────────────────────────────────────┼──────────────────────────────┼──────────────────────────────┤");
    println!("│ Mesolitik                  │ Jaring Ikan Anyam, Garam Pengawet,   │ Jaring(109), Garam(105),     │ Busur & Panah Pemburu,       │");
    println!("│ (10.000 - 8.000 BP)        │ Ikan Asin, Wadah Anyaman             │ Wadah Anyam (111), Kerang    │ Jebakan Ikan Rotan           │");
    println!("├────────────────────────────┼──────────────────────────────────────┼──────────────────────────────┼──────────────────────────────┤");
    println!("│ Neolitik                   │ Gandum Tanam, Gerabah/Tempayan Liat, │ Gandum (103), Tempayan (116) │ Sabit Batu Panen,            │");
    println!("│ (8.000 - 4.000 BP)         │ Hewan Ternak Domestik, Tenun Tekstil │ Pendidikan (402)             │ Hewan Ternak Domestik        │");
    println!("├────────────────────────────┼──────────────────────────────────────┼──────────────────────────────┼──────────────────────────────┤");
    println!("│ Logam & Perunggu Awal      │ Peleburan Tembaga/Perunggu, Sabit,   │ Hak Institusi (301, 302),    │ Tungku Smelter, Biji Tembaga,│");
    println!("│ (4.000 - 1.200 BP)         │ Gerobak Roda, Farmakope, Pembukuan   │ Buku Besar Ledger Kas        │ Alat Perunggu, Gerobak Kayu  │");
    println!("└────────────────────────────┴──────────────────────────────────────┴──────────────────────────────┴──────────────────────────────┘");

    // 7. Environment Nodes Status
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

    // 8. Historical Reality Evaluation (Initial vs Final)
    let initial_agents = 50.0;
    let cagr = ((alive_count as f64 / initial_agents).powf(1.0 / simulated_years.max(1.0)) - 1.0) * 100.0;
    let axes_held = *living_inventory_totals.get("108").unwrap_or(&0);
    let nets_held = *living_inventory_totals.get("109").unwrap_or(&0);
    let rafts_held = *living_inventory_totals.get("106").unwrap_or(&0);
    let baskets_held = *living_inventory_totals.get("111").unwrap_or(&0);
    let jars_held = *living_inventory_totals.get("116").unwrap_or(&0);
    let clothing_held = *living_inventory_totals.get("120").unwrap_or(&0);
    let total_tools_held = axes_held + nets_held + rafts_held + baskets_held + jars_held + clothing_held;
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
    println!("│ Rantai Resep & Buruan Liar │ 0 resep / 1 fauna  │ {:<18} │ 11 Resep multi-input, fauna darat & air realistis │", format!("{} resep / 2 fauna", tools_crafted.len()));
    println!("└────────────────────────────┴────────────────────┴────────────────────┴──────────────────────────────────────────────────┘");

    // 9. Reality Anomaly Detection & Diagnostics
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

    // Check Anomaly 2: Perishability Spoilage & Hoarding
    let perishable_per_capita = if alive_count > 0 { perishable_items_held as f64 / alive_count as f64 } else { 0.0 };
    if perishable_per_capita > 6.0 {
        anomalies_found += 1;
        println!("  2. 🟡 ANOMALI: PENIMBUNAN PANGAN SEGAR TIDAK WAJAR (Perishability Hoarding Trap)");
        println!("     - Observasi : {} unit makanan segar ({:.1} unit/kapita) disimpan tanpa pengawetan garam/asap/jemur.", perishable_items_held, perishable_per_capita);
        println!("     - Realita   : Protein basah dan buah beri membusuk dalam beberapa hari jika melebihi jatah konsumsi harian.");
        println!("     - Diagnosa  : Pembusukan harian belum mengimbangi laju panen segar agen kenyang.");
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
        println!("  ✅ TIDAK DITEMUKAN ANOMALI SIGNIFIKAN: Semua trajektori mikro sesuai tolok ukur sejarah manusia!");
    } else {
        println!("\n  💡 Rekomendasi: Sempurnakan mekanisme mikro di atas untuk mencapai realitas sejarah penuh.");
    }

    // 10. Recipe Origin & Hunting Diversity Audit
    let meat_held = *living_inventory_totals.get("118").unwrap_or(&0);
    let hide_held = *living_inventory_totals.get("119").unwrap_or(&0);
    let stone_held = *living_inventory_totals.get("117").unwrap_or(&0);
    let smoked_meat_crafted = *tools_crafted.get("Wood-Smoked Preserved Meat").unwrap_or(&0);
    let clothing_crafted = *tools_crafted.get("Warm Leather Garment").unwrap_or(&0);
    let leather_eureka = *discoveries.get("Leather Working & Tailoring Blueprint").unwrap_or(&0);

    println!("\n======================================================================");
    println!("🏹 EVALUASI RANTAI PASOK RESEP & KERAGAMAN BURUAN (HUNTING & RECIPE AUDIT)");
    println!("======================================================================");
    println!("  - Stok Kuari Batu (Stone) di Warga : {} unit", stone_held);
    println!("  - Stok Daging Buruan Segar (Meat)  : {} unit", meat_held);
    println!("  - Stok Kulit Hewan Liar (Raw Hide) : {} unit", hide_held);
    println!("  - Daging Asap Diproduksi (Preserved): {} unit", smoked_meat_crafted);
    println!("  - Pakaian Kulit Dibuat (Clothing)  : {} helai", clothing_crafted);
    println!("  - Pakaian Kulit Beredar Saat Ini   : {} helai", clothing_held);
    println!("  - Eureka Penyamakan Kulit & Jahit  : {} penemu", leather_eureka);

    // 11. Render Final Tables
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
