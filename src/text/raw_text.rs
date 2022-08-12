use std::{borrow::Cow, ops::RangeBounds};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::{Grapheme, GraphemeIter};

#[derive(Default, Clone)]
pub struct RawText {
    content: Cow<'static, str>,
    // Cached sizes for content
    size: RawTextSize,
}

#[derive(Default, Clone)]
struct RawTextSize {
    columns: usize,
    lines: usize,
}

impl RawText {
    pub fn new(content: Cow<'static, str>) -> Self {
        let size = Self::compute_size(content.as_ref());
        Self { content, size }
    }

    // Take content (transform it into string if it is borrowed)
    pub fn take(&mut self) -> String {
        self.size = RawTextSize::default();
        std::mem::take(self.content.to_mut())
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.to_mut().push_str(string);
        self.update_size();
    }

    pub fn split_off(&mut self, at: usize) -> RawText {
        let res = self.content.to_mut().split_off(at);
        self.update_size();
        RawText::new(res.into())
    }

    pub fn push(&mut self, c: char) {
        self.content.to_mut().push(c);
        self.update_size();
    }

    pub fn update_size(&mut self) {
        self.size = Self::compute_size(self.content());
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.content.to_mut().insert_str(idx, string);
        self.update_size();
    }

    pub fn replace_range<R>(&mut self, r: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        self.content.to_mut().replace_range(r, replace_with);
        self.update_size();
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

    pub fn truncate(&mut self, new_len: usize) {
        self.content.to_mut().truncate(new_len);
        self.update_size();
    }

    pub fn map<F>(&mut self, f: F)
    where
        F: FnOnce(&mut String),
    {
        f(self.content.to_mut());
        self.update_size();
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

    pub fn create_graphemes(s: &str) -> GraphemeIter<'_> {
        GraphemeIter::new(UnicodeSegmentation::grapheme_indices(s, true).enumerate())
    }
}

impl AsRef<str> for RawText {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
