use crate::{raw::Raw, Mask};

/// [`Text`] is common structure for representing strings in terminal.
///
/// It is wrapper about utf-8 `String` but `Text` providing convenient methods for working
/// with `grapheme` instead of `character`.
///
/// Also `Text` contains [`Mask`]. This mask apply specified styles for all graphemes in this
/// `Text`. By default, applied mask have [`default`] empty styles. All methods for modifying
/// graphemes in this `Text` do not touch [`Mask`]. It is user responsibility to update mask for
/// their purposes.
///
/// [`default`]: crate::style::Style::default
#[derive(Debug, Default)]
pub struct Text {
    raw: Raw,
    mask: Mask,
}

impl Text {
    /// Create empty [`Text`] with empty [`Mask`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Modify text in place with a given closure. Closure can return any value.
    pub fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        self.raw.modify(f)
    }
}

impl From<&'static str> for Text {
    /// Converts a `&'static str` into [`Text`].
    ///
    /// No heap allocation is performed, and the string is not copied.
    fn from(s: &'static str) -> Self {
        Self { raw: s.into(), ..Default::default() }
    }
}

impl From<String> for Text {
    /// Converts the given [`String`] into [`Text`].
    ///
    /// No heap allocation is performed, and the string is not copied.
    fn from(s: String) -> Self {
        Self { raw: s.into(), ..Default::default() }
    }
}

impl From<char> for Text {
    /// Converts [`char`] into [`Text`] with dynamic allocation an owned [`String`].
    fn from(c: char) -> Self {
        String::from(c).into()
    }
}
