use crate::core::domain::statistic::calculator::StatisticContext;
use crate::core::domain::statistic::table::{
    ColumnAlignment, StatisticalTable, TableCell, TableColumn, TableRow, TableSummaryRow,
};
use crate::core::domain::statistic::table_calculators::traits::StatisticalTableCalculator;

/// 3. Master Item, Service & Knowledge Taxonomy Table Calculator (TAB_ITEM_01)
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
