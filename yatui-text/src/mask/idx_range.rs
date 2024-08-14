use std::{
    fmt::Display,
    fs::write,
    ops::{Bound, RangeBounds, RangeInclusive},
};

/// Wrapper about inclusive range \[`start`;`end`\]. `IdxRange` used by [`Mask`] to specify
/// a range of graphemes indexes. `IdxRange` can be constructed from any range of std library.
///
/// [`Mask`]: crate::Mask
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub struct IdxRange {
    pub start: usize,
    pub end: usize,
}

impl IdxRange {
    /// Creates a new inclusive range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    /// assert_eq!(IdxRange { start: 0, end: 3 }, IdxRange::new(0, 3));
    /// ```
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Creates a new inclusive range from specified bounds.
    ///
    /// # Panics
    ///
    /// Panics if overflow happened on calculation `start` or `end` for [`Excluded`] bounds.
    ///
    /// [`Excluded`]: Bound::Excluded
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    /// # use std::ops::Bound;
    /// assert_eq!(
    ///     IdxRange::from_bounds((Bound::Included(0), Bound::Excluded(2))),
    ///     IdxRange::new(0, 1)
    /// );
    /// ```
    pub fn from_bounds(range: impl RangeBounds<usize>) -> Self {
        let start = match range.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => s
                .checked_add(1)
                .expect("There is no support for excluded and overflowed start_bound"),
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => {
                s.checked_sub(1).expect("There is no support for excluded and overflowed end_bound")
            }
            Bound::Unbounded => usize::MAX,
        };

        IdxRange::new(start, end)
    }

    /// Returns `true` if `idx` in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    /// assert!(IdxRange::new(0, 1).contains(1));
    /// assert!(IdxRange::new(0, 3).contains(2));
    /// assert!(!IdxRange::new(0, 3).contains(4));
    ///
    /// // Empty range does not contain any value
    /// assert!(!IdxRange::new(5, 1).contains(4));
    /// ```
    pub const fn contains(&self, idx: usize) -> bool {
        self.start <= idx && idx <= self.end
    }

    /// Returns `true` if `end` is lower than `start`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    /// assert!(!IdxRange::new(0, 1).is_empty());
    /// assert!(IdxRange::new(1, 0).is_empty());
    pub const fn is_empty(&self) -> bool {
        self.end < self.start
    }

    /// Converts range into tuple (`start`, `end`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    /// assert_eq!(IdxRange::new(0, 1).into_tuple(), (0, 1));
    /// ```
    pub const fn into_tuple(self) -> (usize, usize) {
        (self.start, self.end)
    }

    /// Returns distance between `end` and `start` + 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    /// assert_eq!(IdxRange::new(0, 1).len(), 2);
    /// assert_eq!(IdxRange::new(0, 0).len(), 1);
    /// assert_eq!(IdxRange::new(1, 0).len(), 0);
    /// ```
    pub const fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.end - self.start + 1
        }
    }

    /// Returns `true` if `self` contains all values in `[0; usize::MAX]`.
    pub const fn is_full(&self) -> bool {
        self.start == 0 && self.end == usize::MAX
    }
}

impl Display for IdxRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{};{}]", self.start, self.end)
    }
}

impl RangeBounds<usize> for IdxRange {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.end)
    }
}

impl RangeBounds<usize> for &IdxRange {
    fn start_bound(&self) -> Bound<&usize> {
        (*self).start_bound()
    }

    fn end_bound(&self) -> Bound<&usize> {
        (*self).end_bound()
    }
}

#[doc(hidden)]
impl btree_range_map::AsRange for IdxRange {
    type Item = usize;

    fn start(&self) -> Bound<&Self::Item> {
        self.start_bound()
    }

    fn end(&self) -> Bound<&Self::Item> {
        self.end_bound()
    }
}

#[doc(hidden)]
impl btree_range_map::IntoRange for IdxRange {
    fn into_range(self) -> btree_range_map::AnyRange<Self::Item> {
        btree_range_map::AnyRange::from(self)
    }
}

impl IntoIterator for IdxRange {
    type Item = usize;
    type IntoIter = RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

macro_rules! impl_traits_std_range {
    ([$($range:ty $(,)?)*]) => {
        $(
            impl From<$range> for IdxRange {
                fn from(range: $range) -> IdxRange {
                    IdxRange::from_bounds(range)
                }
            }

            impl PartialEq<$range> for IdxRange {
                fn eq(&self, other: &$range) -> bool {
                    Into::<IdxRange>::into(other.clone()) == *self
                }
            }

            impl PartialEq<IdxRange> for $range {
                fn eq(&self, other: &IdxRange) -> bool {
                    Into::<IdxRange>::into(self.clone()) == *self
                }
            }
        )*
    }
}

impl_traits_std_range!([
    std::ops::Range<usize>,
    std::ops::RangeFrom<usize>,
    std::ops::RangeFull,
    std::ops::RangeInclusive<usize>,
    std::ops::RangeTo<usize>,
    std::ops::RangeToInclusive<usize>,
]);

#[doc(hidden)]
impl From<btree_range_map::AnyRange<usize>> for IdxRange {
    fn from(value: btree_range_map::AnyRange<usize>) -> Self {
        Self::from_bounds(value)
    }
}

#[doc(hidden)]
impl From<&btree_range_map::AnyRange<usize>> for IdxRange {
    fn from(value: &btree_range_map::AnyRange<usize>) -> Self {
        Self::from_bounds(*value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::ops::Bound::{Excluded, Included};

    #[rstest]
    #[case::std_range_increasing(IdxRange::from(1..3), IdxRange::new(1, 2))]
    #[case::std_range_decreasing(IdxRange::from(3..1), IdxRange::new(3, 0))]
    #[case::std_range_from(IdxRange::from(1..), IdxRange::new(1, usize::MAX))]
    #[case::std_range_full(IdxRange::from(..), IdxRange::new(0, usize::MAX))]
    #[case::std_range_inclusive_increasing(IdxRange::from(1..=3), IdxRange::new(1, 3))]
    #[case::std_range_inclusive_decreasing(IdxRange::from(3..=1), IdxRange::new(3, 1))]
    #[case::std_range_to(IdxRange::from(..3), IdxRange::new(0, 2))]
    #[case::std_range_to_inclusive(IdxRange::from(..=3), IdxRange::new(0, 3))]
    #[case::excluded_start_increasing(
        IdxRange::from_bounds((Excluded(1), Included(3))),
        IdxRange::new(2, 3))
    ]
    #[case::excluded_start_decreasing(
        IdxRange::from_bounds((Excluded(3), Included(1))),
        IdxRange::new(4, 1))
    ]
    #[case::excluded_both_ends(
        IdxRange::from_bounds((Excluded(1), Excluded(2))),
        IdxRange::new(2, 1))
    ]
    #[should_panic]
    #[case::excluded_equal(
        IdxRange::from_bounds((Excluded(1), Excluded(1))),
        IdxRange::new(0, 0))
    ]
    #[should_panic]
    #[case::overflow(IdxRange::from(0..0), IdxRange::new(0, 0))]
    #[should_panic]
    #[case::overflow(IdxRange::from(usize::MAX..usize::MAX), IdxRange::new(0, 0))]
    #[allow(clippy::reversed_empty_ranges)]
    fn create_from_bounds(#[case] range: IdxRange, #[case] expected: IdxRange) {
        assert_eq!(range, expected);
    }
}
