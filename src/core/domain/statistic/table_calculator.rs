use std::collections::BTreeMap;

use crate::core::domain::agent::human::{Human, Sex};
use crate::core::domain::agent::traits::HasLifecycle;
use crate::core::domain::item::id::ItemId;
use crate::core::domain::statistic::calculator::StatisticContext;
use crate::core::domain::statistic::table::{
    ColumnAlignment, StatisticalTable, TableCell, TableColumn, TableRow, TableSummaryRow,
};

/// Interface for generating tabular statistical releases
pub trait StatisticalTableCalculator: Send + Sync {
    fn calculate_table(&self, ctx: &StatisticContext) -> StatisticalTable;
}

// ============================================================================
// 1. Demographic Cohort & Vital Population Table Calculator
// ============================================================================

pub struct DemographicCohortTableCalculator;

impl StatisticalTableCalculator for DemographicCohortTableCalculator {
    fn calculate_table(&self, ctx: &StatisticContext) -> StatisticalTable {
        let columns = vec![
            TableColumn::new("cohort", "Kelompok Usia (Kohor)", ColumnAlignment::Left),
            TableColumn::new("males", "Pria", ColumnAlignment::Right),
            TableColumn::new("females", "Wanita", ColumnAlignment::Right),
            TableColumn::new("total", "Total Jiwa", ColumnAlignment::Right),
            TableColumn::new("share", "Pangsa (%)", ColumnAlignment::Right),
            TableColumn::new("avg_calories", "Rata-rata Kalori", ColumnAlignment::Right).with_unit("kkal"),
            TableColumn::new("married", "Menikah", ColumnAlignment::Right),
            TableColumn::new("role", "Status Fungsional", ColumnAlignment::Left),
        ];

        let mut table = StatisticalTable::new(
            "TAB_DEMO_01",
            "Tabel Sensus Demografi & Struktur Kohor Penduduk",
            columns,
        )
        .with_subtitle(format!("Waktu Simulasi: Tick {} (Hari)", ctx.current_tick.0));

        let all_humans = ctx.agents.get_all_humans();
        let total_historical = all_humans.len();
        let living_humans: Vec<&Human> = all_humans.iter().filter(|h| h.is_alive()).collect();
        let living_total = living_humans.len();
        let deceased_count = total_historical.saturating_sub(living_total);

        // Cohort definition buckets:
        // 1. Children: 0 .. 15
        // 2. Prime Working & Fertile: 15 .. 45
        // 3. Mature Adults: 45 .. 65
        // 4. Elderly: 65+
        struct CohortAccumulator {
            label: &'static str,
            role: &'static str,
            males: usize,
            females: usize,
            total_calories: f64,
            married: usize,
        }

        let mut cohorts = [
            CohortAccumulator {
                label: "00 - 14 tahun (Balita & Anak)",
                role: "Dependen / Pengasuhan Keluarga",
                males: 0,
                females: 0,
                total_calories: 0.0,
                married: 0,
            },
            CohortAccumulator {
                label: "15 - 44 tahun (Usia Produktif Awal)",
                role: "Tenaga Kerja Aktif / Reproduktif",
                males: 0,
                females: 0,
                total_calories: 0.0,
                married: 0,
            },
            CohortAccumulator {
                label: "45 - 64 tahun (Usia Produktif Lanjut)",
                role: "Tenaga Kerja Terampil / Non-Reproduktif",
                males: 0,
                females: 0,
                total_calories: 0.0,
                married: 0,
            },
            CohortAccumulator {
                label: "65+ tahun (Lansia / Usia Emas)",
                role: "Dependen / Pensiun Alami",
                males: 0,
                females: 0,
                total_calories: 0.0,
                married: 0,
            },
        ];

        let mut total_age_years = 0.0;
        let mut total_males = 0;
        let mut total_females = 0;
        let mut total_living_calories = 0.0;
        let mut total_married = 0;

        for h in &living_humans {
            let age_years = (h.age_ticks as f64) / 365.0;
            total_age_years += age_years;
            total_living_calories += h.calorie_reserve;

            let cohort_idx = if age_years < 15.0 {
                0
            } else if age_years < 45.0 {
                1
            } else if age_years < 65.0 {
                2
            } else {
                3
            };

            let c = &mut cohorts[cohort_idx];
            match h.sex {
                Sex::Male => {
                    c.males += 1;
                    total_males += 1;
                }
                Sex::Female => {
                    c.females += 1;
                    total_females += 1;
                }
            }
            c.total_calories += h.calorie_reserve;
            if h.spouse_id.is_some() {
                c.married += 1;
                total_married += 1;
            }
        }

        // Add cohort rows
        for c in &cohorts {
            let cohort_total = c.males + c.females;
            let share = if living_total > 0 {
                (cohort_total as f64 / living_total as f64) * 100.0
            } else {
                0.0
            };
            let avg_cal = if cohort_total > 0 {
                c.total_calories / cohort_total as f64
            } else {
                0.0
            };

            table.add_row(TableRow::new(vec![
                TableCell::text(c.label),
                TableCell::int(c.males as i64),
                TableCell::int(c.females as i64),
                TableCell::int(cohort_total as i64),
                TableCell::percent(share),
                TableCell::float(avg_cal, 1),
                TableCell::int(c.married as i64),
                TableCell::text(c.role),
            ]));
        }

        // Summary Row 1: Overall Totals
        let overall_avg_cal = if living_total > 0 {
            total_living_calories / living_total as f64
        } else {
            0.0
        };

        table.add_summary_row(TableSummaryRow::new(
            "TOTAL POPULASI HIDUP",
            vec![
                TableCell::text("TOTAL POPULASI HIDUP"),
                TableCell::int(total_males as i64),
                TableCell::int(total_females as i64),
                TableCell::int(living_total as i64),
                TableCell::percent(100.0),
                TableCell::float(overall_avg_cal, 1),
                TableCell::int(total_married as i64),
                TableCell::text("Masyarakat Aktif"),
            ],
        ));

        // Structural Summary Metrics
        let productive_count = (cohorts[1].males + cohorts[1].females) + (cohorts[2].males + cohorts[2].females);
        let dependent_count = (cohorts[0].males + cohorts[0].females) + (cohorts[3].males + cohorts[3].females);
        let dependency_ratio = if productive_count > 0 {
            (dependent_count as f64 / productive_count as f64) * 100.0
        } else {
            0.0
        };

        let sex_ratio = if total_females > 0 {
            (total_males as f64 / total_females as f64) * 100.0
        } else {
            0.0
        };

        let avg_age = if living_total > 0 {
            total_age_years / living_total as f64
        } else {
            0.0
        };

        table.add_footnote(format!(
            "Angka Ketergantungan (Dependency Ratio): {:.1}% | Rasio Jenis Kelamin (Sex Ratio): {:.1} pria/100 wanita",
            dependency_ratio, sex_ratio
        ));
        table.add_footnote(format!(
            "Rata-rata Usia Penduduk: {:.1} tahun | Akumulasi Kematian Alami/Kelaparan: {} jiwa (dari {} total historis)",
            avg_age, deceased_count, total_historical
        ));

        table
    }
}

