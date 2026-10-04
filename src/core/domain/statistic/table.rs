use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::core::domain::statistic::access::AccessRequirement;
use crate::core::domain::statistic::schedule::ReleaseSchedule;
use crate::core::domain::statistic::table_calculator::StatisticalTableCalculator;
use crate::core::domain::time::Tick;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub id: String,
    pub label: String,
    pub unit: Option<String>,
    pub alignment: ColumnAlignment,
}

impl TableColumn {
    pub fn new(id: impl Into<String>, label: impl Into<String>, alignment: ColumnAlignment) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            unit: None,
            alignment,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TableCell {
    Text(String),
    Integer(i64),
    Float { value: f64, decimals: usize },
    Percentage(f64),
}

impl TableCell {
    pub fn text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }

    pub fn int(v: i64) -> Self {
        Self::Integer(v)
    }

    pub fn float(value: f64, decimals: usize) -> Self {
        Self::Float { value, decimals }
    }

    pub fn percent(value: f64) -> Self {
        Self::Percentage(value)
    }

    pub fn format_value(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Integer(i) => i.to_string(),
            Self::Float { value, decimals } => format!("{:.*}", decimals, value),
            Self::Percentage(p) => format!("{:.1}%", p),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> Self {
        Self { cells }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSummaryRow {
    pub label: String,
    pub cells: Vec<TableCell>,
}

impl TableSummaryRow {
    pub fn new(label: impl Into<String>, cells: Vec<TableCell>) -> Self {
        Self {
            label: label.into(),
            cells,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalTable {
    pub table_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub summary_rows: Vec<TableSummaryRow>,
    pub footnotes: Vec<String>,
}

impl StatisticalTable {
    pub fn new(table_id: impl Into<String>, title: impl Into<String>, columns: Vec<TableColumn>) -> Self {
        Self {
            table_id: table_id.into(),
            title: title.into(),
            subtitle: None,
            columns,
            rows: Vec::new(),
            summary_rows: Vec::new(),
            footnotes: Vec::new(),
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }

    pub fn add_summary_row(&mut self, summary: TableSummaryRow) {
        self.summary_rows.push(summary);
    }

    pub fn add_footnote(&mut self, footnote: impl Into<String>) {
        self.footnotes.push(footnote.into());
    }
}

/// Official published statistical table release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalTableRelease {
    pub release_id: u64,
    pub table_id: String,
    pub title: String,
    pub release_tick: Tick,
    pub schedule_desc: String,
    pub table: StatisticalTable,
    pub access_requirement: serde_json::Value,
}

/// Formal definition and schedule contract for statistical table publication
pub struct StatisticalTableDefinition {
    pub id: String,
    pub title: String,
    pub schedule: ReleaseSchedule,
    pub access: AccessRequirement,
    pub calculator: Arc<dyn StatisticalTableCalculator>,
}

impl StatisticalTableDefinition {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        schedule: ReleaseSchedule,
        access: AccessRequirement,
        calculator: impl StatisticalTableCalculator + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            schedule,
            access,
            calculator: Arc::new(calculator),
        }
    }
}
