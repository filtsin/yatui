use crate::{Mask, raw::Raw};

/// [`Text`] is common structure for representing strings in terminal.
///
/// It is wrapper about utf-8 [`String`] but `Text` providing convenient methods for working
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

    /// Apply `mask` to this text.
    pub fn apply(&mut self, mask: Mask) {
        self.mask = mask;
    }

    /// Maxium count of columns in the terminal way in all lines in the text.
    /// It is *O(*1*)* operation if cache of size was not invalidated. Otherwise it is the
    /// *O(*n*)* operation.
    ///
    /// Note that thre result of this function is not equal to graphemes count in the line, because
    /// some graphemes can fill more than 1 column and some having zero width (control characters).
    /// For example german `ö` have width equal 1, but `老` have width 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// /// Second line fills 7 columns
    /// assert_eq!(Text::from("c1\n老hello\r\ntext").columns(), 7);
    /// ```
    pub fn columns(&self) -> usize {
        self.raw.width()
    }

    /// Count if lines in this text.
    /// It is *O(*1*)* operation if cache of size was not invalidated. Otherwise it is the
    /// *O(*n*)* operation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// assert_eq!(Text::from("line1\nline2\r\nline3").lines(), 3);
    /// assert_eq!(Text::from("line1\nline2\r\nline3\n").lines(), 3);
    /// ```
    pub fn lines(&self) -> usize {
        self.raw.height()
    }

    /// Returns `true` if this text has a length of zero, and `false` otherwise.
    /// It is *O(*1*)* operation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// let empty = Text::from("");
    /// assert!(empty.is_empty());
    ///
    /// let not_empty = Text::from("h");
    /// assert!(!not_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Modify text like [`String`] in place with a given closure. Closure can return value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use yatui_text::Text;
    /// let mut text = Text::from("hello");
    /// text.modify(|string| {
    ///     *string = string.replace("el", "i ");
    ///     string.push_str("ok");
    ///     string.make_ascii_uppercase();
    /// });
    /// assert_eq!(text.as_str(), "HI LOOK");
    /// ```
    pub fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        self.raw.modify(f)
    }

    /// Appends a given string slice onto the end of this `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// let mut text = Text::from("hello");
    /// text.push_str(" world");
    ///
    /// assert_eq!(text.as_str(), "hello world");
    /// ```
    pub fn push_str(&mut self, string: &str) {
        self.modify(|s| s.push_str(string));
    }

    /// Extracts a string slive containing the entire `Text`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use yatui_text::Text;
    /// let s = Text::from("foo");
    /// assert_eq!("foo", s.as_str());
    /// ```
    pub fn as_str(&self) -> &str {
        self.raw.as_ref()
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
