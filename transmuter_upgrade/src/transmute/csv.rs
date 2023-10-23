use crate::error::{Error, Result};

pub struct Csv {
    header: Row,
    rows: Vec<Row>,
}

impl Csv {
    pub fn from_str(input: &str) -> Result<Self> {
        let mut lines_iter = input.lines();
        let first_line = lines_iter
            .next()
            .ok_or(Error::new("The CSV header must be provided!"))?;
        let header = Row::from_str(first_line);
        let mut rows = vec![];

        while let Some(line) = lines_iter.next() {
            let row: Row = Row::from_str(line);
            if row.cells.len() > header.cells.len() {
                return Err(Error::new(
                    "Invalid CSV file containing values for non existing columns",
                ));
            }
            rows.push(row);
        }

        Ok(Self { header, rows })
    }

    pub fn to_table_layout(&self) -> String {
        // Calculate the maximum width for each column
        let max_column_widths: Vec<usize> = (0..self.header.cells.len())
            .map(|col| {
                let header_width = self.header.cells[col].len();
                let row_widths: Vec<usize> = self
                    .rows
                    .iter()
                    .map(|row| {
                        row.cells.get(col).map(|cell| cell.len()).unwrap_or(0) // Handle empty cells
                    })
                    .collect();
                let max_row_width = row_widths.iter().max().unwrap_or(&0);
                std::cmp::max(header_width, *max_row_width)
            })
            .collect();

        // Create the header row
        let header_row = self
            .header
            .cells
            .iter()
            .enumerate()
            .map(|(col, cell)| format!("{:<width$}", cell, width = max_column_widths[col]));

        // Create the separator row
        let separator_row: String = max_column_widths
            .iter()
            .map(|&width| "-".repeat(width))
            .collect::<Vec<String>>()
            .join("  ");

        // Create rows for the data
        let data_rows: Vec<String> = self
            .rows
            .iter()
            .map(|row| {
                row.cells
                    .iter()
                    .enumerate()
                    .map(|(col, cell)| format!("{:<width$}", cell, width = max_column_widths[col]))
                    .collect::<Vec<String>>()
                    .join("  ")
            })
            .collect();

        // Combine all rows
        let mut table = String::new();
        table.push_str(&header_row.collect::<Vec<String>>().join("  "));
        table.push('\n');
        table.push_str(&separator_row);
        table.push('\n');
        table.push_str(&data_rows.join("\n"));

        table
    }
}

#[derive(Debug)]
struct Row {
    cells: Vec<String>,
}

impl Row {
    fn from_str(input: &str) -> Self {
        Self {
            cells: input.split_terminator(',').map(|c| c.to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "Name,Age,Location,Occupation
        John,30,New York,Engineer
        Alice,25,Los Angeles,Teacher
        Bob,35,Chicago,Doctor
        Eva,28,San Francisco,Designer";

        let csv = Csv::from_str(input).unwrap();
        let mut input_iter = input.lines();
        let expected_header_line = input_iter.next().unwrap();

        assert_eq!(csv.header.cells.join(","), expected_header_line.to_string());

        input_iter.zip(csv.rows).for_each(|(line, row)| {
            assert_eq!(row.cells.join(","), line.to_string());
        });
    }
}
