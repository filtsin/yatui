use std::{
    fmt::Display,
    fs::write,
    ops::{Bound, RangeBounds, RangeInclusive},
};

/// Wrapper about inclusive range [`start`;`end`]. `IdxRange` used by [`Mask`] to specify
/// a range of graphemes indexes. `IdxRange` can be constructed from any range from std library.
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
    ///
    /// assert_eq!(IdxRange { start: 0, end: 3 }, IdxRange::new(0, 3));
    /// ```
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns `true` if `idx` in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::IdxRange;
    ///
    /// assert!(IdxRange::new(0, 1).contains(1));
    /// assert!(IdxRange::new(0, 3).contains(2));
    /// assert!(!IdxRange::new(0, 3).contains(4));
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
    ///
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
    ///
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
    ///
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
                    utils::bound_to_inclusive(range)
                }
            }

            impl PartialEq<$range> for IdxRange {
                fn eq(&self, other: &$range) -> bool {
                    Into::<IdxRange>::into(other.clone()) == *self
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
    std::ops::RangeToInclusive<usize>
]);

mod utils {
    use crate::IdxRange;
    use std::ops::{RangeBounds, RangeInclusive};

    pub(super) fn bound_to_inclusive(range: impl RangeBounds<usize>) -> IdxRange {
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

        IdxRange::new(start, end)
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

        #[test]
        fn bound_to_inclusive_start_gt_end() {
            assert!(bound_to_inclusive((Included(3), Included(2))).is_empty());
        }
    }
}