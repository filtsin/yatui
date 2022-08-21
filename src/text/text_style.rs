use super::Grapheme;
use crate::text::{utils::bound_to_range, Style};

use std::{
    cmp::{Eq, PartialEq},
    collections::{btree_map::Iter as BIter, BTreeMap},
    hash::Hash,
    ops::{
        Add, AddAssign,
        Bound::{self, Excluded, Included, Unbounded},
        RangeBounds, RangeInclusive, Sub, SubAssign,
    },
};

pub type StyleInfo = (RangeInclusive<usize>, Style);

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TextStyle {
    data: BTreeMap<RangeWrapper, Style>,
}

#[derive(Debug, Clone, Eq)]
pub(crate) struct RangeWrapper {
    range: RangeInclusive<usize>,
}

/// An iterator over the items of `TextStyle`.
///
/// This struct is created by the [`iter`](TextStyle::iter) method on [`TextStyle`](TextStyle).
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Iter<'a> {
    inner: BIter<'a, RangeWrapper, Style>,
}

impl TextStyle {
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
    /// use yatui::text::{Color, Style, TextStyle};
    ///
    /// let mut style = TextStyle::new();
    /// style.add(0..=1, Style::new().fg(Color::Red));
    /// style.add(0..=1, Style::new().fg(Color::Yellow));
    ///
    /// assert_eq!(style.into_vec(), vec![(0..=1, Style::new().fg(Color::Yellow))]);
    /// ```
    pub fn add<R: RangeBounds<usize>>(&mut self, range: R, style: Style) {
        self.add_inner(range, style);
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
    /// style.add(4..=6, Style::new().fg(Color::Red));
    /// style.remove_range(0..=3);
    ///
    /// assert_eq!(style.into_vec(), vec![(4..=6, Style::new().fg(Color::Red))]);
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
    /// style.add(4..=6, Style::new().fg(Color::Red));
    /// style.remove(1..2);
    ///
    /// assert_eq!(
    ///     style.into_vec(),
    ///     vec![
    ///         (0..=0, Style::new().bg(Color::Red)),
    ///         (2..=3, Style::new().bg(Color::Red)),
    ///         (4..=6, Style::new().fg(Color::Red))
    ///     ]
    /// );
    /// ```
    pub fn remove<R: RangeBounds<usize>>(&mut self, range: R) {
        let range = bound_to_range(range);
        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .flat_map(|(r, style)| {
                let [left, right] = r.cut(range.clone());
                [left.map(|left| (left, style)), right.map(|right| (right, style))]
            })
            .flatten()
            .collect();
    }

    /// Gets an iterator that visits the elements in the `TextStyle` in ascending order.
    /// If you want get non-overlaping ranges look [`ranges`](Self::ranges)
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

    /// TODO: New ranges can intersect each other if usize::MAX use or usize::MAX - n with delta >=
    /// n. Deal with it
    pub(crate) fn positive_shift(&mut self, start_from: usize, delta: usize) {
        if delta == 0 {
            return;
        }

        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .map(|(mut range, style)| {
                if range.start() >= start_from {
                    range += delta;
                }
                (range, style)
            })
            .collect();
    }

    ///  TODO: It is very unsafe method, because new ranges can overlap. Refactor this method
    ///  with using remove existing ranges. Set panic on intersect and uncomment
    ///  `negative_shift_with_overlap` test.
    pub(crate) fn negative_shift(&mut self, start_from: usize, delta: usize) {
        if delta == 0 {
            return;
        }

        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .map(|(mut range, style)| {
                if range.start() >= start_from {
                    range -= delta;
                }
                (range, style)
            })
            .collect();
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
    fn add_inner<R: RangeBounds<usize>>(&mut self, range: R, style: Style) {
        let mut range = RangeWrapper::new(bound_to_range(range));
        let end_range = RangeWrapper::new(range.end()..);

        let mut previous: Bound<RangeWrapper> = Unbounded;

        while let Some((cur_range, cur_styles)) = self
            .data
            .range((previous.clone(), Included(end_range.clone())))
            .find(|(r, _)| r.overlap_with(range.range()))
            .map(|(range, styles)| (range.clone(), *styles))
        {
            // If ranges are equal do not erase and insert it again, just modify styles in place
            if cur_range == range {
                self.data.get_mut(&cur_range).unwrap().merge(style);
                return;
            }

            self.data.remove(&cur_range);

            let intersection = cur_range.intersection(range.range()).unwrap();
            self.data.insert(intersection.clone(), Style::new().merge(cur_styles).merge(style));

            let [left, middle, right] = cur_range.split_to_3(range.range());
            let left = left.unwrap();
            let middle = middle.unwrap();

            if left != intersection {
                let style = if cur_range.overlap_with(left.range()) { cur_styles } else { style };
                self.data.insert(left, style);
            }

            // Only v and vi variants
            if middle != intersection {
                if cur_range.overlap_with(middle.range()) {
                    self.data.insert(middle, cur_styles);
                    return;
                } else {
                    previous = Included((middle.start()..=middle.start()).into());
                    range = middle;
                    continue;
                }
            }

            if let Some(right) = right {
                if range.overlap_with(right.range()) {
                    previous = Included((right.start()..=right.start()).into());
                    range = right;
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

impl<'a> Iter<'a> {
    pub(crate) fn new(inner: BIter<'a, RangeWrapper, Style>) -> Self {
        Self { inner }
    }
}

impl Iterator for Iter<'_> {
    type Item = StyleInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(r, style)| (r.range(), *style))
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
        self.inner.next_back().map(|(r, style)| (r.range(), *style))
    }
}

impl RangeWrapper {
    fn new<R: RangeBounds<usize>>(range: R) -> Self {
        Self { range: bound_to_range(range) }
    }

    fn cut<R: RangeBounds<usize>>(&self, range: R) -> [Option<RangeWrapper>; 2] {
        let range = bound_to_range(range);

        let start = *range.start();
        let end = *range.end();

        if start > self.end() || end < self.start() {
            return [Some(self.clone()), None];
        }

        let start = if start < self.start() { self.start() } else { start };
        let end = if end > self.end() { self.end() } else { end };

        let left = if start != self.start() { Some(Self::new(self.start()..start)) } else { None };
        let right = if end != self.end() { Some(Self::new(end + 1..=self.end())) } else { None };

        [left, right]
    }

    fn intersection<R: RangeBounds<usize>>(&self, range: R) -> Option<RangeWrapper> {
        let range = bound_to_range(range);

        if self.overlap_with(range.clone()) {
            let left = std::cmp::max(self.start(), *range.start());
            let right = std::cmp::min(self.end(), *range.end());
            Some((left..=right).into())
        } else {
            None
        }
    }

    fn split_to_3<R: RangeBounds<usize>>(&self, range: R) -> [Option<RangeWrapper>; 3] {
        let range: RangeWrapper = bound_to_range(range).into();

        if let Some(intersection) = self.intersection(range.range()) {
            if self.start() == range.start() {
                let bigger_range = if self.len() > range.len() { self.clone() } else { range };
                return [
                    Some(intersection.clone()),
                    bigger_range.cut(intersection.range())[1].clone(),
                    None,
                ];
            }

            let min_start = std::cmp::min(self.start(), range.start());
            let max_end = std::cmp::max(self.end(), range.end());

            let left_result = Some((min_start..intersection.start()).into());
            let middle_result = Some(intersection.clone());
            let right_result = if intersection.end() < max_end {
                Some((intersection.end() + 1..=max_end).into())
            } else {
                None
            };

            [left_result, middle_result, right_result]
        } else {
            [None, None, None]
        }
    }

    fn overlap_with<R: RangeBounds<usize>>(&self, range: R) -> bool {
        let range = bound_to_range(range);
        self.start() <= *range.end() && *range.start() <= self.end()
    }

    fn start(&self) -> usize {
        *self.range.start()
    }

    fn end(&self) -> usize {
        *self.range.end()
    }

    fn range(&self) -> RangeInclusive<usize> {
        self.range.clone()
    }

    fn len(&self) -> usize {
        self.end() - self.start() + 1
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<StyleInfo>> for TextStyle {
    fn into(self) -> Vec<StyleInfo> {
        self.into_vec()
    }
}

impl PartialEq for RangeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.range().eq(other.range())
    }
}

impl PartialEq<RangeInclusive<usize>> for RangeWrapper {
    fn eq(&self, other: &RangeInclusive<usize>) -> bool {
        self.range().eq(other.clone())
    }
}

impl Hash for RangeWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.range.hash(state);
    }
}

impl Ord for RangeWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range().cmp(other.range())
    }
}

