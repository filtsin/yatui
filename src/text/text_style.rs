use super::Grapheme;
use crate::text::{utils::bound_to_range, Style};

use std::{
    cmp::{Eq, PartialEq},
    collections::{btree_set::Iter as BIter, BTreeSet},
    ops::{
        Add, AddAssign,
        Bound::{self, Excluded, Included},
        Range, RangeBounds, RangeInclusive, Sub, SubAssign,
    },
};

pub type StyleInfo = (RangeInclusive<usize>, Style);

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TextStyle {
    data: BTreeSet<RangeStyle>,
}

#[derive(Debug, Clone, Eq)]
struct RangeStyle {
    info: StyleInfo,
}

#[derive(Clone)]
pub struct Iter<'a> {
    inner: BIter<'a, RangeStyle>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add style for the `range`.
    ///
    /// There are no checks for grapheme offsets on target Text.
    /// This allows to create styles for content that doesn't exist yes.
    /// If `range` already exists, replace styles on it.
    ///
    /// # Panics
    ///
    /// Panics if start > end
    ///
    /// # Examples
    ///
    /// ```
    /// use yatui::text::{Color, Style, TextStyle};
    ///
    /// let mut style = TextStyle::new();
    /// style.add(0..=1, Style::new().bg(Color::Red));
    /// style.add(0..=1, Style::new().bg(Color::Yellow));
    ///
    /// assert_eq!(style.into_vec(), vec![(0..=1, Style::new().bg(Color::Yellow))]);
    /// ```
    pub fn add<R: RangeBounds<usize>>(&mut self, range: R, style: Style) {
        self.data.replace(RangeStyle::new(range, style));
    }

    /// Remove style with byte offsets.
    ///
    /// It removes only full match of `range`. If `range` is subset ob bigger range then
    /// there will be no changes. Possibly you want [`remove`](Self::remove).
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut style = TextStyle::new();
    ///
    /// style.add(0..=3, Style::new().bg(Color::Red));
    /// style.add(1..=2, Style::new().fg(Color::Red));
    /// style.remove_range(0..=3);
    ///
    /// assert_eq!(style.into_vec(), vec![(1..=2, Style::new().fg(Color::Red))]);
    /// ```
    pub fn remove_range<R: RangeBounds<usize>>(&mut self, range: R) {
        self.data.remove(&range.into());
    }

    /// Remove all styles for the `range`.
    ///
    /// Unlike [`remove_range`][Self::remove_range] it removes all intersections
    /// with `range` of bigger ranges. Possibly you want use it for style clearing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut style = TextStyle::new();
    ///
    /// style.add(0..=3, Style::new().bg(Color::Red));
    /// style.add(1..=2, Style::new().fg(Color::Red));
    /// style.remove(0..=3);
    ///
    /// assert_eq!(style.into_vec(), vec![]);
    /// ```
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut style = TextStyle::new();
    ///
    /// style.add(0..=3, Style::new().bg(Color::Red));
    /// style.add(1..=2, Style::new().fg(Color::Red));
    /// style.remove(1..2);
    ///
    /// assert_eq!(
    ///     style.into_vec(),
    ///     vec![
    ///         (0..=0, Style::new().bg(Color::Red)),
    ///         (2..=2, Style::new().fg(Color::Red)),
    ///         (2..=3, Style::new().bg(Color::Red))
    ///     ]
    /// );
    /// ```
    pub fn remove<R: RangeBounds<usize>>(&mut self, range: R) {
        let range = bound_to_range(range);
        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .flat_map(|style| style.cut(range.clone()))
            .flatten()
            .collect();
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.data.iter())
    }

    pub fn into_vec(self) -> Vec<StyleInfo> {
        self.iter().collect()
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

    pub(crate) fn positive_shift(&mut self, start_from: usize, delta: usize) {
        if delta == 0 {
            return;
        }

        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .map(|mut range| {
                if range.start() >= start_from {
                    range += delta;
                }
                range
            })
            .collect();
    }

    pub(crate) fn negative_shift(&mut self, start_from: usize, delta: usize) {
        if delta == 0 {
            return;
        }

        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .map(|mut range| {
                if range.start() >= start_from {
                    range -= delta;
                }
                range
            })
            .collect();
    }
}

