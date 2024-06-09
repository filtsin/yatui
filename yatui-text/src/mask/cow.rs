use btree_range_map::{
    generic::map::{IntoIter, Iter},
    AnyRange, DefaultMapContainer, RangeMap,
};

use crate::{IdxRange, Style};

/// It is a smart variant for `Mask` content.
/// Simply, it has two options:
/// 1. Single - Single style for one unbounded range (.. from std).
/// 2. Multiple - Multiple styles for different ranges.
/// The type is designed to avoid memory allocation for the most common case when all Text's
/// graphemes should have single style.
/// When mutation of styles needed, it convert `Single` variant to `Multiple` (like std::Cow).
#[derive(Clone, Eq, PartialEq, Hash)]
pub(super) enum Cow {
    Single(Style),
    Multiple(RangeMap<usize, Style>),
}

pub(super) enum CowIter<'a> {
    Single(Option<Style>),
    Multiple(Iter<'a, usize, Style, DefaultMapContainer<usize, Style>>),
}

impl Cow {
    pub(super) fn new() -> Self {
        Self::default()
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

    /// Returns `true` if it had allocated memory.
    pub(super) fn is_owned(&self) -> bool {
        match *self {
            Cow::Single(_) => false,
            Cow::Multiple(_) => true,
        }
    }
}

// impl CowIter {
//     fn consume_single(&mut self) -> Option<<Self as Iterator>::Item> {
//         let Self::Single(style) = self else { unreachable!() };
//         style.take().map(|style| (IdxRange::from(..), style))
//     }
// }

fn convert_any_range_to_idx_range<S>((range, style): (AnyRange<usize>, S)) -> (IdxRange, S) {
    (IdxRange::from_bounds(range), style)
}

impl Default for Cow {
    fn default() -> Self {
        Self::Single(Style::default())
    }
}
