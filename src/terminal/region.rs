use super::cursor::{Cursor, Index};
use crate::terminal::size::Size;

/// Region represents an area in the terminal
#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
pub struct Region {
    left_top: Cursor,
    right_bottom: Cursor,
}

impl Region {
    /// Creates a new region
    ///
    /// # Panics
    /// Panic if `right_bottom` < `left_top`
    pub fn new(left_top: Cursor, right_bottom: Cursor) -> Self {
        Self::try_new(left_top, right_bottom).unwrap()
    }

    pub fn try_new(left_top: Cursor, right_bottom: Cursor) -> Option<Self> {
        match (
            right_bottom.column().checked_sub(left_top.column()),
            right_bottom.line().checked_sub(left_top.line()),
        ) {
            (Some(_), Some(_)) => Some(Self { left_top, right_bottom }),
            _ => None,
        }
    }

    pub fn first_line(&self) -> Region {
        let v = self.n_line(0);
        // SAFETY: first line is always exists
        unsafe { self.n_line(0).unwrap_unchecked() }
    }

    pub fn last_line(&self) -> Region {
        let v = self.n_line(self.height() - 1);
        // SAFETY: last line is always exists
        unsafe { v.unwrap_unchecked() }
    }

    pub fn first_column(&self) -> Region {
        let v = self.n_column(0);
        // SAFETY: first column is always exists
        unsafe { v.unwrap_unchecked() }
    }

    pub fn last_column(&self) -> Region {
        let v = self.n_column(self.width() - 1);
        // SAFETY: last column is always exists
        unsafe { v.unwrap_unchecked() }
    }

    pub fn n_line(&self, line: Index) -> Option<Region> {
        if line < self.height() {
            let line = self.left_top().line() + line;
            Some(Region::new(
                Cursor::new(self.left_top().column(), line),
                Cursor::new(self.right_bottom().column(), line),
            ))
        } else {
            None
        }
    }

    pub fn n_column(&self, column: Index) -> Option<Region> {
        if column < self.width() {
            let column = self.left_top().column() + column;
            Some(Region::new(
                Cursor::new(column, self.left_top().line()),
                Cursor::new(column, self.right_bottom().line()),
            ))
        } else {
            None
        }
    }

    pub fn with_size(left_top: Cursor, size: Size) -> Self {
        assert!(size.width() > 0 && size.height() > 0);

        let right_bottom = Cursor::new(
            left_top.column().checked_add(size.width() - 1).unwrap(),
            left_top.line().checked_add(size.height() - 1).unwrap(),
        );

        Self { left_top, right_bottom }
    }

    pub fn left_top(&self) -> Cursor {
        self.left_top
    }

    pub fn right_bottom(&self) -> Cursor {
        self.right_bottom
    }

    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    /// Count of columns in the region
    pub fn width(&self) -> Index {
        self.right_bottom.column() - self.left_top.column() + 1
    }

    /// Count of rows in the region
    pub fn height(&self) -> Index {
        self.right_bottom.line() - self.left_top.line() + 1
    }

    /// Count of rows multiplied to count of columns in the region
    pub fn area(&self) -> usize {
        self.width() as usize * self.height() as usize
    }
}

impl From<Size> for Region {
    fn from(v: Size) -> Self {
        Region::new(Cursor::new(0, 0), Cursor::new(v.width() - 1, v.height() - 1))
    }
}

/// Iterator over every cell in Region in horizontal order
#[derive(Eq, PartialEq, Debug)]
pub struct RegionIter {
    region: Region,
    cursor: Option<Cursor>,
}

impl Iterator for RegionIter {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.cursor {
            let mut next_cursor = cursor.next_column();

            if next_cursor.column() > self.region.right_bottom().column() {
                next_cursor = next_cursor.next_line();
                next_cursor.set_column(self.region.left_top().column());
                if next_cursor.line() > self.region.right_bottom().line() {
                    return None;
                }
            }

            self.cursor = Some(next_cursor);
        } else {
            self.cursor = Some(self.region.left_top());
        }

        self.cursor
    }
}

impl IntoIterator for Region {
    type Item = Cursor;

    type IntoIter = RegionIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { region: self, cursor: None }
    }
}
