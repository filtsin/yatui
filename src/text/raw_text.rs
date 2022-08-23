use super::{Grapheme, GraphemeIter};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use std::{
    borrow::Cow,
    cmp::{Eq, PartialEq},
    ops::RangeBounds,
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RawText {
    content: Cow<'static, str>,
    size: RawTextSize,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct RawTextSize {
    pub(super) columns: usize,
    pub(super) lines: usize,
}

impl RawText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        let result = f(self.content.to_mut());
        self.update_size();
        result
    }

    pub fn update_size(&mut self) {
        self.size = Self::compute_size(self.as_str());
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

    pub fn as_str(&self) -> &str {
        &self.content
    }

    pub(crate) fn compute_size(s: &str) -> RawTextSize {
        let mut columns = 0;
        let mut lines = 0;

        for line in s.lines() {
            lines += 1;

            let width = UnicodeWidthStr::width(line);
            columns = columns.max(width);
        }

        RawTextSize { columns, lines }
    }

    pub fn create_graphemes(s: &str) -> GraphemeIter<'_> {
        GraphemeIter::new(UnicodeSegmentation::grapheme_indices(s, true).enumerate())
    }
}

impl AsRef<str> for RawText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<C: Into<Cow<'static, str>>> From<C> for RawText {
    fn from(s: C) -> Self {
        let mut result = Self::new();
        result.content = s.into();
        result.update_size();
        result
    }
}
