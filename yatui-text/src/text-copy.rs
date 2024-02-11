use crate::raw::Raw;

/// `Text` is common structure for representing strings in terminal.
/// It is a simple wrapper about utf-8 string but `Text` providing convenient methods for
/// working with `graphemes`.
#[derive(Debug, Default)]
pub struct Span {
    raw: Raw,
}

#[derive(Debug, Default)]
pub struct Text {
    span: Span,
    mask: Mask,
}

/// `Mask` in simple map of `range` to `Style`.
#[derive(Debug, Default)]
pub struct Mask {}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        self.raw.as_str()
    }

    /// Modify text in place with a given closure. Closure can return value.
    ///
    /// # Examples
    pub fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        self.raw.modify(f)
    }

    pub fn apply(self, mask: Mask) -> TextWithMask {
        TextWithMask { t: self, m: mask }
    }
}

impl TextWithMask {
    pub fn text(&self) -> &Text {
        &self.t
    }

    pub fn text_mut(&mut self) -> &mut Text {
        &mut self.t
    }
}

fn foo() {}