// ============================================================================
// 2. Circulating Commodity & Asset Census Table Calculator
// ============================================================================

pub struct CommodityCirculationTableCalculator;

impl StatisticalTableCalculator for CommodityCirculationTableCalculator {
    fn calculate_table(&self, ctx: &StatisticContext) -> StatisticalTable {
        let columns = vec![
            TableColumn::new("item_id", "ID", ColumnAlignment::Right),
            TableColumn::new("commodity", "Komoditas / Gagasan", ColumnAlignment::Left),
            TableColumn::new("ontology", "Sifat Ontologis", ColumnAlignment::Left),
            TableColumn::new("circulating", "Beredar (Warga)", ColumnAlignment::Right),
            TableColumn::new("nature_stock", "Cadangan Alam", ColumnAlignment::Right),
            TableColumn::new("per_capita", "Rata-rata/Kapita", ColumnAlignment::Right),
            TableColumn::new("max_single", "Max 1 Agen", ColumnAlignment::Right),
            TableColumn::new("velocity", "Frekuensi Transaksi", ColumnAlignment::Right),
            TableColumn::new("economic_role", "Peran Ekonomi", ColumnAlignment::Left),
        ];

        let mut table = StatisticalTable::new(
            "TAB_COMM_01",
            "Tabel Sensus Komoditas, Alat Modal & Sirkulasi Aset",
            columns,
        )
        .with_subtitle(format!("Waktu Simulasi: Tick {} (Hari)", ctx.current_tick.0));

        let all_humans = ctx.agents.get_all_humans();
        let living_humans: Vec<&Human> = all_humans.iter().filter(|h| h.is_alive()).collect();
        let living_pop = living_humans.len();

        // 1. Count circulating items and max concentration across living agents
        let mut circulating_counts: BTreeMap<ItemId, u32> = BTreeMap::new();
        let mut max_single_holdings: BTreeMap<ItemId, u32> = BTreeMap::new();

        for h in &living_humans {
            for (id, qty) in &h.inventory {
                *circulating_counts.entry(*id).or_insert(0) += qty;
                let current_max = max_single_holdings.entry(*id).or_insert(0);
                if *qty > *current_max {
                    *current_max = *qty;
                }
            }
        }

        // 2. Count nature reserves from environmental nodes
        let mut nature_reserves: BTreeMap<ItemId, u32> = BTreeMap::new();
        for node in ctx.environment.nodes() {
            *nature_reserves.entry(node.item_id).or_insert(0) += node.current_stock;
        }

        // 3. Count transaction velocity from Ultimate Ledger
        let mut item_trade_velocity: BTreeMap<ItemId, u64> = BTreeMap::new();
        for entry in ctx.ledger.all_entries() {
            for it in &entry.items_from_a {
                *item_trade_velocity.entry(it.item_id).or_insert(0) += 1;
            }
            for it in &entry.items_from_b {
                *item_trade_velocity.entry(it.item_id).or_insert(0) += 1;
            }
        }

        let registry = crate::core::domain::item::ItemRegistry::canonical();
        let mut total_circulating_physical: u64 = 0;
        let mut total_nature_physical: u64 = 0;
        let mut total_capital_tools: u64 = 0;
        let mut max_velocity_item = ItemId(0);
        let mut max_velocity_count = 0;

        for def in registry.all() {
            let item_id = def.id;
            let circ = circulating_counts.get(&item_id).copied().unwrap_or(0);
            let nat = nature_reserves.get(&item_id).copied();
            let per_capita = if living_pop > 0 {
                circ as f64 / living_pop as f64
            } else {
                0.0
            };
            let max_one = max_single_holdings.get(&item_id).copied().unwrap_or(0);
            let vel = item_trade_velocity.get(&item_id).copied().unwrap_or(0);

            if !item_id.is_knowledge() && !item_id.is_permit() {
                total_circulating_physical += circ as u64;
                if let Some(n) = nat {
                    total_nature_physical += n as u64;
                }
                if item_id.is_tool() {
                    total_capital_tools += circ as u64;
                }
            }

            if vel > max_velocity_count {
                max_velocity_count = vel;
                max_velocity_item = item_id;
            }

            let nature_cell = match nat {
                Some(n) => TableCell::int(n as i64),
                None => TableCell::text("-"),
            };

            let ontology = match def.nature {
                crate::core::domain::item::ItemNature::RivalPhysical => match def.category {
                    crate::core::domain::item::ItemCategory::Service => "Rival Jasa / Waktu",
                    crate::core::domain::item::ItemCategory::Currency => "Rival Alat Tukar",
                    _ => "Rival Bahan / Barang",
                },
                crate::core::domain::item::ItemNature::NonRivalKnowledge => "Non-Rival Gagasan",
                crate::core::domain::item::ItemNature::InstitutionalRight => "Hak Institusional",
            };

            let desc = def
                .attributes
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or(&def.name);

            table.add_row(TableRow::new(vec![
                TableCell::int(item_id.as_u64() as i64),
                TableCell::text(def.name.clone()),
                TableCell::text(ontology),
                TableCell::int(circ as i64),
                nature_cell,
                TableCell::float(per_capita, 2),
                TableCell::int(max_one as i64),
                TableCell::int(vel as i64),
                TableCell::text(desc),
            ]));
        }

        // Summary Rows
        table.add_summary_row(TableSummaryRow::new(
            "TOTAL STOK FISIK",
            vec![
                TableCell::text("Agregat"),
                TableCell::text("TOTAL STOK FISIK"),
                TableCell::text("Rival Fisik"),
                TableCell::int(total_circulating_physical as i64),
                TableCell::int(total_nature_physical as i64),
                TableCell::float(
                    if living_pop > 0 {
                        total_circulating_physical as f64 / living_pop as f64
                    } else {
                        0.0
                    },
                    2,
                ),
                TableCell::text("-"),
                TableCell::int(ctx.ledger.total_records() as i64),
                TableCell::text("Cadangan Fisik Konservatif"),
            ],
        ));

        let capital_share = if total_circulating_physical > 0 {
            (total_capital_tools as f64 / total_circulating_physical as f64) * 100.0
        } else {
            0.0
        };

        let dominant_name = match max_velocity_item.0 {
            101 => "Kayu Mentah (Timber)",
            102 => "Ikan Segar (Fish)",
            103 => "Biji Gandum (Grain)",
            104 => "Buah Beri (Berries)",
            105 => "Garam Kristal (Salt)",
            106 => "Rakit (Raft)",
            108 => "Kapak Batu (Stone Axe)",
            109 => "Jaring Ikan (Fishing Net)",
            _ => "Belum Ada (Barter Bilateral Acak)",
        };

        table.add_footnote(format!(
            "Intensitas Barang Modal (Capital Tool Ratio): {:.2}% ({} unit perkakas modal dari {} aset fisik)",
            capital_share, total_capital_tools, total_circulating_physical
        ));
        table.add_footnote(format!(
            "Komoditas Paling Likuid (Velocity Tertinggi): {} ({} transaksi pertukaran tercatat)",
            dominant_name, max_velocity_count
        ));

        table
    }
}

