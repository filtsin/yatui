use crate::text::Style;

use std::{
    collections::BTreeSet,
    ops::{
        Add, AddAssign,
        Bound::{self, Excluded, Included},
        Range, RangeBounds, RangeInclusive, Sub, SubAssign,
    },
};

use super::{bound_to_range, Grapheme};

#[derive(Default)]
pub struct TextStyle {
    data: BTreeSet<RangeStyle>,
}

#[derive(Debug, Clone, Eq)]
struct RangeStyle {
    range: RangeInclusive<usize>,
    style: Style,
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
    /// assert!(style.iter().eq([(0, 1, Style::new().bg(Color::Yellow))]));
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
    /// assert!(style.iter().eq([(1, 2, Style::new().fg(Color::Red))]));
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
    /// assert!(style.iter().eq([]));
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
    /// assert!(style.iter().eq([
    ///     (0, 0, Style::new().bg(Color::Red)),
    ///     (2, 2, Style::new().fg(Color::Red)),
    ///     (2, 3, Style::new().bg(Color::Red))
    /// ]));
    /// ```
    pub fn remove<R: RangeBounds<usize> + Clone>(&mut self, range: R) {
        let mut copy = BTreeSet::new();

        std::mem::swap(&mut self.data, &mut copy);

        for style in copy.into_iter() {
            let (left, right) = style.cut(range.clone());

            if let Some(left) = left {
                self.data.replace(left);
            }
            if let Some(right) = right {
                self.data.replace(right);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, Style)> + '_ {
        self.data.iter().map(|v| (v.start(), v.end(), v.style))
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

impl RangeStyle {
    fn new<R: RangeBounds<usize>>(range: R, style: Style) -> Self {
        Self { range: bound_to_range(range), style }
    }

    fn only_with_range<R: RangeBounds<usize>>(range: R) -> Self {
        Self::new(range, Style::default())
    }

    fn cut<R: RangeBounds<usize>>(&self, range: R) -> (Option<RangeStyle>, Option<RangeStyle>) {
        let range = bound_to_range(range);

        let start = *range.start();
        let end = *range.end();

        let start = if start < self.start() { self.start() } else { start };
        let end = if end > self.end() { self.end() } else { end };

        let left = if start != self.start() {
            Some(Self::new(self.start()..start, self.style))
        } else {
            None
        };

        let right = if end != self.end() {
            Some(Self::new(end + 1..=self.end(), self.style))
        } else {
            None
        };

        (left, right)
    }

    fn start(&self) -> usize {
        *self.range.start()
    }

    fn end(&self) -> usize {
        *self.range.end()
    }
}

impl PartialEq for RangeStyle {
    fn eq(&self, other: &Self) -> bool {
        self.range.clone().eq(other.range.clone())
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
        let start = self.start().checked_add(rhs).unwrap();
        let end = self.end().checked_add(rhs).unwrap();
        self.range = start..=end;
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
        let start = self.start().checked_sub(rhs).unwrap();
        let end = self.end().checked_sub(rhs).unwrap();
        self.range = start..=end;
    }
}

#[cfg(test)]
mod tests {
    use super::RangeStyle;

    #[test]
    fn cut() {
        let mut s: RangeStyle = (0..=4).into();

        let (left, right) = s.cut(1..=2);
        assert_eq!(left, Some((0..=0).into()));
        assert_eq!(right, Some((3..=4).into()));

        let (left, right) = s.cut(0..=1);
        assert_eq!(left, None);
        assert_eq!(right, Some((2..=4).into()));

        let (left, right) = s.cut(3..=4);
        assert_eq!(left, Some((0..=2).into()));
        assert_eq!(right, None);

        let mut s: RangeStyle = (2..=2).into();

        let (left, right) = s.cut(1..=2);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let (left, right) = s.cut(0..=4);
        assert_eq!(left, None);
        assert_eq!(right, None);
    }
}
