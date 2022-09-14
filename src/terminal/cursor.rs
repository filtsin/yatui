pub type Index = u16;

/// Cursor points to a column (x-coord) and line (y-coord) of terminal
/// ((0, 0)-based) where (0,0) is left-top cell
#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
pub struct Cursor {
    column: Index,
    line: Index,
}

impl Cursor {
    pub fn new(column: Index, line: Index) -> Self {
        Self { column, line }
    }

    pub fn line(&self) -> Index {
        self.line
    }

    pub fn column(&self) -> Index {
        self.column
    }

    pub fn map_line<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Index),
    {
        f(&mut self.line)
    }

    pub fn map_column<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Index),
    {
        f(&mut self.column)
    }

    #[must_use]
    pub fn next_line(self) -> Cursor {
        Self::new(self.column, self.line.saturating_add(1))
    }

    #[must_use]
    pub fn prev_line(self) -> Cursor {
        Self::new(self.column, self.line.saturating_sub(1))
    }

    #[must_use]
    pub fn next_column(self) -> Cursor {
        Self::new(self.column.saturating_add(1), self.line)
    }

    #[must_use]
    pub fn prev_column(self) -> Cursor {
        Self::new(self.column.saturating_sub(1), self.line)
    }

    #[must_use]
    pub fn wrap_line(self) -> Cursor {
        let mut result = self.next_line();
        result.set_column(0);
        result
    }

    pub fn set_line(&mut self, line: Index) {
        self.line = line;
    }

    pub fn set_column(&mut self, column: Index) {
        self.column = column;
    }
}

impl From<(Index, Index)> for Cursor {
    fn from((column, line): (Index, Index)) -> Self {
        Self::new(column, line)
    }
}
