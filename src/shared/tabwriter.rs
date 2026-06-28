use std::{collections::HashMap, fmt::Display, io};

use console::measure_text_width;

pub struct TabWriter<W: io::Write> {
    writer: W,
    column_widths: Vec<usize>,
    current_row: Vec<String>,
    header_row: Vec<String>,
    rows: Vec<Vec<String>>,
    padding: usize,
}

impl<W> TabWriter<W>
where
    W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            padding: 2,
            rows: vec![],
            current_row: vec![],
            header_row: vec![],
            column_widths: vec![],
        }
    }

    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn begin_row(&mut self) {
        self.current_row.clear();
    }

    pub fn add_cell<S: Display>(&mut self, cell: &S) {
        let cell_str: String = cell.to_string();
        self.current_row.push(cell_str);
    }

    pub fn end_row(&mut self) -> io::Result<()> {
        if self.current_row.is_empty() {
            return Ok(());
        }

        let row = self.current_row.clone();
        self.rows.push(row);

        self.current_row.clear();
        Ok(())
    }

    pub fn set_header_row<S: Display + Clone>(&mut self, content: &[S]) {
        for v in content {
            self.header_row.push(v.to_string());
        }
    }

    pub fn push_header<S: Display>(&mut self, cell: &S) {
        let cell_str = cell.to_string();
        if !self.header_row.contains(&cell_str) {
            self.header_row.push(cell.to_string());
        }
    }

    pub fn add_row<S: Display + Clone>(&mut self, content: &[S]) -> io::Result<()> {
        self.begin_row();
        for value in content {
            self.add_cell(value);
        }
        self.end_row()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if !self.rows.is_empty() {
            self.rows.insert(0, self.header_row.clone());

            // Recalculate column widths from all rows
            self.recalculate_widths();

            // Write all rows
            let rows = self.rows.clone();
            for row in &rows {
                self.write_row(row)?;
            }

            self.rows.clear();
        }

        self.writer.flush()
    }

    fn recalculate_widths(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        // Find max columns
        let max_cols = self.rows.iter().map(|row| row.len()).max().unwrap_or(0);

        // Initialize or resize column widths
        self.column_widths.resize(max_cols, 0);

        // Calculate widths
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                self.column_widths[i] = self.column_widths[i].max(measure_text_width(cell));
            }
        }
    }

    /// Write a formatted row with the current column widths
    fn write_row(&mut self, row: &[String]) -> io::Result<()> {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                write!(self.writer, "{}", " ".repeat(self.padding))?;
            }

            let width = if i < self.column_widths.len() {
                self.column_widths[i]
            } else {
                measure_text_width(cell)
            };

            write!(self.writer, "{cell}")?;

            let visible_len = measure_text_width(cell);
            if visible_len < width {
                write!(self.writer, "{}", " ".repeat(width - visible_len))?;
            }
        }
        writeln!(self.writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_tab_writer() {
        let mut output = Vec::new();
        {
            let mut tw = TabWriter::new(&mut output);
            tw.add_row(&["Name", "Age", "City"]).unwrap();
            tw.add_row(&["Alice", "30", "New York"]).unwrap();
            tw.add_row(&["Bob", "25", "London"]).unwrap();
            tw.flush().unwrap();
        }

        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));

        assert!(result.contains("Age"));
        assert!(result.contains("30"));
        assert!(result.contains("25"));

        assert!(result.contains("City"));
        assert!(result.contains("New York"));
        assert!(result.contains("London"));
    }

    #[test]
    fn test_custom_padding() {
        let mut output = Vec::new();
        let padding = 9;
        {
            let mut tw = TabWriter::new(&mut output).padding(padding);
            tw.add_row(&["X", "Y"]).unwrap();
            tw.add_row(&["1", "2"]).unwrap();
            tw.flush().unwrap();
        }

        assert!(
            output
                .get(1..padding + 1)
                .unwrap()
                .iter()
                .all(|c| char::from(*c) == ' ')
        );
    }

    #[test]
    fn test_varying_row_lengths() {
        let mut output = Vec::new();
        {
            let mut tw = TabWriter::new(&mut output);
            tw.add_row(&["A", "B", "C"]).unwrap();
            tw.add_row(&["1", "2"]).unwrap();
            tw.add_row(&["X", "Y", "Z", "W"]).unwrap();
            tw.flush().unwrap();
        }

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("W"));
    }
}
