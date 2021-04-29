use super::cursor::Cursor;

/// Region represents an area in the terminal
pub struct Region {
    left_top: Cursor,
    right_bottom: Cursor
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
    pub fn width(&self) -> u8 {
        self.right_bottom.column() - self.left_top.column()
    }
    /// Count of rows in the region
    pub fn height(&self) -> u8 {
        self.right_bottom.row() - self.left_top.row()
    }
    /// Count of rows multiplied to count of columns in the region
    pub fn area(&self) -> u16 {
        self.width() as u16 * self.height() as u16
    }
}
