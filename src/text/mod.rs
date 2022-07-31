pub mod graphemes;
mod style;

use self::graphemes::Grapheme;
pub use self::style::{Color, Modifier, Style};

use unicode_segmentation::UnicodeSegmentation;

use std::{
    borrow::Cow,
    cmp::{Ord, PartialOrd},
    collections::BTreeSet,
    ops::{RangeBounds, RangeInclusive},
};

#[derive(Default, Clone)]
pub struct Text {
    data: RawText,
    styles: Styles,
}

#[derive(Default, Clone)]
pub struct Styles {
    data: BTreeSet<RangeStyle>,
}

#[derive(Default, Clone)]
struct RawText {
    content: Cow<'static, str>,
    // length in unicode graphemes
    length: usize,
}

#[derive(Debug, Clone, Eq)]
struct RangeStyle {
    // range in bytes offset of content
    range: RangeInclusive<usize>,
    style: Style,
}

impl Text {
    pub fn new<C>(content: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { data: RawText::new(content.into()), ..Default::default() }
    }

    pub fn parts(&mut self) -> (impl Iterator<Item = Grapheme<'_>>, &'_ mut Styles) {
        let g = UnicodeSegmentation::grapheme_indices(self.data.content(), true).map(Grapheme::new);
        (g, &mut self.styles)
    }

    pub fn clear(&mut self) {
        self.styles_mut().clear();
        self.raw_mut().update_content("".into());
    }

    /// Replace *graphemes*
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        //
    }

    pub fn push_str(&mut self, string: &str) {
        self.raw_mut().push_str(string);
    }

    /// Remove *grapheme* from this Text
    pub fn remove(&mut self, grapheme_idx: usize) {
        //
    }

    pub fn styles(&self) -> &Styles {
        &self.styles
    }

    pub fn styles_mut(&mut self) -> &mut Styles {
        &mut self.styles
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

    pub fn as_str(&self) -> &str {
        self.raw().content()
    }

    fn raw(&self) -> &RawText {
        &self.data
    }

    fn raw_mut(&mut self) -> &mut RawText {
        &mut self.data
    }
}

impl Styles {
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, Style)> + '_ {
        self.data.iter().map(|v| (*v.range.start(), *v.range.end(), v.style))
    }

    pub fn add_style_raw(&mut self, from: usize, to: usize, style: Style) {
        assert!(from <= to);

        self.data.replace(RangeStyle::new(from..=to, style));
    }

    pub fn add_style(&mut self, from: &Grapheme<'_>, to: &Grapheme<'_>, style: Style) {
        self.add_style_raw(from.index, to.index + to.data().len() - 1, style);
    }

    pub fn remove_full(&mut self, from: usize, to: usize) {
        self.data.remove(&RangeStyle::from(from..=to));
    }

    pub fn remove(&mut self, from: usize, to: usize) {
        let mut copy = BTreeSet::new();

        std::mem::swap(&mut self.data, &mut copy);

        for style in copy.into_iter() {
            let (left, right) = style.cut(from, to);

            if let Some(left) = left {
                self.data.insert(left);
            }
            if let Some(right) = right {
                self.data.insert(right);
            }
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    fn push_str(&mut self, string: &str) {
        let len = Self::compute_length(string);

        self.content.to_mut().push_str(string);
        self.length += len;
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
    fn new(range: RangeInclusive<usize>, style: Style) -> Self {
        Self { range, style }
    }

    fn cut(&self, from: usize, to: usize) -> (Option<RangeStyle>, Option<RangeStyle>) {
        assert!(from <= to);

        let from = if from < self.start() { self.start() } else { from };
        let to = if to > self.end() { self.end() } else { to };

        let left = if from != self.start() { Some(self.start()..=from - 1) } else { None };
        let right = if to != self.end() { Some(to + 1..=self.end()) } else { None };

        (
            left.map(|range| RangeStyle::new(range, self.style)),
            right.map(|range| RangeStyle::new(range, self.style)),
        )
    }

    fn contains(&self, from: usize, to: usize) -> bool {
        from >= self.start() || to <= self.end()
    }

    fn start(&self) -> usize {
        *self.range.start()
    }

    fn end(&self) -> usize {
        *self.range.end()
    }
}

impl From<RangeInclusive<usize>> for RangeStyle {
    fn from(range: RangeInclusive<usize>) -> Self {
        Self::new(range, Style::default())
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

#[cfg(test)]
mod tests {
    use super::RangeStyle;

    #[test]
    fn cut() {
        let mut s: RangeStyle = (0..=4).into();

        let (left, right) = s.cut(1, 2);
        assert_eq!(left, Some((0..=0).into()));
        assert_eq!(right, Some((3..=4).into()));

        let (left, right) = s.cut(0, 1);
        assert_eq!(left, None);
        assert_eq!(right, Some((2..=4).into()));

        let (left, right) = s.cut(3, 4);
        assert_eq!(left, Some((0..=2).into()));
        assert_eq!(right, None);

        let mut s: RangeStyle = (2..=2).into();

        let (left, right) = s.cut(1, 2);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let (left, right) = s.cut(0, 4);
        assert_eq!(left, None);
        assert_eq!(right, None);
    }
}
