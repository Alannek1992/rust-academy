use std::{
    fmt::{Display, Formatter},
    fs::File,
};

use csv::{Error, ReaderBuilder, StringRecord};
use prettytable::{format, Cell, Row as PrettyRow, Table};

use crate::error::Result;

pub struct Csv {
    header: Row,
    rows: Vec<Row>,
}

impl Csv {
    pub fn from_file(file_path: &str) -> Result<Self> {
        let file = File::open(file_path.trim())?;
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

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

        // some fun
        table.add_row(PrettyRow::new(
            self.header
                .cells
                .iter()
                .map(|_| Cell::new_align("🦀", format::Alignment::CENTER))
                .collect(),
        ));

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
        let file_path = "examples/data.csv";
        let _csv = Csv::from_file(file_path).unwrap();
        assert!(true, "The CSV was succesfully constructed");
    }

    #[test]
    fn parsing_invalid_content() {
        let file_path = "examples/invalid_data.csv";
        let csv = Csv::from_file(file_path);
        assert_eq!(csv.is_err(), true);
    }
}
