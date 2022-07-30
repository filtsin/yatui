pub mod graphemes;
mod style;

use self::graphemes::{Grapheme, Graphemes};
pub use self::style::{Color, Modifier, Style};

use unicode_segmentation::UnicodeSegmentation;

use std::{
    borrow::Cow,
    cmp::{Ord, PartialOrd},
    collections::BinaryHeap,
    ops::Range,
};

#[derive(Default, Clone)]
pub struct Text {
    data: RawText,
    styles: Styles,
}

#[derive(Default, Clone)]
pub struct Styles {
    data: BinaryHeap<RangeStyle>,
}

#[derive(Default, Clone)]
struct RawText {
    content: Cow<'static, str>,
    // length in unicode graphemes
    length: usize,
}

#[derive(Clone, Eq)]
struct RangeStyle {
    // range in bytes offset of content
    range: Range<usize>,
    style: Style,
}

impl Text {
    pub fn new<C>(content: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { data: RawText::new(content.into()), ..Default::default() }
    }

    pub fn parts(&mut self) -> (Graphemes<'_>, &'_ mut Styles) {
        let g = UnicodeSegmentation::grapheme_indices(self.data.content(), true)
            .map(Grapheme::new)
            .collect();

        (Graphemes::new(g), &mut self.styles)
    }

    pub fn add_style(&mut self, from: usize, to: usize, style: Style) {
        self.styles.add_style_raw(from, to, style);
    }

    pub fn is_borrowed(&self) -> bool {
        self.raw().is_borrowed()
    }

    pub fn is_owned(&self) -> bool {
        self.raw().is_owned()
    }

    pub fn len(&self) -> usize {
        self.raw().length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn raw(&self) -> &RawText {
        &self.data
    }

    fn raw_mut(&mut self) -> &mut RawText {
        &mut self.data
    }
}

impl Styles {
    pub fn add_style_raw(&mut self, from: usize, to: usize, style: Style) {
        assert!(from < to);
        assert!(from < self.data.len() && to <= self.data.len());

        self.data.push(RangeStyle::new(from..to, style));
    }

    pub fn add_style(&mut self, from: &Grapheme<'_>, to: &Grapheme<'_>, style: Style) {
        self.add_style_raw(from.index, to.index, style);
    }
}

impl RawText {
    fn new(content: Cow<'static, str>) -> Self {
        let mut result = Self::default();
        result.update_content(content);
        result
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn update_content(&mut self, content: Cow<'static, str>) {
        let len = Self::compute_length(&content);
        self.content = content;
        self.length = len;
    }

    fn compute_length(s: &str) -> usize {
        UnicodeSegmentation::graphemes(s, true).count()
    }

    fn is_borrowed(&self) -> bool {
        matches!(self.content, Cow::Borrowed(_))
    }

    fn is_owned(&self) -> bool {
        matches!(self.content, Cow::Owned(_))
    }
}

impl RangeStyle {
    fn new(range: Range<usize>, style: Style) -> Self {
        Self { range, style }
    }
}

impl<C> From<C> for Text
where
    C: Into<Cow<'static, str>>,
{
    fn from(v: C) -> Self {
        Self::new(v)
    }
}

impl PartialEq for RangeStyle {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl Ord for RangeStyle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.clone().cmp(other.range.clone())
    }
}

impl PartialOrd for RangeStyle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.clone().partial_cmp(other.range.clone())
    }
}
