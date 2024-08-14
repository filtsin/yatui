use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use std::{borrow::Cow, cell::Cell};

/// Raw - it is just a wrapper about std string.
/// The only responsibility of `RawText` is cache size of string and invalidate it when
/// string is changed
#[derive(Debug, Default)]
pub(super) struct Raw {
    data: Cow<'static, str>,
    size: Cell<Option<Size>>,
}

/// Cached size for `RawText` in terminal way
///
/// You should know:
///
/// 1) Not every grapheme have width=1. For example, cjk symbols: "す" have width=2.
/// This means that you need two terminal columns to display it.
///
/// 2) `Height` is count of lines in terminal way. Empty line is line too. Line delimiter
/// is "\n" or "\r\n". The final line ending is optional and do not create a new line in terminal.
///
/// 3) Why cache `graphemes`? For `DoubleEndedIterator` by graphemes :)
///
/// ```text
/// 
/// 1\n2 - 2 lines
/// 1\n2\r\n3 - 3 lines
/// 1\n2\r\n3\n - 3 lines too
/// ```
#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Size {
    /// Width of `text` in terminal way
    /// It is the largest line in the `text`
    width: usize,
    /// Height of `text` in terminal way
    /// It is count of lines in the `text`
    height: usize,
    /// Count of unicode graphemes (by `unicode_width` crate)
    graphemes: usize,
}

impl Raw {
    fn new() -> Self {
        Self::default()
    }

    pub(super) fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        let result = f(self.data.to_mut());
        self.invalidate_cache();
        result
    }

    pub(super) fn as_str(&self) -> &str {
        &self.data
    }

    fn width(&self) -> usize {
        self.get_size().width
    }

    fn height(&self) -> usize {
        self.get_size().height
    }

    fn graphemes(&self) -> usize {
        self.get_size().graphemes
    }

    fn invalidate_cache(&self) {
        self.size.set(None);
    }

    /// Get current cached `Size`. If cache was invalidated compute and cache new `Size`
    fn get_size(&self) -> Size {
        let size = self.size.get();

        match size {
            Some(s) => s,
            None => {
                let size = Size::compute(&self.data);
                self.size.set(Some(size));
                size
            }
        }
    }
}

impl Size {
    fn compute(s: &str) -> Size {
        let mut result = Size::default();

        for line in s.split_inclusive("\n") {
            let width = UnicodeWidthStr::width(line);

            result.width = result.width.max(width);
            // TODO: may be remove?
            result.graphemes += UnicodeSegmentation::graphemes(line, true).count();

            result.height += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modify_invalidate_raw_cache() {
        let mut raw =
            Raw { data: Cow::Borrowed("hello"), size: Cell::new(Some(Size::compute("hello"))) };

        raw.modify(|_| {});

        assert_eq!(raw.size.get(), None);
    }

    #[test]
    fn compute_size_different_types_of_unicode_points() {
        let empty_string = "";
        let ascii_string = "d";
        let japan_string = "老";
        let russian_string = "т";
        let control_string = "\n";
        let one_unicode_point_string = "\u{00fd}"; // it is Latin Small Letter Y with Acute "ý"
        let two_unicode_point_string = "y\u{0301}"; // it is Latin Small Letter Y with Combining
        // Acute Accent "◌́"

        assert_eq!(Size::compute(empty_string), Size { width: 0, height: 0, graphemes: 0 });
        assert_eq!(Size::compute(ascii_string), Size { width: 1, height: 1, graphemes: 1 });
        assert_eq!(Size::compute(japan_string), Size { width: 2, height: 1, graphemes: 1 });
        assert_eq!(Size::compute(russian_string), Size { width: 1, height: 1, graphemes: 1 });
        assert_eq!(Size::compute(control_string), Size { width: 0, height: 1, graphemes: 1 });
        assert_eq!(
            Size::compute(one_unicode_point_string),
            Size { width: 1, height: 1, graphemes: 1 }
        );
        assert_eq!(
            Size::compute(two_unicode_point_string),
            Size { width: 1, height: 1, graphemes: 1 }
        );
    }

    #[test]
    fn compute_size() {
        let empty_string = "";
        let one_line_string = "hello";
        let two_lines_string = "h\ne";
        let different_width_of_lines_string = "bigger\nless";

        let japan_string = "老老虎\nこれは単なるテストです";
        let control_string = "\n\t\r\n";
        let russian_string = "тест\nпривет";

        let multiple_lines_string = "1\n22\n333\r\n4444\r\n\n666666";

        assert_eq!(Size::compute(empty_string), Size { width: 0, height: 0, graphemes: 0 });
        assert_eq!(Size::compute(one_line_string), Size { width: 5, height: 1, graphemes: 5 });
        assert_eq!(Size::compute(two_lines_string), Size { width: 1, height: 2, graphemes: 3 });
        assert_eq!(
            Size::compute(different_width_of_lines_string),
            Size { width: 6, height: 2, graphemes: 11 }
        );
        assert_eq!(Size::compute(japan_string), Size { width: 22, height: 2, graphemes: 15 });
        assert_eq!(Size::compute(control_string), Size { width: 0, height: 2, graphemes: 3 });
        assert_eq!(Size::compute(russian_string), Size { width: 6, height: 2, graphemes: 11 });
        assert_eq!(
            Size::compute(multiple_lines_string),
            Size { width: 6, height: 6, graphemes: 21 }
        );
    }
}
