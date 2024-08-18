use std::{borrow::Cow, cell::Cell};

use unicode_width::UnicodeWidthStr;

/// Wrapper about string.
/// The only responsobility of this struct is cache size of string and invalidate it when string
/// is changed.
#[derive(Debug, Default)]
pub(super) struct Raw {
    data: Cow<'static, str>,
    size: Cell<Option<Size>>,
}

/// Cached size for `Raw` in terminal way
///
/// You should know:
///
/// 1) Not every grapheme have width=1. For example, cjk symbols: "す" have width=2.
///     This means that you need two terminal columns to display it.
///
/// 2) `Height` is count of lines in terminal way. Line delimiter is "\n" or "\r\n".
///
/// ```text
/// 1 - 1 line
/// \n - 1 line too (empty)
/// 1\n2 - 2 lines
/// 1\n2\r\n3 - 3 lines
/// 1\n2\r\n3\n - 4 lines
/// ```
#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Size {
    /// Width of `text` in terminal way.
    /// It is the largest line in the `text`.
    width: usize,
    /// Height of `text` in terminal way.
    /// It is count of lines in the `text`.
    height: usize,
}

impl Raw {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        self.invalidate_size_cache();
        f(self.data.to_mut())
    }

    pub(super) fn width(&self) -> usize {
        self.get_size().width
    }

    pub(super) fn height(&self) -> usize {
        self.get_size().height
    }

    fn invalidate_size_cache(&self) {
        self.size.set(None);
    }

    /// Get current cached size. If cache was invalidated compute and cache new size.
    fn get_size(&self) -> Size {
        match self.size.get() {
            Some(s) => s,
            None => {
                let size = self.as_ref().into();
                self.size.set(Some(size));
                size
            }
        }
    }
}

impl AsRef<str> for Raw {
    fn as_ref(&self) -> &str {
        self.data.as_ref()
    }
}

impl<'a> From<&'a str> for Size {
    fn from(s: &'a str) -> Self {
        let mut size = s.split_inclusive('\n').fold(Size::default(), |mut size, line| {
            let width = UnicodeWidthStr::width(line);
            size.width = size.width.max(width);
            size.height += 1;
            size
        });
        if s.ends_with('\n') || s.ends_with("\r\n") {
            size.height += 1;
        }
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn modify_invalidate_raw_cache() {
        let mut raw =
            Raw { data: Cow::Borrowed("hello"), size: Cell::new(Some(Size::from("hello"))) };

        raw.modify(|_| {});

        assert_eq!(raw.size.get(), None);
    }

    #[rstest]
    #[case::empty("", Size { width: 0, height: 0 })]
    #[case::empty_newline("\n", Size { width: 0, height: 2 })]
    #[case::ascii_line("hello", Size { width: 5, height: 1 })]
    #[case::ascii_two_lines("hello\nbiggest", Size { width: 7, height: 2 })]
    #[case::japan_line("老", Size { width: 2, height: 1 })]
    #[case::japan_two_lines("老老虎\nこれは単なるテストです", Size { width: 22, height: 2 })]
    #[case::russian_line("привет", Size { width: 6, height: 1 })]
    #[case::russian_lines("большая\nстрока", Size { width: 7, height: 2 })]
    // it is Latin Small Letter Y with Acute "ý"
    #[case::one_unicode_point("\u{00fd}", Size { width: 1, height: 1 })]
    // it is Latin Small Letter Y with Combinin Acute Accent "◌́"
    #[case::two_unicode_points("y\u{0301}", Size { width: 1, height: 1 })]
    #[case::constrol_symbols("\n\t\r\n", Size { width: 0, height: 3 })]
    fn compute_size(#[case] string: &str, #[case] expected: Size) {
        let actual = Size::from(string);
        assert_eq!(actual, expected);
    }
}
