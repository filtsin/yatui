mod cow;
mod idx_range;

use crate::Style;
use btree_range_map::{
    generic::map::{IntoIter as MapIntoIter, Iter as MapIter},
    AnyRange, DefaultMapContainer as MapSlab, RangeMap,
};
use cow::Cow;
pub use idx_range::IdxRange;
use std::{
    borrow::Borrow,
    iter::{ExactSizeIterator, Extend, FromIterator, FusedIterator},
    ops::{Index, RangeInclusive},
};

use self::cow::{CowIntoIter, CowIter};

/// [`Mask`] saves [`styles`] for specified ranges of graphemes.
///
/// [`styles`]: Style
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Mask {
    cow: Cow,
}

/// An iterator over the items of [`Mask`].
///
/// The iterator element type is ([`IdxRange`], &'a [`Style`]).
///
/// This struct is created by the [`iter`] method on [`Mask`]. See its documentation
/// for more.
///
/// [`iter`]: Mask::iter
#[must_use = "Iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    inner: CowIter<'a>,
}

/// An owning iterator over the items of [`Mask`].
///
/// The iterator element type is ([`IdxRange`], [`Style`]).
///
/// This struct is created by the [`into_iter`] method on [`Mask`].
///
/// [`into_iter`]: IntoIterator::into_iter
#[must_use = "Iterators are lazy and do nothing unless consumed"]
pub struct IntoIter {
    inner: CowIntoIter,
}

impl Mask {
    /// Create empty [`Mask`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Add `style` for specified `range`. Merges all styles for intersecting ranges.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::{Mask, Style, Color};
    /// let mut mask = Mask::new();
    /// mask.add(0..=1, Style::new().fg(Color::Yellow));
    /// mask.add(0..2, Style::new().bg(Color::Green));
    /// assert_eq!(mask[0], Style::new().fg(Color::Yellow).bg(Color::Green));
    /// ```
    pub fn add(&mut self, range: impl Into<IdxRange>, style: Style) {
        self.cow.add_style(range, style);
    }

    /// Replace `style` for specified `range`. All styles in the `range` are erased before
    /// insert the new `style`. If you wan't to save existed styles, try to use [`add`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::{Mask, Style, Color};
    /// let mut mask = Mask::new();
    /// mask.add(0..=1, Style::new().fg(Color::Yellow));
    /// mask.replace(0..2, Style::new().bg(Color::Green));
    /// assert_eq!(mask[0], Style::new().bg(Color::Green));
    /// ```
    ///
    /// [`add`]: Self::add
    pub fn replace(&mut self, range: impl Into<IdxRange>, style: Style) {
        self.cow.replace_style(range, style);
    }

    /// Remove styles for specified `range`. Internally it calls [`replace`] with [`default`] styles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::{Mask, Style, Color};
    /// let mut mask = Mask::new();
    /// mask.add(0..=0, Style::new().fg(Color::Yellow));
    /// mask.remove(0..=0);
    /// assert_eq!(mask[0], Style::default());
    /// ```
    ///
    /// [`replace`]: Self::replace
    /// [`default`]: crate::Color::default
    pub fn remove(&mut self, range: impl Into<IdxRange>) {
        self.cow.replace_style(range, Style::default());
    }

    /// Gets an iterator over all pairs of ranges and their styles. It returns non intersecting
    /// ranges in ascending order with style info.
    ///
    /// The iterator element type is ([`IdxRange`], &'a [`Style`]).
    pub fn iter(&self) -> Iter<'_> {
        Iter { inner: self.cow.iter() }
    }
}

impl<R: Into<IdxRange>> Extend<(R, Style)> for Mask {
    fn extend<T: IntoIterator<Item = (R, Style)>>(&mut self, iter: T) {
        for (range, style) in iter.into_iter() {
            self.add(range.into(), style);
        }
    }
}

impl<'a, R: Into<IdxRange>> Extend<(R, &'a Style)> for Mask {
    fn extend<T: IntoIterator<Item = (R, &'a Style)>>(&mut self, iter: T) {
        for (range, style) in iter.into_iter() {
            self.add(range.into(), *style)
        }
    }
}

impl<R: Into<IdxRange>> FromIterator<(R, Style)> for Mask {
    fn from_iter<T: IntoIterator<Item = (R, Style)>>(iter: T) -> Self {
        let mut mask = Mask::default();
        mask.extend(iter);
        mask
    }
}

impl<R, const N: usize> From<[(R, Style); N]> for Mask
where
    R: Into<IdxRange>,
{
    fn from(value: [(R, Style); N]) -> Self {
        Self::from_iter(value)
    }
}

impl Index<usize> for Mask {
    type Output = Style;

