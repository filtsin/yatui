use std::ops::{Add, AddAssign, RangeBounds, RangeInclusive, Sub, SubAssign};

use crate::text::utils::bound_to_range;

/// Inclusive range [`start`; `end`]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new<R>(r: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let range = bound_to_range(r);
        Self { start: *range.start(), end: *range.end() }
    }

    pub fn intersects_with<R: Into<Range>>(&self, rhs: R) -> bool {
        let rhs = rhs.into();
        self.start <= rhs.end && rhs.start <= self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }

    pub(crate) fn cut<R: Into<Range>>(&self, range: R) -> [Option<Self>; 2] {
        let range = range.into();
        let start = range.start;
        let end = range.end;

        if start > self.end || end < self.start {
            return [Some(*self), None];
        }

        let start = if start < self.start { self.start } else { start };
        let end = if end > self.end { self.end } else { end };

        let left = if start != self.start { Some(Self::new(self.start..start)) } else { None };
        let right = if end != self.end { Some(Self::new(end + 1..=self.end)) } else { None };

        [left, right]
    }

    pub(crate) fn intersection<R: Into<Range>>(&self, range: R) -> Option<Self> {
        let range = range.into();
        if self.intersects_with(range) {
            let left = std::cmp::max(self.start, range.start);
            let right = std::cmp::min(self.end, range.end);
            Some((left..=right).into())
        } else {
            None
        }
    }

    pub(crate) fn split_to_3<R: Into<Range>>(&self, range: R) -> [Option<Self>; 3] {
        let range = range.into();
        if let Some(intersection) = self.intersection(range) {
            if self.start == range.start {
                let bigger_range = if self.len() > range.len() { *self } else { range };
                return [Some(intersection), bigger_range.cut(intersection)[1], None];
            }

            let min_start = std::cmp::min(self.start, range.start);
            let max_end = std::cmp::max(self.end, range.end);

            let left_result = Some((min_start..intersection.start).into());
            let middle_result = Some(intersection);
            let right_result = if intersection.end < max_end {
                Some((intersection.end + 1..=max_end).into())
            } else {
                None
            };

            [left_result, middle_result, right_result]
        } else {
            [None, None, None]
        }
    }
}

impl<R> From<R> for Range
where
    R: RangeBounds<usize>,
{
    fn from(r: R) -> Self {
        Self::new(r)
    }
}

impl AddAssign<usize> for Range {
    fn add_assign(&mut self, rhs: usize) {
        self.start = self.start.saturating_add(rhs);
        self.end = self.end.saturating_add(rhs);
    }
}

impl Add<usize> for Range {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<usize> for Range {
    fn sub_assign(&mut self, rhs: usize) {
        self.start = self.start.saturating_sub(rhs);
        self.end = self.end.saturating_sub(rhs);
    }
}

impl Sub<usize> for Range {
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self -= rhs;
        self
    }
}

impl PartialEq<RangeInclusive<usize>> for Range {
    fn eq(&self, other: &RangeInclusive<usize>) -> bool {
        self.start == *other.start() && self.end == *other.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut() {
        let s: Range = (0..=4).into();

        let [left, right] = s.cut(1..=2);
        assert_eq!(left, Some((0..=0).into()));
        assert_eq!(right, Some((3..=4).into()));

        let [left, right] = s.cut(0..=1);
        assert_eq!(left, None);
        assert_eq!(right, Some((2..=4).into()));

        let [left, right] = s.cut(3..=4);
        assert_eq!(left, Some((0..=2).into()));
        assert_eq!(right, None);

        let s: Range = (1..=2).into();

        let [left, right] = s.cut(1..=2);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let [left, right] = s.cut(0..=4);
        assert_eq!(left, None);
        assert_eq!(right, None);

        let s: Range = (1..=1).into();

        let [left, right] = s.cut(3..=5);
        assert_eq!(left, Some((1..=1).into()));
        assert_eq!(right, None);

        let [left, right] = s.cut(0..=0);
        assert_eq!(left, Some((1..=1).into()));
        assert_eq!(right, None);

        let s: Range = (2..=10).into();
        let [left, right] = s.cut(2..=5);
        assert_eq!(left, None);
        assert_eq!(right, Some((6..=10).into()));
    }

    #[test]
    fn intersection() {
        let s: Range = (2..=5).into();
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
        let s: Range = (2..=5).into();
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
}
