use std::hash::Hash;

#[derive(Debug, Clone)]
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

    pub fn distance_to(&self, rhs: GraphemeInfo) -> usize {
        if self.byte_offset > rhs.byte_offset {
            GraphemeInfo::distance_between(*self, rhs)
        } else {
            GraphemeInfo::distance_between(rhs, *self)
        }
    }

    fn distance_between(lhs: GraphemeInfo, rhs: GraphemeInfo) -> usize {
        lhs.end() - rhs.start() + 1
    }
}

impl AsRef<str> for Grapheme<'_> {
    fn as_ref(&self) -> &str {
        self.g
    }
}

impl Hash for Grapheme<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.g.hash(state);
    }
}