impl PartialOrd for RangeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range().partial_cmp(other.range())
    }
}

impl<R: RangeBounds<usize>> From<R> for RangeWrapper {
    fn from(range: R) -> Self {
        Self::new(range)
    }
}

impl Add<usize> for RangeWrapper {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<usize> for RangeWrapper {
    fn add_assign(&mut self, rhs: usize) {
        let start = self.range.start().saturating_add(rhs);
        let end = self.range.end().saturating_add(rhs);
        self.range = start..=end;
    }
}

impl Sub<usize> for RangeWrapper {
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<usize> for RangeWrapper {
    fn sub_assign(&mut self, rhs: usize) {
        let start = self.range.start().saturating_sub(rhs);
        let end = self.range.end().saturating_sub(rhs);
        self.range = start..=end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{Color, Modifier, Style};

    #[test]
    fn cut() {
        let s: RangeWrapper = (0..=4).into();

        let [left, right] = s.cut(1..=2);
        assert_eq!(left, Some((0..=0).into()));
        assert_eq!(right, Some((3..=4).into()));

        let [left, right] = s.cut(0..=1);
        assert_eq!(left, None);
        assert_eq!(right, Some((2..=4).into()));

        let [left, right] = s.cut(3..=4);
        assert_eq!(left, Some((0..=2).into()));
        assert_eq!(right, None);

        let s: RangeWrapper = (1..=2).into();

        let [left, right] = s.cut(1..=2);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let [left, right] = s.cut(0..=4);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let s: RangeWrapper = (1..=1).into();

        let [left, right] = s.cut(3..=5);
        assert_eq!(left, Some((1..=1).into()));
        assert_eq!(right, None);

        let [left, right] = s.cut(0..=0);
        assert_eq!(left, Some((1..=1).into()));
        assert_eq!(right, None);

        let s: RangeWrapper = (2..=10).into();
        let [left, right] = s.cut(2..=5);
        assert_eq!(left, None);
        assert_eq!(right, Some((6..=10).into()));
    }

    #[test]
    fn intersection() {
        let s: RangeWrapper = (2..=5).into();
        assert_eq!(s.intersection(0..=1), None);
        assert_eq!(s.intersection(3..=8), Some((3..=5).into()));
        assert_eq!(s.intersection(1..=3), Some((2..=3).into()));
    }

    //     ─────────────
    // ───                          - 1
    // ────                         - 2
    // ──────────                   - 3
    // ─────────────────            - 4
    // ───────────────────────      - 5
    //     ─────                    - 6
    //     ─────────────            - 7
    //     ───────────────────      - 8
    //           ────               - 9
    //           ───────            - 10
    //           ─────────────      - 11
    //                  ──────      - 12
    //                      ──────  - 13
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 0   2            5
    #[test]
    fn split_to_3() {
        let s: RangeWrapper = (2..=5).into();
        assert_eq!(s.split_to_3(0..=1), [None, None, None]); // 1
        assert_eq!(
            s.split_to_3(0..=2),
            [Some((0..=1).into()), Some((2..=2).into()), Some((3..=5).into())]
        ); // 2
        assert_eq!(
            s.split_to_3(0..=4),
            [Some((0..=1).into()), Some((2..=4).into()), Some((5..=5).into())]
        ); // 3
        assert_eq!(s.split_to_3(0..=5), [Some((0..=1).into()), Some((2..=5).into()), None]); // 4
        assert_eq!(
            s.split_to_3(0..=10),
            [Some((0..=1).into()), Some((2..=5).into()), Some((6..=10).into())]
        ); // 5
        assert_eq!(s.split_to_3(2..=3), [Some((2..=3).into()), Some((4..=5).into()), None]); // 6
        assert_eq!(s.split_to_3(2..=5), [Some((2..=5).into()), None, None]); // 7
        assert_eq!(s.split_to_3(2..=10), [Some((2..=5).into()), Some((6..=10).into()), None]); // 8
        assert_eq!(
            s.split_to_3(3..=4),
            [Some((2..=2).into()), Some((3..=4).into()), Some((5..=5).into())]
        ); // 9
        assert_eq!(s.split_to_3(3..=5), [Some((2..=2).into()), Some((3..=5).into()), None]); // 10
        assert_eq!(
            s.split_to_3(3..=10),
            [Some((2..=2).into()), Some((3..=5).into()), Some((6..=10).into())]
        ); // 11
        assert_eq!(
            s.split_to_3(5..=10),
            [Some((2..=4).into()), Some((5..=5).into()), Some((6..=10).into())]
        ); // 12
        assert_eq!(s.split_to_3(6..=10), [None, None, None]); // 13
    }

    #[test]
    fn positive_shift() {
        let mut styles = TextStyle::new();

        styles.add(..=4, Style::new().bg(Color::Red));
        styles.add(6..=7, Style::new().bg(Color::Yellow));
        styles.add(8..12, Style::new().bg(Color::Black));
        styles.add(12..14, Style::new().bg(Color::Green));
        styles.add(14.., Style::new().bg(Color::White));

        println!("{:?}", styles.clone().into_vec());

        styles.positive_shift(7, 10);

        let result = vec![
            (0..=4, Style::new().bg(Color::Red)),
            (6..=7, Style::new().bg(Color::Yellow)),
            (18..=21, Style::new().bg(Color::Black)),
            (22..=23, Style::new().bg(Color::Green)),
            (24..=usize::MAX, Style::new().bg(Color::White)),
        ];

        assert_eq!(styles.into_vec(), result);
    }

    #[test]
    fn negative_shift() {
        let mut styles = TextStyle::new();

        styles.add(..=0, Style::new().bg(Color::Blue));
        styles.add(1..=4, Style::new().bg(Color::Red));
        styles.add(5..=6, Style::new().bg(Color::Yellow));
        styles.add(17..=26, Style::new().bg(Color::Yellow));
        styles.add(27.., Style::new().bg(Color::Yellow));

        styles.negative_shift(7, 10);

        let result = vec![
            (0..=0, Style::new().bg(Color::Blue)),
            (1..=4, Style::new().bg(Color::Red)),
            (5..=6, Style::new().bg(Color::Yellow)),
            (7..=16, Style::new().bg(Color::Yellow)),
            (17..=std::usize::MAX - 10, Style::new().bg(Color::Yellow)),
        ];

        assert_eq!(styles.into_vec(), result);
    }

    // #[test]
    // #[should_panic]
    // fn negative_shift_with_overlap() {
    //     let mut styles = TextStyle::new();
    //
    //     styles.add(..=0, Style::new().bg(Color::Blue));
    //     styles.add(1..=4, Style::new().bg(Color::Red));
    //     styles.add(5..=10, Style::new().bg(Color::Red));
    //
    //     styles.negative_shift(5, 3);
    // }
}
