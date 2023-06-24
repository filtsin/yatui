use self::range::Range;
use super::Style;

use std::{
    collections::{
        btree_map::{Iter as BIter, Range as BRange},
        BTreeMap,
    },
    iter::FusedIterator,
    ops::{
        Bound::{self, Included, Unbounded},
        RangeInclusive,
    },
};

pub mod range;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Mask {
    data: BTreeMap<Range, Style>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct StyleInfo {
    pub range: Range,
    pub style: Style,
}

/// An iterator over the items of `Mask`.
///
/// This struct is created by the [`iter`](Mask::iter) method on [`Mask`](Mask).
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Iter<'a> {
    inner: BIter<'a, Range, Style>,
}

/// An iterator over the items of `Mask`.
///
/// This struct is created by the [`range`](Mask::range) method on [`Mask`](Mask).
pub struct RangeIter<'a> {
    inner: BRange<'a, Range, Style>,
    start: usize,
    end: usize,
}

impl Mask {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add style for the `range`.
    ///
    /// There are no checks for grapheme offsets on target Text.
    /// This allows to create styles for content that doesn't exists yet.
    /// It merges all styles for overlaping ranges.
    ///
    /// # Panics
    ///
    /// Panics if start > end
    ///
    /// # Examples
    ///
    /// ```
    /// use yatui::text::{Color, Mask, Style};
    ///
    /// let mut mask = Mask::new();
    /// mask.add(0..=1, Style::new().fg(Color::Red));
    /// mask.add(0..=1, Style::new().fg(Color::Yellow));
    ///
    /// assert_eq!(mask.into_vec(), vec![(0..=1, Style::new().fg(Color::Yellow))]);
    /// ```
    pub fn add<R: Into<Range>>(&mut self, range: R, style: Style) {
        self.add_inner(range, style);
    }
    ///
    /// Remove style with byte offsets.
    ///
    /// It removes only full match of `range`. If `range` is subset ob bigger range then
    /// there will be no changes. Possibly you want [`remove`](Self::remove).
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut mask = Mask::new();
    ///
    /// mask.add(0..=3, Style::new().bg(Color::Red));
    /// mask.add(4..=6, Style::new().fg(Color::Red));
    /// mask.remove_range(0..=3);
    ///
    /// assert_eq!(mask.into_vec(), vec![(4..=6, Style::new().fg(Color::Red))]);
    /// ```
    pub fn remove_range<R: Into<Range>>(&mut self, range: R) {
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
    /// let mut mask = Mask::new();
    ///
    /// mask.add(0..=3, Style::new().bg(Color::Red));
    /// mask.add(1..=2, Style::new().fg(Color::Red));
    /// mask.remove(0..=3);
    ///
    /// assert_eq!(mask.into_vec(), vec![]);
    /// ```
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut mask = Mask::new();
    ///
    /// mask.add(0..=3, Style::new().bg(Color::Red));
    /// mask.add(4..=6, Style::new().fg(Color::Red));
    /// mask.remove(1..2);
    ///
    /// assert_eq!(
    ///     mask.into_vec(),
    ///     vec![
    ///         (0..=0, Style::new().bg(Color::Red)),
    ///         (2..=3, Style::new().bg(Color::Red)),
    ///         (4..=6, Style::new().fg(Color::Red))
    ///     ]
    /// );
    /// ```
    pub fn remove<R: Into<Range>>(&mut self, range: R) {
        let range = range.into();
        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .flat_map(|(r, style)| {
                let [left, right] = r.cut(range);
                [left.map(|left| (left, style)), right.map(|right| (right, style))]
            })
            .flatten()
            .collect();
    }

    /// Gets an iterator that visits the elements in the `Mask` in ascending order.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.data.iter())
    }

    /// Gets an iterator over a sub-range of styles. Iterator will yield all styles for specified
    /// `range`.
    pub fn range<R: Into<Range>>(&self, range: R) -> RangeIter<'_> {
        let range = range.into();
        let end: Range = (range.end..).into();

        let inner = self.data.range((Unbounded, Included(end)));
        RangeIter::new(inner, range.start, range.end)
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

