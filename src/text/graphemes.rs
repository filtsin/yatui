use std::{hash::Hash, ops::Deref};

pub struct Graphemes<'a> {
    graphemes: Vec<Grapheme<'a>>,
}

pub struct Grapheme<'a> {
    c: &'a str,
    pub(crate) index: usize,
}

impl<'a> Graphemes<'a> {
    pub(crate) fn new(graphemes: Vec<Grapheme<'a>>) -> Self {
        Self { graphemes }
    }
}

impl<'a> Grapheme<'a> {
    pub(crate) fn new((index, c): (usize, &'a str)) -> Self {
        Self { c, index }
    }

    pub fn data(&self) -> &str {
        **self
    }
}

impl<'a> Deref for Graphemes<'a> {
    type Target = Vec<Grapheme<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.graphemes
    }
}

impl<'a> Deref for Grapheme<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

impl Hash for Grapheme<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.c.hash(state);
    }
}
