mod grapheme;
mod raw_text;
mod style;
mod text_style;

pub use grapheme::Grapheme;
pub use style::{Color, Modifier, Style};
pub use text_style::TextStyle;

use raw_text::RawText;

use std::{
    borrow::Cow,
    collections::BTreeSet,
    ops::{Range, RangeBounds, RangeTo},
};

#[derive(Default)]
pub struct Text {
    raw: RawText,
    style: TextStyle,
}

impl Text {
    pub fn new<C>(content: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { raw: RawText::new(content.into()), ..Self::default() }
    }

    pub fn parts(&mut self) -> (impl Iterator<Item = Grapheme<'_>>, &'_ mut TextStyle) {
        (RawText::create_graphemes(self.raw.as_ref()), &mut self.style)
    }

    pub fn push_str(&mut self, string: &str) {
        self.raw.push_str(string);
    }

    pub fn remove(&mut self, grapheme_idx: usize) {
        self.replace_range(grapheme_idx..=grapheme_idx, "");
    }

    pub fn replace_range<R>(&mut self, r: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        let range = range(r, ..self.raw.count());

        let g1 = RawText::create_graphemes(self.raw.as_ref()).nth(range.start).unwrap().info();
        let g2 = RawText::create_graphemes(self.raw.as_ref()).nth(range.end - 1).unwrap().info();

        self.raw.replace_range(g1.start()..=g2.end(), replace_with);
        self.style.remove(g1.start(), g2.end());

        let old_len = g2.distance_to(g1);
        let new_len = replace_with.len();

        if old_len > new_len {
            self.styles_mut().negative_shift(g2.end(), old_len - new_len);
        } else {
            self.styles_mut().positive_shift(g2.end(), new_len - old_len);
        }
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.style.clear()
    }

    pub fn styles(&self) -> &TextStyle {
        &self.style
    }

    pub fn styles_mut(&mut self) -> &mut TextStyle {
        &mut self.style
    }

    pub fn lines(&self) -> usize {
        self.raw.lines()
    }

    pub fn columns(&self) -> usize {
        self.raw.columns()
    }

    pub fn is_borrowed(&self) -> bool {
        self.raw.is_borrowed()
    }

    pub fn is_owned(&self) -> bool {
        self.raw.is_owned()
    }
}

// TODO: Replace with std::slice::range when stabilized
fn range<R>(range: R, bounds: RangeTo<usize>) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        std::ops::Bound::Included(&n) => n,
        std::ops::Bound::Excluded(&n) => n.checked_add(1).unwrap(),
        std::ops::Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        std::ops::Bound::Included(&n) => n.checked_add(1).unwrap(),
        std::ops::Bound::Excluded(&n) => n,
        std::ops::Bound::Unbounded => bounds.end,
    };

    assert!(start < end);
    assert!(end <= bounds.end);

    start..end
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.raw.as_ref()
    }
}

impl From<&'static str> for Text {
    fn from(s: &'static str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