    // TODO: Update doc, maybe hide from public api
    /// Shift all existing styles in `range` with `delta` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut mask = Mask::new();
    /// mask.add(0..=3, Style::new().bg(Color::Green));
    /// mask.add(4..=6, Style::new().fg(Color::Red));
    ///
    /// mask.shift_add(3..=3, 2);
    ///
    /// assert_eq!(
    ///     mask.into_vec(),
    ///     vec![
    ///         (0..=2, Style::new().bg(Color::Green)),
    ///         (4..=4, Style::new().fg(Color::Red)),
    ///         (5..=5, Style::new().fg(Color::Red).bg(Color::Green)),
    ///         (6..=6, Style::new().fg(Color::Red))
    ///     ]
    /// );
    /// ```
    pub fn shift_add<R: Into<Range>>(&mut self, range: R, delta: usize) {
        let range = range.into();
        let mut styles: Vec<_> = self.range(range).collect();

        for (StyleInfo { range, .. }) in &mut styles {
            self.remove(*range);
            *range += delta;
        }

        self.extend(styles);
    }

    // TODO: Update doc, maybe hide from public api
    /// Shift all existing styles in `range` with `delta` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut mask = Mask::new();
    /// mask.add(0..=3, Style::new().bg(Color::Green));
    /// mask.add(5..=8, Style::new().fg(Color::Red));
    ///
    /// mask.shift_sub(5..=5, 2);
    ///
    /// assert_eq!(
    ///     mask.into_vec(),
    ///     vec![
    ///         (0..=2, Style::new().bg(Color::Green)),
    ///         (3..=3, Style::new().bg(Color::Green).fg(Color::Red)),
    ///         (6..=8, Style::new().fg(Color::Red))
    ///     ]
    /// );
    /// ```
    pub fn shift_sub<R: Into<Range>>(&mut self, range: R, delta: usize) {
        let range = range.into();
        let mut styles: Vec<_> = self.range(range).collect();

        for (StyleInfo { range, .. }) in &mut styles {
            self.remove(*range);
            *range -= delta;
        }

        self.extend(styles);
    }

    // Variants:
    //
    // Top range is `range` from argument
    // Bottom range is range from current map
    //
    // i. x' < x && y' >= y
    //
    //       ─────────────
    //       x           y
    // ────────────────────────
    // x'                     y'
    //
    // Split [x', y'] to 3 ranges:
    //      - [x', x): With existing styles
    //      - [x, y]: Merge existing styles and `style`
    //      - (y, y']: With existing styles
    // Stop, because no one existing range can overlap `range`.
    //
    //
    // ii. x' > x && y' >= y
    //
    //       ─────────────
    //       x           y
    //          ────────────────────────
    //          x'                     y'
    //
    // Split [x', y'] to 3 ranges:
    //      - [x, x'): With `style`
    //      - [x', y]: Merge existing styles and `style`
    //      - (y, y'): With existing styles
    // Stop, because no one existing range can overlap `range`.
    //
    // iii. x' > x && y ' < y
    //
    //       ─────────────
    //       x           y
    //          ───────
    //          x'    y'
    //
    // Split [x, y] to 3 ranges:
    //      - [x, x'): With `style`
    //      - [x', y']: Merge existing styles and `style`
    //      - (y', y]: Do not add now, replace `range` with it and do next iteration
    //
    // iv. x' < x && y' < y
    //
    //       ─────────────
    //       x           y
    //  ───────
    //  x'    y'
    //
    //  Split [x', y'] to 3 ranges:
    //      - [x', x): With existing styles
    //      - [x, y']: Merge existing styles and `style`
    //      - (y', y]: Do not add now, replace `range` with it and do next iteration
    //
    // v. x = x', y' >= y
    //
    //       ─────────────
    //       x           y
    //       ────────────────────────
    //       x'                     y'
    //
    //  Split [x', y'] to 2 ranges:
    //      - [x', y]: Merge existing styles and `style`
    //      - (y, y']: Existing styles
    // Stop, because no one existing range can overlap `range`.
    //
    // vi. x = x', y' < y
    //       ─────────────
    //       x           y
    //       ─────────
    //       x'      y'
    //
    //  Split [x', y'] to 2 ranges:
    //      - [x', y]: Merge existing styles and `style`
    //      - (y, y']: Do not add now, replace `range` with it and do next iteration
    fn add_inner<R: Into<Range>>(&mut self, range: R, style: Style) {
        let mut range = range.into();
        let end_range: Range = (range.end..).into();

        let mut previous: Bound<Range> = Unbounded;

        while let Some((cur_range, cur_styles)) = self
            .data
            .range((previous, Included(end_range)))
            .find(|(r, _)| r.intersects_with(range))
            .map(|(range, styles)| (*range, *styles))
        {
            // If ranges are equal do not erase and insert it again, just modify styles in place
            if cur_range == range {
                self.data.get_mut(&cur_range).unwrap().merge(style);
                return;
            }

            self.data.remove(&cur_range);

            let intersection = cur_range.intersection(range).unwrap();
            self.data.insert(intersection, Style::new().merge(cur_styles).merge(style));

            let [left, middle, right] = cur_range.split_to_3(range);
            let left = left.unwrap();
            let middle = middle.unwrap();

            if left != intersection {
                let style = if cur_range.intersects_with(left) { cur_styles } else { style };
                self.data.insert(left, style);
            }

            // Only v and vi variants
            if middle != intersection {
                if cur_range.intersects_with(middle) {
                    self.data.insert(middle, cur_styles);
                    return;
                } else {
                    previous = Included((middle.start..=middle.start).into());
                    range = middle;
                    continue;
                }
            }

            if let Some(right) = right {
                if range.intersects_with(right) {
                    previous = Included((right.start..=right.start).into());
                    range = right;
                    continue;
                } else {
                    self.data.insert(right, cur_styles);
                    return;
                }
            } else {
                return;
            }
        }

        self.data.insert(range, style);
    }
}

