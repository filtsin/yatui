mod grapheme;
mod raw_text;
mod style;
mod text_style;

pub use grapheme::Grapheme;
pub use style::{Color, Modifier, Style};
pub use text_style::TextStyle;

use raw_text::RawText;

use std::{borrow::Cow, collections::BTreeSet};

#[derive(Default)]
pub struct Text {
    raw: RawText,
    style: TextStyle,
}

impl Text {
    pub fn new<C>(content: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { raw: RawText::new(content.into()), ..Self::default() }
    }

    pub fn parts(&mut self) -> (impl Iterator<Item = Grapheme<'_>>, &'_ mut TextStyle) {
        (RawText::create_graphemes(self.raw.as_ref()), &mut self.style)
    }

    pub fn push_str(&mut self, string: &str) {
        self.raw.push_str(string);
    }

    pub fn remove(&mut self, grapheme_idx: usize) {
        self.raw.remove(grapheme_idx);
        // TODO:
        // remove styles
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.style.clear()
    }

    pub fn styles(&self) -> &TextStyle {
        &self.style
    }

    pub fn styles_mut(&mut self) -> &mut TextStyle {
        &mut self.style
    }

    pub fn lines(&self) -> usize {
        self.raw.lines()
    }

    pub fn columns(&self) -> usize {
        self.raw.columns()
    }

    pub fn is_borrowed(&self) -> bool {
        self.raw.is_borrowed()
    }

    pub fn is_owned(&self) -> bool {
        self.raw.is_owned()
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.raw.as_ref()
    }
}

impl From<&'static str> for Text {
    fn from(s: &'static str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
