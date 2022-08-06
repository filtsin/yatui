use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Grapheme<'a> {
    g: &'a str,
    byte_offset: usize,
}

impl<'a> Grapheme<'a> {
    pub(crate) fn new((byte_offset, g): (usize, &'a str)) -> Self {
        Self { g, byte_offset }
    }

    pub fn data(&self) -> &str {
        self.as_ref()
    }

    pub fn start(&self) -> usize {
        self.byte_offset
    }

    pub fn end(&self) -> usize {
        self.byte_offset + self.g.len() - 1
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
