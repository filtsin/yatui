use crate::text::{raw_text::RawText, Grapheme, GraphemeInfo};
use std::{
    cmp::Eq,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds, RangeInclusive,
    },
};

// Fast alias for iter_bound for GraphemeInfo
pub(crate) fn get_graphemes_info<'a, I, R>(iter: I, range: R) -> (GraphemeInfo, GraphemeInfo)
where
    I: IntoIterator<Item = Grapheme<'a>>,
    R: RangeBounds<usize>,
{
    let [g1, g2] = iter_bound(iter, range);
    (g1.info(), g2.info())
}

// Get elements from iterator by any Range
pub(crate) fn iter_bound<I, R, K>(iter: I, range: R) -> [K; 2]
where
    I: IntoIterator<Item = K>,
    R: RangeBounds<usize>,
    K: Clone,
{
    let mut iter = iter.into_iter();

    let start_idx = match range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n.checked_add(1).unwrap(),
        Unbounded => 0,
    };

    let left = iter.nth(start_idx).unwrap();

    let right = if range.end_bound() == Unbounded {
        iter.last().unwrap()
    } else {
        let end_idx = match range.end_bound() {
            Included(&n) => n,
            Excluded(&n) => n.checked_sub(1).unwrap(),
            Unbounded => unreachable!(),
        };

        assert!(start_idx <= end_idx);

        if end_idx == start_idx { left.clone() } else { iter.nth(end_idx - start_idx - 1).unwrap() }
    };

    [left, right]
}

// convert any Range to RangeInclusive
pub(crate) fn bound_to_range<R: RangeBounds<usize>>(range: R) -> RangeInclusive<usize> {
    let start = match range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n.checked_add(1).unwrap(),
        Unbounded => 0,
    };

    let end = match range.end_bound() {
        Included(&n) => n,
        Excluded(&n) => n.checked_sub(1).unwrap(),
        Unbounded => usize::MAX,
    };

    assert!(start <= end);

    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_to_range_() {
        assert_eq!(bound_to_range(..), 0..=usize::MAX);
        assert_eq!(bound_to_range(1..), 1..=usize::MAX);
        assert_eq!(bound_to_range(..5), 0..=4);
        assert_eq!(bound_to_range(1..4), 1..=3);
        assert_eq!(bound_to_range(3..=6), 3..=6);
        assert_eq!(bound_to_range(..=6), 0..=6);
    }

    #[test]
    fn iter_bound_() {
        let arr = [0, 1, 2, 3, 4, 5, 6];

        assert_eq!(iter_bound(arr, ..), [0, 6]);
        assert_eq!(iter_bound(arr, 1..), [1, 6]);
        assert_eq!(iter_bound(arr, ..5), [0, 4]);
        assert_eq!(iter_bound(arr, 1..4), [1, 3]);
        assert_eq!(iter_bound(arr, 3..=6), [3, 6]);
        assert_eq!(iter_bound(arr, ..=6), [0, 6]);
    }
}
