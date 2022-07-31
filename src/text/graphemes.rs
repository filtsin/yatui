use std::{hash::Hash, ops::Deref};

#[derive(Debug)]
pub struct Grapheme<'a> {
    c: &'a str,
    pub(crate) index: usize,
}

impl<'a> Grapheme<'a> {
    pub(crate) fn new((index, c): (usize, &'a str)) -> Self {
        Self { c, index }
    }

    pub fn data(&self) -> &str {
        &*self
    }
}

impl<'a> Deref for Grapheme<'a> {
    type Target = str;

    fn deref(&self) -> &'a Self::Target {
        self.c
    }
}

impl Hash for Grapheme<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.c.hash(state);
    }
}
