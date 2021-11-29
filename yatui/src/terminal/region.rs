use super::cursor::{Cursor, Index};

/// Region represents an area in the terminal
#[derive(Debug, Default)]
pub struct Region {
    pub left_top: Cursor,
    pub right_bottom: Cursor,
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
    /// Count of columns in the region
    pub fn width(&self) -> Index {
        self.right_bottom.column() - self.left_top.column()
    }
    /// Count of rows in the region
    pub fn height(&self) -> Index {
        self.right_bottom.row() - self.left_top.row()
    }
    /// Count of rows multiplied to count of columns in the region
    pub fn area(&self) -> usize {
        self.width() as usize * self.height() as usize
    }
}
