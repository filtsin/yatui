use crate::{
    backend::Backend,
    terminal::{Cursor, Index, Region, Size},
    text::Text,
};

/// Printer is allow you to write some data to the terminal window.
///
/// Printer can write data only to the `mapped region`. This prevents components from writing
/// to a region occupied by another component. Component gets the `printer` object with
/// specified `region` that it can not increase (but can shrink).
///
/// When using `printer`, you do not need to think about the coordinates of the terminal. All
/// components works with local coordinates (starting by **(0, 0)**). `Printer` will take care of
/// translating local coordinates into global coordinates.
///
/// You can `remap` printer to another `region`. The new `region` must be inside the current
/// mapped region. Note, that `remap` works with local coordinates too, like **all** functions of
/// printer.
pub struct Printer<'a> {
    backend: &'a mut dyn Backend,

    region: Region,
    mapped_region: Region,
}

impl<'a> Printer<'a> {
    /// Creates `Printer` over specified `Backend` with current terminal size.
    ///
    /// Printer do not listen `backend` events (e.g. resize), so the size of created `printer`
    /// does not change. It is user responsobility to update `printer` for correct terminal size.
    /// Usually if `terminal` resized and `printer` have old size nothing bad will happen but
    /// some text will be cut in the `terminal`. If you do not create `printer` manually, you
    /// have nothing to worry about: `printer` will be updated by the tui application's main
    /// thread.
    ///
    /// # Panics
    ///
    /// Panics if terminal size is undefined.
    pub fn new(backend: &'a mut dyn Backend) -> Self {
        let region = Region::with_size(Cursor::default(), backend.get_size().unwrap());
        Self { backend, region, mapped_region: region }
    }

    /// Write text with styles to the current mapped region.
    ///
    /// If the text does not fit in the current region, it will be cut off.
    pub fn write<C>(&mut self, start_from: C, text: &Text)
    where
        C: Into<Cursor>,
    {
        todo!()
    }

    /// Write text without styles to the current mapped region.
    ///
    /// If the text does not fit in the current region, it will be cut off.
    pub fn write_str<C>(&mut self, start_from: C, s: &str)
    where
        C: Into<Cursor>,
    {
        todo!()
    }

    /// Fill current region with `text`.
    ///
    /// `text` will be repeated until all region is changed.
    pub fn fill(&mut self, text: &Text) {
        todo!()
    }

    /// Fill current region with `s`.
    ///
    /// `s` will be repeated until all region is changed.
    pub fn fill_str(&mut self, s: &str) {
        todo!()
    }

    /// Clear current region with spaces.
    pub fn clear(&mut self) {
        todo!()
    }

    /// Set padding for current mapped region and return new printer.
    pub fn padding(&mut self, padding: Index) -> Printer<'_> {
        todo!()
    }

    /// Try to remap current `printer` region to the inner `region`.
    ///
    /// If `region` included in the current printer's mapped region then `Some` will be returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// assert!(printer.try_map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3))).is_some());
    /// assert!(printer.try_map(Region::new(Cursor::new(10, 15), Cursor::new(30, 30))).is_none());
    /// ```
    pub fn try_map(&mut self, region: Region) -> Option<Printer<'_>> {
        let global_left = self.local_to_global(region.left_top())?;
        let global_right = self.local_to_global(region.right_bottom())?;
        Some(Printer {
            backend: self.backend,
            region: self.region,
            mapped_region: Region::new(global_left, global_right),
        })
    }

    /// Map specified `line` to new printer.
    ///
    /// # Panics
    ///
    /// Panics if `line` is out of bounds.
    pub fn map_line(&mut self, line: Index) -> Printer<'_> {
        todo!()
    }

    /// Map first line to the new printer. Never panics because first line always exists.
    pub fn map_first_line(&mut self) -> Printer<'_> {
        todo!()
    }

    /// Map last line to the new printer. Never panics because last line always exists.
    pub fn map_last_line(&mut self) -> Printer<'_> {
        todo!()
    }

    /// Map specified `column` to new printer.
    ///
    /// # Panics
    ///
    /// Panics if `column` is out of bounds.
    pub fn map_column(&mut self, column: Index) -> Printer<'_> {
        todo!()
    }

    /// Map first column to the new printer. Never panics because first column always exists.
    pub fn map_first_column(&mut self) -> Printer<'_> {
        todo!()
    }

    /// Map last column to the new printer. Never panics because last column always exists.
    pub fn map_last_column(&mut self) -> Printer<'_> {
        todo!()
    }

    /// Map current `printer` region to the inner `region`.
    ///
    /// Check not panic version [`try_map`](Self::try_map)
    ///
    /// # Panics
    ///
    /// Panics if `region` is not included in the current region.
    pub fn map(&mut self, region: Region) -> Printer<'_> {
        self.try_map(region).unwrap()
    }

    /// Height of current `mapped region`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(6, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// assert_eq!(printer.height(), 5);
    /// ```
    pub fn height(&self) -> Index {
        self.mapped_region.height()
    }

    /// Width of current `mapped region`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(6, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// assert_eq!(printer.width(), 6);
    /// ```
    pub fn width(&self) -> Index {
        self.mapped_region.width()
    }

    /// Local region of this printer. Always starts from **(0, 0)**.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// assert_eq!(printer.local_region(), Region::new(Cursor::new(0, 0), Cursor::new(4, 4)));
    /// ```
    pub fn local_region(&self) -> Region {
        Region::with_size(Cursor::new(0, 0), Size::new(self.width(), self.height()))
    }

    /// Return current mapped region to the region. This region contains
    /// global coordinates of terminal. Possibly you look for [`local_region`](Self::local_region)
    pub fn mapped_region(&self) -> Region {
        self.mapped_region
    }

    // Converts local row to the global.
    fn local_to_global_row(&self, local_row: Index) -> Option<Index> {
        if local_row < self.mapped_region.height() {
            Some(local_row + self.mapped_region.left_top().row())
        } else {
            None
        }
    }

    // Converts local column to the global.
    fn local_to_global_column(&self, local_column: Index) -> Option<Index> {
        if local_column < self.mapped_region.width() {
            Some(local_column + self.mapped_region.left_top().column())
        } else {
            None
        }
    }

    fn local_to_global(&self, local_cursor: Cursor) -> Option<Cursor> {
        Some(Cursor::new(
            self.local_to_global_column(local_cursor.column())?,
            self.local_to_global_row(local_cursor.row())?,
        ))
    }
}
