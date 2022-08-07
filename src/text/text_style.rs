use crate::text::Style;

use std::{
    collections::BTreeSet,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::Grapheme;

#[derive(Default)]
pub struct TextStyle {
    data: BTreeSet<RangeStyle>,
}

#[derive(Debug, Clone, Default, Eq)]
struct RangeStyle {
    // [start;end]
    start: usize,
    end: usize,
    style: Style,
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add style with byte offsets `\[start;end\]`
    /// There are no checks for byte offsets on target Text.
    /// If `\[start;end\]` style already exists, replace it.
    ///
    /// # Panics
    ///
    /// Panics if start > end
    pub fn add(&mut self, start: usize, end: usize, style: Style) {
        self.data.replace(RangeStyle::new(start, end, style));
    }

    /// Add style with byte offsets of `Grapheme` wrapper.
    /// There is no checks about `Grapheme` parent `Text` and their byte offsets.
    /// Simplistically this is an alias for add_style.
    ///
    /// # Panics
    ///
    /// Panics if `start_g` have bigger byte ofsset than `end_g`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// # let mut text: Text = "0".into();
    /// # let (mut graphemes, _) = text.parts();
    /// # let start_g = &graphemes.next().unwrap();
    /// # let end_g = &start_g.clone();
    /// # let style = Style::default();
    /// # use yatui::text::TextStyle;
    /// # let mut styles1 = TextStyle::new();
    /// # let mut styles2 = TextStyle::new();
    /// styles1.add_with_grapheme(start_g, end_g, style);
    /// styles2.add(start_g.start(), end_g.end(), style);
    ///
    /// assert!(styles1.iter().eq(styles2.iter()));
    /// ```
    pub fn add_with_grapheme(
        &mut self,
        start_g: &Grapheme<'_>,
        end_g: &Grapheme<'_>,
        style: Style,
    ) {
        self.add(start_g.start(), end_g.end(), style)
    }

    /// Remove style with byte offsets.
    ///
    /// It removes only full match `\[start;end\]`. If `\[start;end\]` is subset ob bigger range then
    /// there will be no changes. Possibly you want [`remove`](Self::remove)
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut style = TextStyle::new();
    ///
    /// style.add(0, 3, Style::new().bg(Color::Red));
    /// style.add(1, 2, Style::new().fg(Color::Red));
    /// style.remove_range(0, 3);
    ///
    /// assert!(style.iter().eq([(1, 2, Style::new().fg(Color::Red))]));
    /// ```
    pub fn remove_range(&mut self, start: usize, end: usize) {
        self.data.remove(&RangeStyle::only_with_range(start, end));
    }

    /// Remove style with byte offsets of `Grapheme` wrapper. There is no checks about `Grapheme`
    /// parent `Text` and their byte offsets.
    /// Simplistically this is an alias for `remove`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// # let mut text: Text = "0".into();
    /// # let (mut graphemes, _) = text.parts();
    /// # let start_g = &graphemes.next().unwrap();
    /// # let end_g = &start_g.clone();
    /// # let style = Style::default();
    /// # use yatui::text::TextStyle;
    /// # let mut styles1 = TextStyle::new();
    /// # let mut styles2 = TextStyle::new();
    /// # styles1.add(0, 0, Style::default());
    /// # styles2.add(0, 0, Style::default());
    /// styles1.remove_with_grapheme(start_g, end_g);
    /// styles2.remove(start_g.start(), end_g.end());
    ///
    /// assert!(styles1.iter().eq(styles2.iter()));
    /// ```
    pub fn remove_with_grapheme(&mut self, start_g: &Grapheme<'_>, end_g: &Grapheme<'_>) {
        self.remove(start_g.start(), end_g.end());
    }

    /// Remove all styles for `\[start;end\]`.
    ///
    /// Unlike [`remove_range`][Self::remove_range] it removes all intersections
    /// with \[start;end\] of bigger ranges. Possibly you want use it for style clearing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut style = TextStyle::new();
    ///
    /// style.add(0, 3, Style::new().bg(Color::Red));
    /// style.add(1, 2, Style::new().fg(Color::Red));
    /// style.remove(0, 3);
    ///
    /// assert!(style.iter().eq([]));
    /// ```
    ///
    /// ```
    /// # use yatui::text::*;
    /// let mut style = TextStyle::new();
    ///
    /// style.add(0, 3, Style::new().bg(Color::Red));
    /// style.add(1, 2, Style::new().fg(Color::Red));
    /// style.remove(1, 1);
    ///
    /// assert!(style.iter().eq([
    ///     (0, 0, Style::new().bg(Color::Red)),
    ///     (2, 2, Style::new().fg(Color::Red)),
    ///     (2, 3, Style::new().bg(Color::Red))
    /// ]));
    /// ```
    pub fn remove(&mut self, start: usize, end: usize) {
        let mut copy = BTreeSet::new();

        std::mem::swap(&mut self.data, &mut copy);

        for style in copy.into_iter() {
            let (left, right) = style.cut(start, end);

            if let Some(left) = left {
                self.data.insert(left);
            }
            if let Some(right) = right {
                self.data.insert(right);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, Style)> + '_ {
        self.data.iter().map(|v| (v.start, v.end, v.style))
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
        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .map(|mut range| {
                if range.start >= start_from {
                    range += delta;
                }
                range
            })
            .collect();
    }

    pub(crate) fn negative_shift(&mut self, start_from: usize, delta: usize) {
        self.data = std::mem::take(&mut self.data)
            .into_iter()
            .map(|mut range| {
                if range.start >= start_from {
                    range -= delta;
                }
                range
            })
            .collect();
    }
}

impl RangeStyle {
    fn new(start: usize, end: usize, style: Style) -> Self {
        Self { start, end, style }
    }

    fn only_with_range(start: usize, end: usize) -> Self {
        Self::new(start, end, Style::default())
    }

    fn cut(&self, start: usize, end: usize) -> (Option<RangeStyle>, Option<RangeStyle>) {
        assert!(start <= end);

        let start = if start < self.start { self.start } else { start };
        let end = if end > self.end { self.end } else { end };

        let left = if start != self.start {
            Some(Self::new(self.start, start - 1, self.style))
        } else {
            None
        };

        let right =
            if end != self.end { Some(Self::new(end + 1, self.end, self.style)) } else { None };

        (left, right)
    }
}

impl PartialEq for RangeStyle {
    fn eq(&self, other: &Self) -> bool {
        [self.start, self.end] == [other.start, other.end]
    }
}

impl Ord for RangeStyle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        [self.start, self.end].cmp(&[other.start, other.end])
    }
}

impl PartialOrd for RangeStyle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        [self.start, self.end].partial_cmp(&[other.start, other.end])
    }
}

impl From<std::ops::RangeInclusive<usize>> for RangeStyle {
    fn from(range: std::ops::RangeInclusive<usize>) -> Self {
        Self::only_with_range(*range.start(), *range.end())
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
        self.start = self.start.checked_add(rhs).unwrap();
        self.end = self.end.checked_add(rhs).unwrap();
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
        self.start = self.start.checked_sub(rhs).unwrap();
        self.end = self.end.checked_sub(rhs).unwrap();
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
