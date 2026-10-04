use crate::core::domain::statistic::table::{
    ColumnAlignment, StatisticalTable, TableCell,
};

/// Port trait for formatting statistical tables into presentation strings
pub trait TableRenderer: Send + Sync {
    fn render(&self, table: &StatisticalTable) -> String;
}

// ============================================================================
// 1. Clean Unicode / ASCII Console Table Renderer
// ============================================================================

pub struct AsciiTableRenderer {
    use_box_drawing: bool,
}

impl AsciiTableRenderer {
    pub fn new() -> Self {
        Self {
            use_box_drawing: true,
        }
    }

    pub fn with_box_drawing(mut self, enabled: bool) -> Self {
        self.use_box_drawing = enabled;
        self
    }
}

impl Default for AsciiTableRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TableRenderer for AsciiTableRenderer {
    fn render(&self, table: &StatisticalTable) -> String {
        let num_cols = table.columns.len();
        if num_cols == 0 {
            return String::new();
        }

        // Calculate max width for each column
        let mut col_widths = Vec::with_capacity(num_cols);
        for (i, col) in table.columns.iter().enumerate() {
            let mut max_w = col.label.chars().count();
            if let Some(unit) = &col.unit {
                max_w = max_w.max(col.label.chars().count() + unit.chars().count() + 3);
            }

            for row in &table.rows {
                if let Some(cell) = row.cells.get(i) {
                    max_w = max_w.max(cell.format_value().chars().count());
                }
            }

            for summary in &table.summary_rows {
                if let Some(cell) = summary.cells.get(i) {
                    max_w = max_w.max(cell.format_value().chars().count());
                }
            }

            // Min width of 4 for padding
            col_widths.push(max_w.max(4));
        }

        let mut out = String::new();

        // Title and Subtitle
        out.push_str(&format!("📊 {}\n", table.title));
        if let Some(sub) = &table.subtitle {
            out.push_str(&format!("   {}\n", sub));
        }

        // Top Border
        // ┌──────┬──────┐
        out.push('┌');
        for (i, w) in col_widths.iter().enumerate() {
            out.push_str(&"─".repeat(*w + 2));
            if i + 1 < num_cols {
                out.push('┬');
            }
        }
        out.push_str("┐\n");

        // Header Row
        // │ Col1 │ Col2 │
        out.push('│');
        for (i, col) in table.columns.iter().enumerate() {
            let label = if let Some(u) = &col.unit {
                format!("{} ({})", col.label, u)
            } else {
                col.label.clone()
            };
            out.push_str(&format!(" {} ", pad_string(&label, col_widths[i], col.alignment)));
            out.push('│');
        }
        out.push('\n');

        // Header Separator
        // ├──────┼──────┤
        out.push('├');
        for (i, w) in col_widths.iter().enumerate() {
            out.push_str(&"─".repeat(*w + 2));
            if i + 1 < num_cols {
                out.push('┼');
            }
        }
        out.push_str("┤\n");

        // Data Rows
        for row in &table.rows {
            out.push('│');
            for i in 0..num_cols {
                let cell_str = row
                    .cells
                    .get(i)
                    .map(TableCell::format_value)
                    .unwrap_or_default();
                let alignment = table.columns[i].alignment;
                out.push_str(&format!(" {} ", pad_string(&cell_str, col_widths[i], alignment)));
                out.push('│');
            }
            out.push('\n');
        }

        // Summary Rows (if any)
        if !table.summary_rows.is_empty() {
            // Double Line Separator
            // ╞══════╪══════╡
            out.push('╞');
            for (i, w) in col_widths.iter().enumerate() {
                out.push_str(&"═".repeat(*w + 2));
                if i + 1 < num_cols {
                    out.push('╪');
                }
            }
            out.push_str("╡\n");

            for summary in &table.summary_rows {
                out.push('│');
                for i in 0..num_cols {
                    let cell_str = summary
                        .cells
                        .get(i)
                        .map(TableCell::format_value)
                        .unwrap_or_default();
                    let alignment = table.columns[i].alignment;
                    out.push_str(&format!(" {} ", pad_string(&cell_str, col_widths[i], alignment)));
                    out.push('│');
                }
                out.push('\n');
            }
        }

        // Bottom Border
        // └──────┴──────┘
        out.push('└');
        for (i, w) in col_widths.iter().enumerate() {
            out.push_str(&"─".repeat(*w + 2));
            if i + 1 < num_cols {
                out.push('┴');
            }
        }
        out.push_str("┘\n");

        // Footnotes
        if !table.footnotes.is_empty() {
            for note in &table.footnotes {
                out.push_str(&format!("ℹ️  {}\n", note));
            }
        }

        out
    }
}

// ============================================================================
// 2. Markdown Table Renderer (GitHub Flavored Markdown)
// ============================================================================

pub struct MarkdownTableRenderer;

impl TableRenderer for MarkdownTableRenderer {
    fn render(&self, table: &StatisticalTable) -> String {
        let num_cols = table.columns.len();
        if num_cols == 0 {
            return String::new();
        }

        let mut out = String::new();

        // Title and Subtitle
        out.push_str(&format!("### {}\n", table.title));
        if let Some(sub) = &table.subtitle {
            out.push_str(&format!("*{}*\n\n", sub));
        } else {
            out.push('\n');
        }

        // Header Row
        out.push('|');
        for col in &table.columns {
            let label = if let Some(u) = &col.unit {
                format!("{} ({})", col.label, u)
            } else {
                col.label.clone()
            };
            out.push_str(&format!(" {} |", label));
        }
        out.push('\n');

        // Alignment Separators
        out.push('|');
        for col in &table.columns {
            match col.alignment {
                ColumnAlignment::Left => out.push_str(":---|"),
                ColumnAlignment::Center => out.push_str(":---:|"),
                ColumnAlignment::Right => out.push_str("---:|"),
            }
        }
        out.push('\n');

        // Rows
        for row in &table.rows {
            out.push('|');
            for i in 0..num_cols {
                let cell_str = row
                    .cells
                    .get(i)
                    .map(TableCell::format_value)
                    .unwrap_or_default();
                out.push_str(&format!(" {} |", cell_str));
            }
            out.push('\n');
        }

        // Summary Rows
        for summary in &table.summary_rows {
            out.push('|');
            for i in 0..num_cols {
                let cell_str = summary
                    .cells
                    .get(i)
                    .map(TableCell::format_value)
                    .unwrap_or_default();
                out.push_str(&format!(" **{}** |", cell_str));
            }
            out.push('\n');
        }

        // Footnotes
        if !table.footnotes.is_empty() {
            out.push('\n');
            for note in &table.footnotes {
                out.push_str(&format!("> ℹ️ *{}*\n", note));
            }
        }

        out
    }
}

fn pad_string(s: &str, width: usize, alignment: ColumnAlignment) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }

    let diff = width - char_count;
    match alignment {
        ColumnAlignment::Left => format!("{}{}", s, " ".repeat(diff)),
        ColumnAlignment::Right => format!("{}{}", " ".repeat(diff), s),
        ColumnAlignment::Center => {
            let left = diff / 2;
            let right = diff - left;
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
        }
    }
}
