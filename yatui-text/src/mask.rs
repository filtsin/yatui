use std::ops::{RangeBounds, RangeInclusive};

use crate::Style;
use rangemap::{
    inclusive_map::IntoIter as MapIntoIter, inclusive_map::Iter as MapIter, RangeInclusiveMap,
};

/// [`Mask`] saves [`styles`] for specified ranges. It is low-level structure for creation of
/// styled [`text`]. Look at high-level functions if you just want create styled text: TODO
///
/// [`styles`]: Style
/// [`text`]: crate::Text
#[derive(Debug, Clone, Default)]
pub struct Mask {
    map: RangeInclusiveMap<usize, Style>,
}

/// An iterator over the items of [`Mask`].
///
/// The iterator element type is `(&'a RangeInclusive<usize>, &'a Style)`.
///
/// This struct is created by the [`iter`] method on [`Mask`]. See its documentation
/// for more.
///
/// [`iter`]: Mask::iter
#[must_use = "Iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    inner: MapIter<'a, usize, Style>,
}

/// An owning iterator over the items of [`Mask`].
///
/// The iterator element type is `(RangeInclusive<usize>, Style)`.
///
/// This struct is created by the [`into_iter`] method on [`Mask`].
///
/// [`into_iter`]: IntoIterator::into_iter
#[must_use = "Iterators are lazy and do nothing unless consumed"]
pub struct IntoIter {
    inner: MapIntoIter<usize, Style>,
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
    ///
    /// let mut mask = Mask::new();
    /// mask.add(0..=1, Style::new().fg(Color::Yellow));
    /// mask.add(0..=2, Style::new().bg(Color::Green));
    /// ```
    pub fn add(&mut self, range: impl RangeBounds<usize>, style: Style) {
        //
    }

    /// Get `Style` for specified index `idx`. If no styles in this `Mask` for this `idx` then
    /// default `Style` will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    ///
    /// let mask = Mask::new();
    /// assert_eq!(mask.get(0), Style::default());
    /// ```
    ///
    /// TODO: Add example with existed styles
    pub fn get(&self, idx: usize) -> Style {
        self.map.get(&idx).cloned().unwrap_or_default()
    }

    /// Gets an iterator over all pairs of ranges and their styles. It returns non intersecting
    /// ranges in ascending order with style info.
    ///
    /// The iterator element type is `(&'a RangeInclusive<usize>, &'a Style)`.
    pub fn iter(&self) -> Iter<'_> {
        Iter { inner: self.map.iter() }
    }
}

impl std::iter::IntoIterator for Mask {
    type Item = <IntoIter as Iterator>::Item;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        /// Gets an owned iterator over all pairs of ranges and their styles. It returns non
        /// interseting ranges in ascending order with style info.
        Self::IntoIter { inner: self.map.into_iter() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a RangeInclusive<usize>, &'a Style);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl Iterator for IntoIter {
    type Item = (RangeInclusive<usize>, Style);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

mod utils {
    use std::ops::{RangeBounds, RangeInclusive};
    pub(super) fn bound_to_inclusive(range: impl RangeBounds<usize>) -> RangeInclusive<usize> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v
                .checked_add(1)
                .expect("There is no support for excluded and overflowed start bound"),
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => {
                v.checked_sub(1).expect("There is no support for excluded and overflowed end bound")
            }
            std::ops::Bound::Unbounded => usize::MAX,
        };

        if start > end {
            panic!("`start_bound` must be lower than `end_bound`, but {start} > {end}");
        }

        RangeInclusive::new(start, end)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::ops::Bound::{self, Excluded, Included, Unbounded};
        #[test]
        fn bound_to_inclusive_without_overflow() {
            assert_eq!(bound_to_inclusive((Excluded(1), Excluded(5))), 2..=4);
            assert_eq!(bound_to_inclusive((Excluded(1), Included(5))), 2..=5);
            assert_eq!(bound_to_inclusive((Included(1), Excluded(5))), 1..=4);
            assert_eq!(bound_to_inclusive((Included(1), Included(5))), 1..=5);
            assert_eq!(bound_to_inclusive((Unbounded, Included(5))), 0..=5);
            assert_eq!(bound_to_inclusive((Included(1), Unbounded)), 1..=usize::MAX);
            assert_eq!(bound_to_inclusive((Bound::<usize>::Unbounded, Unbounded)), 0..=usize::MAX);
        }

        #[test]
        #[should_panic]
        fn bound_to_inclusive_overflow_start() {
            bound_to_inclusive((Excluded(usize::MAX), Included(usize::MAX)));
        }

        #[test]
        #[should_panic]
        fn bound_to_inclusive_overflow_end() {
            bound_to_inclusive((Included(0), Excluded(0)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Mask, Style};

    #[test]
    fn add_full_overlapping() {
        let mut mask = Mask::new();
        mask.add(0..=10, Style::new().fg(Color::Red));
        mask.add(0..=10, Style::new().bg(Color::Green));
    }
}
