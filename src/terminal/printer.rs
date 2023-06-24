use log::info;

use crate::{
    backend::Backend,
    terminal::{Cursor, Index, Region, Size},
};

/// Printer allows you to write some data to the terminal window.
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
}

impl<'a> Printer<'a> {
    /// Creates `Printer` over specified `Backend` with current terminal size.
    ///
    /// Printer do not listen `backend` events (e.g. resize), so the size of created `printer`
    /// does not change. It is user responsibility to update `printer` for correct terminal size.
    /// Usually if `terminal` resized and `printer` have old size nothing bad will happen but
    /// some text will be truncated in the `terminal`. If you do not create `printer` manually, you
    /// have nothing to worry about: `printer` will be updated by the tui application's main
    /// thread.
    ///
    /// # Panics
    ///
    /// Panics if terminal size is undefined.
    pub fn new(backend: &'a mut dyn Backend) -> Self {
        let region = Region::with_size(Cursor::default(), backend.get_size().unwrap());
        info!("region = {:?}", region);
        Self { backend, region }
    }

    /// Write text with styles to the current mapped region.
    ///
    /// If the text does not fit in the current region, it will be cut off. This method *does not*
    /// insert line breaks to prevent text from overflowing its line box, instead it is cut off the
    /// text.
    /// Ignore all zero-width graphemes ("\t", "\0", etc.) excluding new line ("\n", "\r\n")
    // pub fn write<C, T>(&mut self, start_from: C, styled: T)
    // where
    //     C: Into<Cursor>,
    //     T: StyledStr,
    // {
    //     let mut part_iter = parts(styled.str(), styled.styles());
    //     self.write_with_iter(start_from, part_iter, false);
    // }

    /// Write text with styles to the current mapped region.
    ///
    /// If the text does not fit in the current region, it will be cut off. This method insert
    /// line breaks to prevent text from overflowing its line box.
    /// Ignore all zero-width graphemes ("\t", "\0", etc.) excluding new line ("\n", "\r\n")
    // pub fn write_wrap<C, T>(&mut self, start_from: C, styled: T)
    // where
    //     C: Into<Cursor>,
    //     T: StyledStr,
    // {
    //     let mut part_iter = parts(styled.str(), styled.styles());
    //     self.write_with_iter(start_from, part_iter, true);
    // }

    /// Fill current region with `styled`.
    ///
    /// `text` will be repeated until all region is changed.
    // pub fn fill<T>(&mut self, styled: T)
    // where
    //     T: StyledStr,
    // {
    //     let mut part_iter = parts(styled.str(), styled.styles()).cycle();
    //     self.write_with_iter((0, 0), part_iter, true);
    // }

    /// Clear current region with spaces.
    pub fn clear(&mut self) {
        // self.fill(" ");
    }

