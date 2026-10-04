use crate::core::domain::agent::human::Sex;
use crate::core::domain::statistic::calculator::StatisticContext;
use crate::core::domain::statistic::table::{
    ColumnAlignment, StatisticalTable, TableCell, TableColumn, TableRow, TableSummaryRow,
};
use crate::core::domain::statistic::table_calculators::traits::StatisticalTableCalculator;

/// 1. Demographic Cohort & Vital Population Table Calculator (TAB_DEMO_01)
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

        let living_total = ctx.agents.count_alive();
        let total_historical = ctx.agents.count_total();
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

        for h in ctx.agents.iter_living_humans() {
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
