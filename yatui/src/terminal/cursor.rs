use std::cmp::Ord;

/// For row and column indexing (u8 should be enough?)
pub type Index = u8;

/// Cursor points to a row and column of terminal
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Cursor {
    row: Index,
    column: Index,
}

impl Cursor {
    pub fn new(row: Index, column: Index) -> Self {
        Self { row, column }
    }
    pub fn row(&self) -> Index {
        self.row
    }
    pub fn column(&self) -> Index {
        self.column
    }
}
