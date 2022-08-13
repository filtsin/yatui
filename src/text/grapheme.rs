use std::{
    hash::Hash,
    ops::{Deref, RangeInclusive},
};

#[derive(Debug, Clone, Eq)]
pub struct Grapheme<'a> {
    g: &'a str,
    byte_offset: usize,
    index: usize,
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
