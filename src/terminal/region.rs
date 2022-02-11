use super::cursor::{Cursor, Index};
use crate::terminal::size::Size;

/// Region represents an area in the terminal
#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
pub struct Region {
    pub(crate) left_top: Cursor,
    pub(crate) right_bottom: Cursor,
}

impl Region {
    /// Creates a new region
    ///
    /// # Panics
    /// Panic if `right_bottom` < `left_top`
    pub fn new(left_top: Cursor, right_bottom: Cursor) -> Self {
        assert!(right_bottom >= left_top);
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
        self.right_bottom.row() - self.left_top.row() + 1
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
                next_cursor = next_cursor.next_row();
                next_cursor.set_column(self.region.left_top().column());
                if next_cursor.row() > self.region.right_bottom().row() {
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
