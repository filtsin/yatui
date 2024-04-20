use crate::{IdxRange, Style};
use btree_range_map::{
    generic::map::{IntoIter as MapIntoIter, Iter as MapIter},
    AnyRange, DefaultMapContainer as MapSlab, RangeMap,
};
use std::{
    iter::{ExactSizeIterator, Extend, FromIterator, FusedIterator},
    ops::Index,
};

/// [`Mask`] saves [`styles`] for specified ranges of graphemes.
///
/// [`styles`]: Style
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
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
    /// Create empty `Mask`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add `style` for specified `range`. It merges all styles for overlapping ranges.
    ///
    /// # Panics
    ///
    /// Panics if `start_bound` of range > `end_bound`
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::{Mask, Style, Color};
    /// let mut mask = Mask::new();
    /// mask.add(0..=1, Style::new().fg(Color::Yellow));
    /// mask.add(0..2, Style::new().bg(Color::Green));
    /// ```
    pub fn add(&mut self, range: impl Into<IdxRange>, style: Style) {
        self.map.update(range.into(), |styles| {
            Some(match styles {
                Some(cur_style) => cur_style.merge(style),
                None => style,
            })
        })
    }

    /// Insert tuple of [`IdxRange`] and [`Style`] into `Mask`. Iternally it calls [`add`] method.
    /// See its documentation for more.
    ///
    /// [`add`]: Mask::add
    pub fn insert(&mut self, (range, style): (IdxRange, Style)) {
        self.add(range, style)
    }

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
            self.add(range.into(), style.clone())
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
