use crate::{IdxRange, Style};
use btree_range_map::{
    generic::map::{IntoIter as MapIntoIter, Iter as MapIter},
    AnyRange, DefaultMapContainer as MapSlab, RangeMap,
};
use std::{
    borrow::Borrow,
    iter::{ExactSizeIterator, Extend, FromIterator, FusedIterator},
    num::NonZeroU8,
    ops::Index,
};

/// [`Mask`] saves [`styles`] for specified ranges of graphemes.
///
/// TODO: Avoid memory allocation on empty mask
///
///
/// [`styles`]: Style
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Mask {
    map: RangeMap<usize, Style>,
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
    inner: MapIter<'a, usize, Style, MapSlab<usize, Style>>,
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
    inner: MapIntoIter<usize, Style, MapSlab<usize, Style>>,
}

impl Mask {
    /// Create empty [`Mask`].
    pub fn new() -> Self {
        // TODO: Replace RangeMap to avoid memory allocation on empty mask
        let mut map = RangeMap::new();
        map.insert(0.., Style::default());
        Self { map }
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
    /// //TODO: Check
    /// ```
    pub fn add(&mut self, range: impl Into<IdxRange>, style: Style) {
        self.map.update(range.into(), |styles| {
            Some(match styles {
                Some(cur_style) => cur_style.merge(style),
                None => style,
            })
        })
    }

    /// Replace `style` for specified `range`. All styles in the `range` are erased before
    /// insert the new `style`. If you wan't to save existed styles, try to use [`add`] method.
    ///
    /// # Examples
    /// TODO
    ///
    /// ```
    /// # use yatui_text::{Mask, Style, Color};
    /// ```
    ///
    /// [`add`]: Self::add
    pub fn replace(&mut self, range: impl Into<IdxRange>, style: Style) {
        self.map.insert(range.into(), style);
    }

    /// Insert tuple of [`IdxRange`] and [`Style`] into `Mask`. Iternally it calls [`add`] method.
    /// See its documentation for more.
    ///
    /// // TODO: Doc use case with iterators
    ///
    /// [`add`]: Mask::add
    pub fn insert(&mut self, (range, style): (IdxRange, Style)) {
        self.add(range, style)
    }

    /// Remove styles for specified `range`. Internally it calls [`reset`] with [`default`] styles.
    ///
    /// [`reset`]: Self::replace
    /// [`default`]: crate::Color::default
    pub fn remove(&mut self, range: impl Into<IdxRange>) {}

    /// Gets an iterator over all pairs of ranges and their styles. It returns non intersecting
    /// ranges in ascending order with style info.
    ///
    /// The iterator element type is ([`IdxRange`], &'a [`Style`]).
    pub fn iter(&self) -> Iter<'_> {
        Iter { inner: self.map.iter() }
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

impl Default for Mask {
    fn default() -> Self {
        Self::new()
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
        static DEFAULT_STYLE: Style = Style::new();
        self.map.get(index).unwrap_or(&DEFAULT_STYLE)
    }
}

impl std::iter::IntoIterator for Mask {
    type Item = (IdxRange, Style);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        /// Gets an owned iterator over all pairs of ranges and their styles. It returns non
        /// intersecting ranges in ascending order with style info.
        Self::IntoIter { inner: self.map.into_iter() }
    }
}

impl<'a> std::iter::IntoIterator for &'a Mask {
    type Item = (IdxRange, &'a Style);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn convert_any_range_to_idx_range<S>((range, style): (AnyRange<usize>, S)) -> (IdxRange, S) {
    (IdxRange::from_bounds(range), style)
}

fn convert_any_range_to_idx_range_ref<S>((range, style): (&AnyRange<usize>, S)) -> (IdxRange, S) {
    convert_any_range_to_idx_range((*range, style))
}

impl<'a> Iterator for Iter<'a> {
    type Item = (IdxRange, &'a Style);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(convert_any_range_to_idx_range_ref)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(convert_any_range_to_idx_range_ref)
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
        self.inner.next().map(convert_any_range_to_idx_range)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(convert_any_range_to_idx_range)
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl FusedIterator for IntoIter {}

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
}
