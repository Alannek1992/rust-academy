use std::fmt::{Display, Formatter};

use csv::{Error, ReaderBuilder, StringRecord};
use prettytable::{Cell, Row as PrettyRow, Table};

use crate::error::Result;

pub struct Csv {
    header: Row,
    rows: Vec<Row>,
}

impl Csv {
    pub fn from_str(input: &str) -> Result<Self> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(input.as_bytes());

        let header = reader.headers()?.clone();
        let rows: std::result::Result<Vec<Row>, Error> = reader
            .records()
            .map(|record| {
                let record = record?;
                Ok(Row::from_record(&record))
            })
            .collect();

        Ok(Self {
            header: Row::from_record(&header),
            rows: rows?,
        })
    }
}

impl Display for Csv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(PrettyRow::new(
            self.header
                .cells
                .iter()
                .map(|c| Cell::new(c.trim()))
                .collect(),
        ));

        self.rows.iter().for_each(|r| {
            table.add_row(PrettyRow::new(
                r.cells.iter().map(|c| Cell::new(c.trim())).collect(),
            ));
        });

        write!(f, "{}", table)
    }
}

struct Row {
    cells: Vec<String>,
}

impl Row {
    fn from_record(record: &StringRecord) -> Self {
        Self {
            cells: record.iter().map(|field| field.to_string()).collect(),
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
