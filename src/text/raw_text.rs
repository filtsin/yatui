use std::{borrow::Cow, ops::RangeBounds};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::Grapheme;

#[derive(Default)]
pub struct RawText {
    content: Cow<'static, str>,
    // Cached sizes for content
    size: RawTextSize,
}

#[derive(Default)]
struct RawTextSize {
    columns: usize,
    lines: usize,
}

impl RawText {
    pub fn new(content: Cow<'static, str>) -> Self {
        let size = Self::compute_size(content.as_ref());
        Self { content, size }
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.to_mut().push_str(string);
        // Maybe optimize this way. Find last line on older content and recalculate
        // size with new info from string
        self.size = Self::compute_size(self.content());
    }

    pub fn replace_range<R>(&mut self, r: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        self.content.to_mut().replace_range(r, replace_with);
        self.size = Self::compute_size(self.as_ref());
    }

    pub fn clear(&mut self) {
        self.content.to_mut().clear();
        self.size = RawTextSize::default();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.content.to_mut().reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.content.to_mut().reserve_exact(additional);
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        if let Cow::Owned(ref mut s) = self.content {
            s.shrink_to(min_capacity);
        }
    }

    pub fn shrink_to_fit(&mut self) {
        if let Cow::Owned(ref mut s) = self.content {
            s.shrink_to_fit();
        }
    }

    pub fn capacity(&self) -> usize {
        match &self.content {
            Cow::Borrowed(s) => 0,
            Cow::Owned(s) => s.capacity(),
        }
    }

    pub fn content(&self) -> &str {
        self.as_ref()
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.content, Cow::Borrowed(_))
    }

    pub fn is_owned(&self) -> bool {
        matches!(self.content, Cow::Owned(_))
    }

    pub fn columns(&self) -> usize {
        self.size.columns
    }

    pub fn lines(&self) -> usize {
        self.size.lines
    }

    fn compute_size(s: &str) -> RawTextSize {
        let mut columns = 0;
        let mut lines = 0;

        for line in s.lines() {
            lines += 1;

            let width = UnicodeWidthStr::width(line);
            columns = columns.max(width);
        }

        RawTextSize { columns, lines }
    }

    pub fn create_graphemes(s: &str) -> impl Iterator<Item = Grapheme<'_>> + Clone {
        UnicodeSegmentation::grapheme_indices(s, true).enumerate().map(Grapheme::new)
    }
}

impl AsRef<str> for RawText {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
