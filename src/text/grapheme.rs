use std::{
    hash::Hash,
    ops::{Deref, RangeInclusive},
};

use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Eq)]
pub struct Grapheme<'a> {
    g: &'a str,
    byte_offset: usize,
    index: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GraphemeWidth {
    Zero,
    One,
    Two,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GraphemeInfo {
    index: usize,
    bytes: usize,
    byte_offset: usize,
}

impl<'a> Grapheme<'a> {
    pub(crate) fn new((index, (byte_offset, g)): (usize, (usize, &'a str))) -> Self {
        Self { g, byte_offset, index }
    }

    pub fn data(&self) -> &str {
        self.as_ref()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn start(&self) -> usize {
        self.byte_offset
    }

    pub fn end(&self) -> usize {
        self.clone().info().end()
    }

    pub fn width(&self) -> GraphemeWidth {
        match UnicodeWidthStr::width(self.data()) {
            0 => GraphemeWidth::Zero,
            1 => GraphemeWidth::One,
            2 => GraphemeWidth::Two,
            _ => unreachable!(),
        }
    }

    pub(crate) fn info(self) -> GraphemeInfo {
        GraphemeInfo::new(self.index, self.g.len(), self.byte_offset)
    }
}

impl GraphemeInfo {
    pub fn new(index: usize, bytes: usize, byte_offset: usize) -> Self {
        Self { index, bytes, byte_offset }
    }

    pub fn len(&self) -> usize {
        self.bytes
    }

    pub fn start(&self) -> usize {
        self.byte_offset
    }

    pub fn end(&self) -> usize {
        self.byte_offset + self.len() - 1
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn bytes_range(&self) -> RangeInclusive<usize> {
        self.start()..=self.end()
    }

    pub fn bytes_to(&self, g2: GraphemeInfo) -> RangeInclusive<usize> {
        debug_assert!(self.start() <= g2.start());
        self.start()..=g2.end()
    }
}

impl AsRef<str> for Grapheme<'_> {
    fn as_ref(&self) -> &str {
        self.g
    }
}

impl Deref for Grapheme<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl PartialEq<&str> for Grapheme<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.g == *other
    }
}

impl Hash for Grapheme<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.g.hash(state);
    }
}

impl PartialEq for Grapheme<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.g == other.g
    }
}

impl From<GraphemeWidth> for usize {
    fn from(val: GraphemeWidth) -> Self {
        match val {
            GraphemeWidth::Zero => 0,
            GraphemeWidth::One => 1,
            GraphemeWidth::Two => 2,
        }
    }
}

impl GraphemeWidth {
    pub fn num(self) -> usize {
        self.into()
    }
}