impl<'a> Iter<'a> {
    fn new(inner: BIter<'a, RangeStyle>) -> Self {
        Self { inner }
    }
}

impl Iterator for Iter<'_> {
    type Item = StyleInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|v| v.info.clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    fn max(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|v| v.info.clone())
    }
}

impl RangeStyle {
    fn new<R: RangeBounds<usize>>(range: R, style: Style) -> Self {
        Self { info: (bound_to_range(range), style) }
    }

    fn only_with_range<R: RangeBounds<usize>>(range: R) -> Self {
        Self::new(range, Style::default())
    }

    fn cut<R: RangeBounds<usize>>(&self, range: R) -> [Option<RangeStyle>; 2] {
        let range = bound_to_range(range);

        let start = *range.start();
        let end = *range.end();

        if start > self.end() || end < self.start() {
            return [Some(self.clone()), None];
        }

        let start = if start < self.start() { self.start() } else { start };
        let end = if end > self.end() { self.end() } else { end };

        let left = if start != self.start() {
            Some(Self::new(self.start()..start, self.style()))
        } else {
            None
        };

        let right = if end != self.end() {
            Some(Self::new(end + 1..=self.end(), self.style()))
        } else {
            None
        };

        [left, right]
    }

    fn start(&self) -> usize {
        *self.range().start()
    }

    fn end(&self) -> usize {
        *self.range().end()
    }

    fn range(&self) -> RangeInclusive<usize> {
        self.info.0.clone()
    }

    fn style(&self) -> Style {
        self.info.1
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<StyleInfo>> for TextStyle {
    fn into(self) -> Vec<StyleInfo> {
        self.into_vec()
    }
}

impl PartialEq for RangeStyle {
    fn eq(&self, other: &Self) -> bool {
        self.range().eq(other.range())
    }
}

impl std::hash::Hash for RangeStyle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.0.hash(state);
    }
}

impl Ord for RangeStyle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range().cmp(other.range())
    }
}

impl PartialOrd for RangeStyle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range().partial_cmp(other.range())
    }
}

impl<R: RangeBounds<usize>> From<R> for RangeStyle {
    fn from(range: R) -> Self {
        Self::only_with_range(range)
    }
}

impl Add<usize> for RangeStyle {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<usize> for RangeStyle {
    fn add_assign(&mut self, rhs: usize) {
        let start = self.start().saturating_add(rhs);
        let end = self.end().saturating_add(rhs);
        self.info.0 = start..=end;
    }
}

impl Sub<usize> for RangeStyle {
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<usize> for RangeStyle {
    fn sub_assign(&mut self, rhs: usize) {
        let start = self.start().saturating_sub(rhs);
        let end = self.end().saturating_sub(rhs);
        self.info.0 = start..=end;
    }
}

#[cfg(test)]
mod tests {
    use super::RangeStyle;

    #[test]
    fn cut() {
        let mut s: RangeStyle = (0..=4).into();

        let [left, right] = s.cut(1..=2);
        assert_eq!(left, Some((0..=0).into()));
        assert_eq!(right, Some((3..=4).into()));

        let [left, right] = s.cut(0..=1);
        assert_eq!(left, None);
        assert_eq!(right, Some((2..=4).into()));

        let [left, right] = s.cut(3..=4);
        assert_eq!(left, Some((0..=2).into()));
        assert_eq!(right, None);

        let mut s: RangeStyle = (1..=2).into();

        let [left, right] = s.cut(1..=2);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let [left, right] = s.cut(0..=4);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let mut s: RangeStyle = (1..=1).into();

        let [left, right] = s.cut(3..=5);
        assert_eq!(left, Some((1..=1).into()));
        assert_eq!(right, None);

        let [left, right] = s.cut(0..=0);
        assert_eq!(left, Some((1..=1).into()));
        assert_eq!(right, None);
    }
}
