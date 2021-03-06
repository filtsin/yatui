use std::cmp::{Ord, Ordering};

/// For row and column indexing (u16 should be enough?)
pub type Index = u16;

/// Cursor points to a row and column of terminal
/// ((0, 0)-based) where (0,0) is top-left cell
#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
pub struct Cursor {
    column: Index,
    row: Index,
}

impl Cursor {
    pub fn new(column: Index, row: Index) -> Self {
        Self { column, row }
    }

    pub fn row(&self) -> Index {
        self.row
    }

    pub fn column(&self) -> Index {
        self.column
    }

    #[must_use]
    pub fn next_row(self) -> Cursor {
        Cursor { row: self.row() + 1, ..self }
    }

    #[must_use]
    pub fn next_column(self) -> Cursor {
        Cursor { column: self.column() + 1, ..self }
    }

    #[must_use]
    pub fn prev_row(self) -> Cursor {
        Cursor { row: self.row() - 1, ..self }
    }

    #[must_use]
    pub fn prev_column(self) -> Cursor {
        Cursor { column: self.column() - 1, ..self }
    }

    pub fn set_row(&mut self, row: Index) {
        self.row = row;
    }

    pub fn set_column(&mut self, column: Index) {
        self.column = column;
    }
}

impl From<(Index, Index)> for Cursor {
    fn from((column, row): (Index, Index)) -> Self {
        Self::new(column, row)
    }
}