impl Extend<StyleInfo> for Mask {
    fn extend<T: IntoIterator<Item = StyleInfo>>(&mut self, iter: T) {
        for (StyleInfo { range, style }) in iter.into_iter() {
            self.add(range, style);
        }
    }
}

impl StyleInfo {
    pub fn new<R: Into<Range>>(range: R, style: Style) -> Self {
        Self { range: range.into(), style }
    }
}

impl PartialEq<(RangeInclusive<usize>, Style)> for StyleInfo {
    fn eq(&self, (range, style): &(RangeInclusive<usize>, Style)) -> bool {
        self.range == *range && self.style == *style
    }
}

impl<R: Into<Range>> From<(R, Style)> for StyleInfo {
    fn from((range, style): (R, Style)) -> Self {
        Self::new(range, style)
    }
}

impl<'a> Iter<'a> {
    fn new(inner: BIter<'a, Range, Style>) -> Self {
        Self { inner }
    }
}

impl<'a> RangeIter<'a> {
    fn new(inner: BRange<'a, Range, Style>, start: usize, end: usize) -> Self {
        Self { inner, start, end }
    }

    fn process_inner(&self, range: &Range, style: &Style) -> StyleInfo {
        let start = if range.start < self.start { self.start } else { range.start };
        let end = if range.end > self.end { self.end } else { range.end };

        StyleInfo::new(start..=end, *style)
    }
}

impl Iterator for Iter<'_> {
    type Item = StyleInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(r, style)| (*r, *style).into())
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
        self.inner.next_back().map(|(r, style)| (*r, *style).into())
    }
}

impl Iterator for RangeIter<'_> {
    type Item = StyleInfo;

    fn next(&mut self) -> Option<Self::Item> {
        for (r, style) in self.inner.by_ref() {
            if r.intersects_with(self.start..=self.end) {
                return Some(self.process_inner(r, style));
            }
        }
        None
    }
}

impl DoubleEndedIterator for RangeIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((r, style)) = self.inner.next_back() {
            if r.intersects_with(self.start..=self.end) {
                return Some(self.process_inner(r, style));
            }
        }
        None
    }
}

impl FusedIterator for RangeIter<'_> {}
impl FusedIterator for Iter<'_> {}
