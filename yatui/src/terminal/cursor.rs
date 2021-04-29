use std::cmp::Ord;

/// Cursor points to a row and column of terminal
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Cursor {
    row: u8,
    column: u8
}

impl Cursor {
    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }
    pub fn row(&self) -> u8 {
        self.row
    }
    pub fn column(&self) -> u8 {
        self.column
    }
}