    /// Get [`Style`] for specified index `idx`. If no styles in this [`Mask`] for this `idx` then
    /// [`default style`] will be returned.
    ///
    /// [`default style`]: Style::new
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    /// let mask = Mask::new();
    /// assert_eq!(mask[0], Style::default());
    /// ```
    ///
    /// ```
    /// # use yatui_text::*;
    /// let mut mask = Mask::new();
    /// mask.add(0..=1, Style::new().fg(Color::Green));
    /// assert_eq!(mask[0], Style::new().fg(Color::Green));
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        match &self.cow {
            Cow::Single(s) => s,
            Cow::Multiple(m) => {
                m.get(index).expect("Multiple map always contains styles for every idx")
            }
        }
    }
}

impl std::iter::IntoIterator for Mask {
    type Item = (IdxRange, Style);
    type IntoIter = IntoIter;

    /// Gets an owned iterator over all pairs of ranges and their styles. It returns non
    /// intersecting ranges in ascending order with style info.
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { inner: self.cow.into_iter() }
    }
}

impl<'a> std::iter::IntoIterator for &'a Mask {
    type Item = (IdxRange, &'a Style);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (IdxRange, &'a Style);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl Iterator for IntoIter {
    type Item = (IdxRange, Style);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}
impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl FusedIterator for IntoIter {}

/// Creates a [`Mask`] containing styles for specified ranges.
///
/// This macro is similar to `vec!` macro from `std` library. It allows you to define mask
/// with multiple styles. All styles for overlaping ranges will be merged. Internally
/// macro calls [`add`] for all arguments in order.
///
/// # Examples
///
/// ```
/// # use yatui_text::{mask, Style, Color, Mask};
/// let mask = mask!(
///     // You can specifiy any type of range
///     ..2 => Style::new().fg(Color::Green),
///     3..4 => Style::new().fg(Color::Black),
///     4..=5 => Style::new().bg(Color::Yellow),
///     6.. => Style::new().bg(Color::Green),
/// );
/// ```
///
/// ```
/// # use yatui_text::{mask, Style, Color, Mask};
/// let mask = mask!(
///     // Styles will be merged for overlaping ranges.
///     1..3 => Style::new().fg(Color::Green),
///     2..4 => Style::new().bg(Color::Yellow),
/// );
/// assert_eq!(mask[2], Style::new().fg(Color::Green).bg(Color::Yellow));
/// ```
///
/// [`add`]: Mask::add
/// );
#[macro_export]
macro_rules! mask {
    () => {
        $crate::Mask::new()
    };

    ($($range:expr => $style:expr),+ $(,)?) => {{
        let mut mask = Mask::new();
        $(
            mask.add($range, $style);
        )*
        mask
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mask, Color, Modifier};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::ops::RangeInclusive;

    const MAX: usize = usize::MAX;

    #[rstest]
    #[case::non_intersecting(
        mask!(
            ..2 => Style::new().fg(Color::Red),
            2..5 => Style::new().fg(Color::Blue),
            5..=6 => Style::new().fg(Color::Green),
            7.. => Style::new().fg(Color::Yellow),
        ),
        vec![
            (0..=1, Style::new().fg(Color::Red)),
            (2..=4, Style::new().fg(Color::Blue)),
            (5..=6, Style::new().fg(Color::Green)),
            (7..=MAX, Style::new().fg(Color::Yellow)),
        ]
    )]
    //   ───
    //   1 2
    // ───
    // 0 1
    #[case::intersecting_before_left_to_left_point(
        mask!(
            1..=2 => Style::new().fg(Color::Red),
            0..=1 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().bg(Color::Green)),
            (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=2, Style::new().fg(Color::Red)),
            (3..=MAX, Style::default()),
        ]
    )]
    //   ─────
    //   1   3
    // ─────
    // 0   2
    #[case::intersecting_before_left_to_middle_point(
        mask!(
            1..=3 => Style::new().fg(Color::Red),
            0..=2 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().bg(Color::Green)),
            (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
            (3..=3, Style::new().fg(Color::Red)),
            (4..=MAX, Style::default()),
        ]
    )]
    //   ─────
    //   1   3
    // ───────
    // 0     3
    #[case::intersecting_before_left_to_right_point(
        mask!(
            1..=3 => Style::new().fg(Color::Red),
            0..=3 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().bg(Color::Green)),
            (1..=3, Style::new().fg(Color::Red).bg(Color::Green)),
            (4..=MAX, Style::default()),
        ]
    )]
    //   ─────
    //   1   3
    // ─────────
    // 0       4
    #[case::intersecting_before_left_to_after_right_point(
        mask!(
            1..=3 => Style::new().fg(Color::Red),
            0..=4 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().bg(Color::Green)),
            (1..=3, Style::new().fg(Color::Red).bg(Color::Green)),
            (4..=4, Style::new().bg(Color::Green)),
            (5..=MAX, Style::default()),
        ]
    )]
    // ─────
    // 0   2
    // •
    // 0
    #[case::intersecting_left_single_point(
        mask!(
            0..=2 => Style::new().fg(Color::Red),
            0..=0 => Style::new().bg(Color::Green)
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red).bg(Color::Green)),
            (1..=2, Style::new().fg(Color::Red)),
            (3..=MAX, Style::default()),
        ]
    )]
    // ─────
    // 0   2
    // ───
    // 0 1
    #[case::intersecting_left_to_middle_point(
        mask!(
            0..=2 => Style::new().fg(Color::Red),
            0..=1 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=2, Style::new().fg(Color::Red)),
            (3..=MAX, Style::default()),
        ]
    )]
    // ───
    // 0 1
    // ───
    // 0 1
    #[case::intersecting_left_to_right_point(
        mask!(
            0..=1 => Style::new().fg(Color::Red),
            0..=1 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=MAX, Style::default())
        ]
    )]
    // ───
    // 0 1
    // ─────
    // 0   2
    #[case::intersecting_left_to_after_right_point(
        mask!(
            0..=1 => Style::new().fg(Color::Red),
            0..=2 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=2, Style::new().bg(Color::Green)),
            (3..=MAX, Style::default()),
        ]
    )]
    // ─────
    // 0   2
    //   •
    //   1
    #[case::intersecting_middle_single_point(
        mask!(
            0..=2 => Style::new().fg(Color::Red),
            1..=1 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=2, Style::new().fg(Color::Red)),
            (3..=MAX, Style::default()),
        ]
    )]
    // ───────
    // 0     3
    //   ───
    //   1 2
    #[case::intersecting_middle_to_middle_point(
        mask!(
            0..=3 => Style::new().fg(Color::Red),
            1..=2 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
            (3..=3, Style::new().fg(Color::Red)),
            (4..=MAX, Style::default()),
        ]
    )]
    // ─────
    // 0   2
    //   ───
    //   1 2
    #[case::intersecting_middle_to_right_point(
        mask!(
            0..=2 => Style::new().fg(Color::Red),
            1..=2 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
            (3..=MAX, Style::default()),
        ]
    )]
    // ─────
    // 0   2
    //   ─────
    //   1   3
    #[case::intersecting_middle_to_after_right_point(
        mask!(
            0..=2 => Style::new().fg(Color::Red),
            1..=3 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
            (3..=3, Style::new().bg(Color::Green)),
            (4..=MAX, Style::default()),
        ]
    )]
    // ───
    // 0 1
    //   •
    //   1
    #[case::intersecting_right_single_point(
        mask!(
            0..=1 => Style::new().fg(Color::Red),
            1..=1 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=MAX, Style::default()),
        ]
    )]
    // ───
    // 0 1
    //   ───
    //   1 2
    #[case::intersecting_right_to_after_right_point(
        mask!(
            0..=1 => Style::new().fg(Color::Red),
            1..=2 => Style::new().bg(Color::Green),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
            (2..=2, Style::new().bg(Color::Green)),
            (3..=MAX, Style::default()),
        ]
    )]
    // ─────   ─────
    // 0   2   4   6
    //   ─────────
    //   1       5
    #[case::intersecting_two_ranges_with_smaller(
        mask!(
            0..=2 => Style::new().fg(Color::Red),
            4..=6 => Style::new().bg(Color::Green),
            1..=5 => Style::new().modifier(Modifier::BOLD),
        ),
        vec![
            (0..=0, Style::new().fg(Color::Red)),
            (1..=2, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
            (3..=3, Style::new().modifier(Modifier::BOLD)),
            (4..=5, Style::new().bg(Color::Green).modifier(Modifier::BOLD)),
            (6..=6, Style::new().bg(Color::Green)),
            (7..=MAX, Style::default()),
        ]
    )]
    //   ───   ───
    //   1 2   4 5
    // ─────────────
    // 0           6
    #[case::intersecting_two_ranges_with_bigger(
        mask!(
            1..=2 => Style::new().fg(Color::Red),
            4..=5 => Style::new().bg(Color::Green),
            0..=6 => Style::new().modifier(Modifier::BOLD)
        ),
        vec![
            (0..=0, Style::new().modifier(Modifier::BOLD)),
            (1..=2, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
            (3..=3, Style::new().modifier(Modifier::BOLD)),
            (4..=5, Style::new().bg(Color::Green).modifier(Modifier::BOLD)),
            (6..=6, Style::new().modifier(Modifier::BOLD)),
            (7..=MAX, Style::default()),
        ]
    )]
    fn multiple_styles_in_mask(
        #[case] mask: Mask,
        #[case] expected: Vec<(RangeInclusive<usize>, Style)>,
    ) {
        let mask_vec: Vec<_> =
            mask.into_iter().map(|(range, style)| (range.start..=range.end, style)).collect();
        assert_eq!(mask_vec, expected);
    }

    #[test]
    fn index_mask() {
        let mask: Mask = [(0..=1, Style::new().fg(Color::Red))].into();

        assert_eq!(mask[0], mask[1]);
        assert_eq!(mask[0], Style::new().fg(Color::Red));
        assert_eq!(mask[2], Style::default());
    }

    fn iter_mask() {}
}
