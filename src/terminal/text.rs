use crate::terminal::style::Style;
use smallvec::SmallVec;
use std::borrow::Cow;

#[derive(Clone)]
pub struct Str {
    content: Cow<'static, str>,
    style: Style,
}

pub struct Text {
    content: SmallVec<[Str; 2]>,
}

impl Str {
    pub fn new(content: Cow<'static, str>) -> Self {
        Self { content, style: Style::default() }
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.content, Cow::Borrowed(_))
    }

    pub fn is_owned(&self) -> bool {
        matches!(self.content, Cow::Owned(_))
    }

    pub fn styles(&self) -> Style {
        self.style
    }

    pub fn styles_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Text {
    pub fn new<I>(content: I) -> Self
    where
        I: IntoIterator<Item = Str>,
    {
        Self { content: SmallVec::from_iter(content.into_iter()) }
    }

    pub fn spilled(&self) -> bool {
        self.content.spilled()
    }
}

impl From<&'static str> for Str {
    fn from(v: &'static str) -> Self {
        Self::new(v.into())
    }
}

impl From<String> for Str {
    fn from(v: String) -> Self {
        Self::new(v.into())
    }
}
