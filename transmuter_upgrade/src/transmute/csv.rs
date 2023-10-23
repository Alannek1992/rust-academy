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
        todo!()
    }
}

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn from_str(input: &str) -> Self {
        Self {
            cells: input.split_terminator(',').map(|c| Cell::new(c)).collect(),
        }
    }
}

struct Cell {
    content: String,
}

impl Cell {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}