    ///
    #[must_use]
    pub fn padding(&mut self, padding: Index) -> Printer<'_> {
        self.try_padding(padding).unwrap()
    }

    ///
    #[must_use]
    pub fn try_padding(&mut self, padding: Index) -> Option<Printer<'_>> {
        let new_x = self.local_region().right_bottom().column().checked_sub(padding)?;
        let new_y = self.local_region().right_bottom().line().checked_sub(padding)?;
        Some(self.map(Region::new(Cursor::new(padding, padding), Cursor::new(new_x, new_y))))
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
    #[must_use]
    pub fn try_map(&mut self, region: Region) -> Option<Printer<'_>> {
        let global_left = self.local_to_global(region.left_top())?;
        let global_right = self.local_to_global(region.right_bottom())?;
        Some(Printer { backend: self.backend, region: Region::new(global_left, global_right) })
    }

    /// Map specified `line` to new printer.
    ///
    /// # Panics
    ///
    /// Panics if `line` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// let printer = printer.map_line(1);
    /// assert_eq!(printer.height(), 1);
    /// ```
    #[must_use]
    pub fn map_line(&mut self, line: Index) -> Printer<'_> {
        self.map(self.local_region().n_line(line).unwrap())
    }

    /// Map first line to the new printer. Never panics because first line always exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// let printer = printer.map_first_line();
    /// assert_eq!(printer.height(), 1);
    /// ```
    #[must_use]
    pub fn map_first_line(&mut self) -> Printer<'_> {
        self.map_line(0)
    }

    /// Map last line to the new printer. Never panics because last line always exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// let printer = printer.map_last_line();
    /// assert_eq!(printer.height(), 1);
    /// ```
    #[must_use]
    pub fn map_last_line(&mut self) -> Printer<'_> {
        self.map_line(self.height() - 1)
    }

    /// Map specified `column` to new printer.
    ///
    /// # Panics
    ///
    /// Panics if `column` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// let printer = printer.map_column(1);
    /// assert_eq!(printer.width(), 1);
    /// ```
    #[must_use]
    pub fn map_column(&mut self, column: Index) -> Printer<'_> {
        self.map(self.local_region().n_column(column).unwrap())
    }

    /// Map first column to the new printer. Never panics because first column always exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// let printer = printer.map_first_column();
    /// assert_eq!(printer.width(), 1);
    /// ```
    #[must_use]
    pub fn map_first_column(&mut self) -> Printer<'_> {
        self.map_column(0)
    }

    /// Map last column to the new printer. Never panics because last column always exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::{backend::Raw, terminal::{Cursor, Printer, Region}};
    /// # let mut backend = Raw::new(5, 5);
    /// # let mut printer = Printer::new(&mut backend);
    /// let printer = printer.map_last_column();
    /// assert_eq!(printer.width(), 1);
    /// ```
    #[must_use]
    pub fn map_last_column(&mut self) -> Printer<'_> {
        self.map_column(self.width() - 1)
    }

    /// Map current `printer` region to the inner `region`.
    ///
    /// Check not panic version [`try_map`](Self::try_map)
    ///
    /// # Panics
    ///
    /// Panics if `region` is not included in the current region.
    #[must_use]
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
        self.region.height()
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
        self.region.width()
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

    /// Return current mapped region. This region contains
    /// global coordinates of terminal. Possibly you look for [`local_region`](Self::local_region)
    pub fn mapped_region(&self) -> Region {
        self.region
    }

    // fn write_with_iter<'b, C, I>(&mut self, start_from: C, iter: I, wrap: bool)
    // where
    //     C: Into<Cursor>,
    //     I: IntoIterator<Item = Part<'b>>,
    // {
    //     let mut part_iter = iter.into_iter();
    //     let mut cursor = start_from.into();
    //
    //     if !self.local_region().have_cursor(cursor) {
    //         return;
    //     }
    //
    //     for part in part_iter {
    //         match part {
    //             Part::Str(part_str, part_width, part_style) => {
    //                 info!("Write {} on {:?}", part_str, cursor);
    //                 let mut remainder =
    //                     self.write_part_str(&mut cursor, part_str, part_width, part_style);
    //
    //                 if wrap {
    //                     while let Some((remainder_s, remainder_w)) = remainder {
    //                         cursor = cursor.wrap_line();
    //                         if cursor.line() == self.height() {
    //                             return;
    //                         }
    //
    //                         remainder = self.write_part_str(
    //                             &mut cursor,
    //                             remainder_s,
    //                             remainder_w,
    //                             part_style,
    //                         );
    //                     }
    //                 }
    //             }
    //             Part::NewLine => {
    //                 cursor = cursor.wrap_line();
    //                 if cursor.line() == self.height() {
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // // Returns the part of text that was cut off and the number of remaining graphemes
    // fn write_part_str<'b>(
    //     &mut self,
    //     cursor: &mut Cursor,
    //     s: &'b str,
    //     mut w: usize,
    //     style: Style,
    // ) -> Option<(&'b str, usize)> {
    //     let avail_width = self.width() - cursor.column();
    //
    //     if avail_width == 0 {
    //         Some((s, w))
    //     } else if w <= avail_width.into() {
    //         self.write_raw_inner(cursor, s, w, style);
    //         None
    //     } else {
    //         let (left, right) = truncate_str_to_width(s, avail_width.into());
    //
    //         if !left.is_empty() {
    //             self.write_raw_inner(cursor, left, avail_width as usize, style);
    //             w -= avail_width as usize;
    //         }
    //
    //         Some((right, w))
    //     }
    // }

    // fn write_raw_inner(&mut self, cursor: &mut Cursor, s: &str, w: usize, style: Style) {
    //     self.backend.move_cursor(self.local_to_global(*cursor).unwrap());
    //     self.backend.draw(s, style);
    //     cursor.map_column(|column| *column = column.saturating_add(w as Index));
    // }

    // Converts local row to the global.
    fn local_to_global_row(&self, local_row: Index) -> Option<Index> {
        if local_row < self.region.height() {
            Some(local_row + self.region.left_top().line())
        } else {
            None
        }
    }

    // Converts local column to the global.
    fn local_to_global_column(&self, local_column: Index) -> Option<Index> {
        if local_column < self.region.width() {
            Some(local_column + self.region.left_top().column())
        } else {
            None
        }
    }

    fn local_to_global(&self, local_cursor: Cursor) -> Option<Cursor> {
        Some(Cursor::new(
            self.local_to_global_column(local_cursor.column())?,
            self.local_to_global_row(local_cursor.line())?,
        ))
    }
}

// fn truncate_str_to_width(s: &str, new_width: usize) -> (&str, &str) {
//     if s.is_empty() {
//         return ("", "");
//     }
//
//     let mut width = 0;
//     let mut last_byte = 0;
//
//     for g in Text::create_graphemes(s) {
//         if width + g.width() > new_width {
//             break;
//         }
//         width += g.width();
//         last_byte = g.end();
//     }
//
//     if width == 0 {
//         ("", s)
//     } else {
//         (&s[0..=last_byte], &s[last_byte + 1..])
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::truncate_str_to_width;
//
//     fn truncate_str_to_width_() {
//         let s = "very very big line very very big";
//         assert_eq!(truncate_str_to_width(s, 15), ("very very big l", "ine very very big"));
//     }
//
//     fn truncate_str_to_width_double_width() {
//         let s = "老3456789";
//         assert_eq!(truncate_str_to_width(s, 4), ("老34", "56789"));
//     }
// }
