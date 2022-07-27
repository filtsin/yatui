use crate::terminal::style::Style;
use std::borrow::Cow;

pub struct Str {
    content: Cow<'static, str>,
    style: Style,
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
