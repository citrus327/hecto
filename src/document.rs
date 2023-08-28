use std::fs;

use crate::row::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value))
        }

        Ok(Self {
            rows: rows,
            file_name: Some(file_name.to_string()),
        })
    }

    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