/// Official Structured Table Release: Master Economic Items, Services & Ontology Schema (TAB_ITEM_01)
pub struct MasterItemCatalogueTableCalculator;

impl StatisticalTableCalculator for MasterItemCatalogueTableCalculator {
    fn calculate_table(&self, _ctx: &StatisticContext) -> StatisticalTable {
        let columns = vec![
            TableColumn::new("item_id", "ID", ColumnAlignment::Right),
            TableColumn::new("name", "Nama Item / Jasa", ColumnAlignment::Left),
            TableColumn::new("category", "Kategori", ColumnAlignment::Left),
            TableColumn::new("nature", "Sifat Ontologi", ColumnAlignment::Left),
            TableColumn::new("weight_kg", "Berat (kg)", ColumnAlignment::Right),
            TableColumn::new("perishable", "Perishable", ColumnAlignment::Center),
            TableColumn::new("era", "Fase Sejarah", ColumnAlignment::Left),
            TableColumn::new("unit", "Satuan", ColumnAlignment::Left),
            TableColumn::new("utility", "Peran / Utilitas", ColumnAlignment::Left),
        ];

        let mut table = StatisticalTable::new(
            "TAB_ITEM_01",
            "Tabel Master Katalog Item, Jasa & Skema Ontologi Ekonomi",
            columns,
        )
        .with_subtitle("Standar Klasifikasi Komoditas, Layanan Jasa, Hak Properti & Gagasan Non-Rival");

        let registry = crate::core::domain::item::ItemRegistry::canonical();
        let mut total_goods = 0;
        let mut total_services = 0;
        let mut total_knowledge = 0;
        let mut total_permits = 0;

        for def in registry.all() {
            let cat_str = match def.category {
                crate::core::domain::item::ItemCategory::Good => {
                    total_goods += 1;
                    "Good (Barang)"
                }
                crate::core::domain::item::ItemCategory::Service => {
                    total_services += 1;
                    "Service (Jasa/Waktu)"
                }
                crate::core::domain::item::ItemCategory::Knowledge => {
                    total_knowledge += 1;
                    "Knowledge (Gagasan)"
                }
                crate::core::domain::item::ItemCategory::Permit => {
                    total_permits += 1;
                    "Permit (Hak Akses)"
                }
                crate::core::domain::item::ItemCategory::Currency => "Currency (Uang)",
            };

            let nature_str = match def.nature {
                crate::core::domain::item::ItemNature::RivalPhysical => "Rival Fisik/Waktu",
                crate::core::domain::item::ItemNature::NonRivalKnowledge => "Non-Rival Gagasan",
                crate::core::domain::item::ItemNature::InstitutionalRight => "Hak Institusional",
            };

            let era_str = def
                .attributes
                .get("historical_era")
                .and_then(|v| v.as_str())
                .unwrap_or("-");

            let unit_str = def
                .attributes
                .get("unit")
                .and_then(|v| v.as_str())
                .unwrap_or("-");

            let utility_str = def
                .attributes
                .get("utility_type")
                .and_then(|v| v.as_str())
                .unwrap_or("-");

            table.add_row(TableRow::new(vec![
                TableCell::int(def.id.as_u64() as i64),
                TableCell::text(def.name.clone()),
                TableCell::text(cat_str),
                TableCell::text(nature_str),
                TableCell::float(def.weight_kg, 2),
                TableCell::text(if def.is_perishable { "Ya" } else { "Tidak" }),
                TableCell::text(era_str),
                TableCell::text(unit_str),
                TableCell::text(utility_str),
            ]));
        }

        table.add_summary_row(TableSummaryRow::new(
            "TOTAL REPERTOAR ITEM",
            vec![
                TableCell::text("TOTAL REPERTOAR ITEM"),
                TableCell::text(format!("{} Definisi Terdaftar", registry.len())),
                TableCell::text(format!("{} Barang, {} Jasa", total_goods, total_services)),
                TableCell::text(format!("{} Gagasan, {} Izin", total_knowledge, total_permits)),
                TableCell::text("-"),
                TableCell::text("-"),
                TableCell::text("Lintas 6 Era Sejarah"),
                TableCell::text("-"),
                TableCell::text("Skema JSON Draft-07"),
            ],
        ));

        table.add_footnote("Jasa/Waktu (Service): Dikonsumsi seketika atau dialokasikan sebagai unit waktu kerja terukur.");
        table.add_footnote("Gagasan (Knowledge): Bersifat non-rivalrous, tidak berkurang saat diajarkan/ditransfer.");
        table.add_footnote("Skema atribut tervalidasi terhadap standard JSON Schema Draft-07 via ItemDefinition::json_schema().");

        table
    }
}

