use std::collections::BTreeMap;

use crate::core::domain::agent::human::Human;
use crate::core::domain::agent::traits::HasLifecycle;
use crate::core::domain::item::id::ItemId;
use crate::core::domain::statistic::calculator::StatisticContext;
use crate::core::domain::statistic::table::{
    ColumnAlignment, StatisticalTable, TableCell, TableColumn, TableRow, TableSummaryRow,
};
use crate::core::domain::statistic::table_calculators::traits::StatisticalTableCalculator;

/// 2. Circulating Commodity & Asset Census Table Calculator (TAB_COMM_01)
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
