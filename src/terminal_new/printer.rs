use crate::{
    terminal::{
        cursor::{Cursor, Index},
        region::Region,
    },
    terminal_new::RawPrinter,
};

pub struct Printer<'a> {
    raw: &'a mut RawPrinter<'a>,
    region: Region,
}

impl<'a> Printer<'a> {
    pub(crate) fn new(raw: &'a mut RawPrinter<'a>, region: Region) -> Self {
        Self { raw, region }
    }

    /// Return mapped region of this `Printer`.
    ///
    /// This is a real `region` in the current terminal window. All components
    /// working with local coordinates starting with `(0, 0)`. Result of this function is global
    /// region. Possibly you don't want to use this function in your `components`. Check
    /// [width](Self::width), [height](Self::height).
    pub fn region(&self) -> Region {
        self.region
    }

    /// Return width of mapped region.
    pub fn width(&self) -> Index {
        self.region.width()
    }

    /// Return height of mapped region.
    pub fn height(&self) -> Index {
        self.region.height()
    }

    /// Try to remap current `Printer`.
    pub fn try_map(&'a mut self, region: Region) -> Option<Printer<'a>> {
        let global_left = self.global_cursor(region.left_top())?;
        let global_right = self.global_cursor(region.right_bottom())?;
        Some(Printer::new(self.raw, Region::new(global_left, global_right)))
    }

    /// Remap current `Printer`.
    ///
    /// Check [try_map](Self::try_map).
    ///
    /// # Panics
    ///
    /// Panics if `region` size is out of bounds.
    pub fn map(&'a mut self, region: Region) -> Printer<'a> {
        self.try_map(region).unwrap()
    }

    /// Map specified `line` to new `printer`.
    ///
    /// # Panics
    ///
    /// Panics if `line` is out of bounds.
    pub fn map_line(&'a mut self, line: Index) -> Printer<'a> {
        self.map(Region::new(Cursor::new(0, line), Cursor::new(self.region().width() - 1, line)))
    }

    /// Map specified `column` to new `printer`.
    ///
    /// # Panics
    ///
    /// Panics if `column` is out of bounds.
    pub fn map_column(&'a mut self, column: Index) -> Printer<'a> {
        self.map(Region::new(
            Cursor::new(column, 0),
            Cursor::new(column, self.region().height() - 1),
        ))
    }

    // Converts local row to the global
    fn global_row(&self, local_row: Index) -> Option<Index> {
        if local_row < self.region.height() {
            Some(local_row + self.region.left_top().row())
        } else {
            None
        }
    }

    // Converts local column to the global
    fn global_column(&self, local_column: Index) -> Option<Index> {
        if local_column < self.region.width() {
            Some(local_column + self.region.left_top().column())
        } else {
            None
        }
    }

    fn global_cursor(&self, local_cursor: Cursor) -> Option<Cursor> {
        Some(Cursor::new(
            self.global_column(local_cursor.column())?,
            self.global_row(local_cursor.row())?,
        ))
    }
}
