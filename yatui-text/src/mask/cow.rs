use std::{iter::FusedIterator, ops::RangeBounds};

use btree_range_map::{
    generic::map::{IntoIter, Iter},
    AnyRange, DefaultMapContainer, RangeMap,
};

use crate::{IdxRange, Style};

/// It is a smart enum for `Mask` content.
/// Simply, it has two options:
/// 1. Single - Single style for one unbounded range (.. from std).
/// 2. Multiple - Multiple styles for different ranges.
/// The struct is designed to avoid memory allocation for the most common case when all Text's
/// graphemes should have single style (Also default mask also have single style Style::default).
/// When mutation of styles needed, it converts `Single` variant to `Multiple` (like std::Cow).
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(super) enum Cow {
    Single(Style),
    Multiple(RangeMap<usize, Style>),
}

pub(super) type CowIter<'a> =
    CowIterCommon<&'a Style, Iter<'a, usize, Style, DefaultMapContainer<usize, Style>>>;

pub(super) type CowIntoIter =
    CowIterCommon<Style, IntoIter<usize, Style, DefaultMapContainer<usize, Style>>>;

impl Cow {
    pub(super) fn new() -> Self {
        Self::Single(Style::default())
    }

    pub(super) fn to_mut(&mut self) -> &mut RangeMap<usize, Style> {
        match *self {
            Self::Single(s) => {
                let mut map = RangeMap::new();
                map.insert(0..=usize::MAX, s);
                *self = Self::Multiple(map);
                let Self::Multiple(ref mut map) = self else { unreachable!() };
                map
            }
            Self::Multiple(ref mut map) => map,
        }
    }

    pub(super) fn add_style(&mut self, range: impl Into<IdxRange>, style: Style) {
        let range = range.into();
        match self {
            Cow::Single(s) if range.is_full() => {
                *s = style;
            }
            _ => self.to_mut().update(range, |styles| {
                Some(match styles {
                    Some(cur_style) => cur_style.merge(style),
                    None => style,
                })
            }),
        }
    }

    pub(super) fn replace_style(&mut self, range: impl Into<IdxRange>, style: Style) {
        let range = range.into();
        match self {
            Cow::Single(s) if range.is_full() => {
                *s = style;
            }
            _ => self.to_mut().insert(range, style),
        }
    }

    pub(super) fn iter(&self) -> CowIter<'_> {
        match self {
            Cow::Single(s) => CowIter::Single(Some(s)),
            Cow::Multiple(m) => CowIter::Multiple(m.iter()),
        }
    }

    pub(super) fn into_iter(self) -> CowIntoIter {
        match self {
            Cow::Single(s) => CowIntoIter::Single(Some(s)),
            Cow::Multiple(m) => CowIntoIter::Multiple(m.into_iter()),
        }
    }

    /// Returns `true` if it had allocated memory.
    pub(super) fn is_owned(&self) -> bool {
        match *self {
            Cow::Single(_) => false,
            Cow::Multiple(_) => true,
        }
    }
}

impl Default for Cow {
    fn default() -> Self {
        Self::new()
    }
}

// Common iterator type for cow for owned and borrowed types
// S - it is style type: owned (Style) or borrowed (&Style)
// I - it is iterator of btree_range_map dependency: owned (IntoIter) or borrowed (Iter)
pub(super) enum CowIterCommon<S, I> {
    Single(Option<S>),
    Multiple(I),
}

fn map_multiple_item<S>((range, style): (impl Into<IdxRange>, S)) -> (IdxRange, S) {
    (range.into(), style)
}

impl<I, R, S> Iterator for CowIterCommon<S, I>
where
    I: Iterator<Item = (R, S)>,
    R: Into<IdxRange>,
{
    type Item = (IdxRange, S);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CowIterCommon::Single(s) => s.take().map(|style| (IdxRange::from_bounds(..), style)),
            CowIterCommon::Multiple(m) => m.next().map(map_multiple_item),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            CowIterCommon::Single(s) if s.is_some() => (1, Some(1)),
            CowIterCommon::Single(_) => (0, Some(0)),
            CowIterCommon::Multiple(m) => m.size_hint(),
        }
    }
}

impl<I, R, S> DoubleEndedIterator for CowIterCommon<S, I>
where
    I: DoubleEndedIterator<Item = (R, S)>,
    R: Into<IdxRange>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            CowIterCommon::Single(_) => self.next(),
            CowIterCommon::Multiple(m) => m.next_back().map(map_multiple_item),
        }
    }
}

impl<I, R, S> ExactSizeIterator for CowIterCommon<S, I>
where
    I: Iterator<Item = (R, S)> + ExactSizeIterator,
    R: Into<IdxRange>,
{
}

impl<I, R, S> FusedIterator for CowIterCommon<S, I>
where
    I: Iterator<Item = (R, S)> + FusedIterator,
    R: Into<IdxRange>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use rstest::rstest;

    // No macro in btree_range_map crate :(
    macro_rules! range_map {
        ($($range:expr => $style:expr),+ ,) => {{
            let mut map = RangeMap::new();
            $(
                map.insert($range, $style);
            )*
            map
        }}
    }

    #[test]
    fn single_cow_is_not_owned() {
        assert_eq!(Cow::default(), Cow::Single(Style::default()));
        assert!(!Cow::default().is_owned());
        assert!(!Cow::Single(Style::new().fg(Color::Green)).is_owned());
    }

    #[test]
    fn multiple_cow_is_owned() {
        assert!(Cow::Multiple(RangeMap::new()).is_owned());
    }

    #[test]
    fn cow_single_to_mut() {
        let mut cow = Cow::Single(Style::new().fg(Color::Red));
        cow.to_mut();
        assert_eq!(cow, Cow::Multiple(range_map!(0..=usize::MAX => Style::new().fg(Color::Red),)));
    }

    #[rstest]
    #[case::full_single(
        vec![
            (IdxRange::from(..), Style::new().fg(Color::Red))
        ],
        Cow::Single(Style::new().fg(Color::Red))
    )]
    #[case::partial_single(
        vec![
            (IdxRange::from(5..), Style::new().fg(Color::Red))
        ],
        Cow::Multiple(range_map!(
            0..=4 => Style::default(),
            5.. => Style::new().fg(Color::Red),
        ))
    )]
    #[case::multiple_styles(
        vec![
            (IdxRange::from(1..=2), Style::new().fg(Color::Red)),
            (IdxRange::from(6..=8), Style::new().fg(Color::Green))
        ],
        Cow::Multiple(range_map!(
            0..=0 => Style::default(),
            1..=2 => Style::new().fg(Color::Red),
            3..6 => Style::default(),
            6..=8 => Style::new().fg(Color::Green),
            9.. => Style::default(),
        ))
    )]
    fn cow_add_styles(#[case] styles: Vec<(IdxRange, Style)>, #[case] expected: Cow) {
        let mut cow = Cow::default();
        for (range, style) in styles {
            cow.add_style(range, style);
        }
        assert_eq!(cow, expected);
    }

    #[test]
    fn cow_check_iter_traits() {
        fn check_iter<I, S>(i: I)
        where
            I: Iterator<Item = (IdxRange, S)>
                + DoubleEndedIterator<Item = (IdxRange, S)>
                + FusedIterator
                + ExactSizeIterator,
        {
        }

        let cow = Cow::default();
        check_iter(cow.iter());
        check_iter(cow.into_iter());
    }
}
