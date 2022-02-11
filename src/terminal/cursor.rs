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

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.column.partial_cmp(&other.column) {
            Some(Ordering::Equal | Ordering::Greater) => { /* nothing here */ }
            ord => return ord,
        }

        self.row.partial_cmp(&other.row)
    }
}

impl Ord for Cursor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
