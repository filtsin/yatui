use std::ops::{Deref, RangeBounds};

use crate::raw::Raw;

/// [`Text`] is common structure for representing strings in terminal.
///
/// It is wrapper about utf-8 [`String`] but `Text` providing convenient methods for working
/// with `grapheme` instead of `character`.
///
/// `Text` cached its size and invalidate it on any change of content.
#[derive(Debug, Default)]
pub struct Text {
    raw: Raw,
}

impl Text {
    /// Create empty [`Text`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Maximum count of columns in the terminal way in all lines in the text.
    /// It is *O(*1*)* operation if cache of size was not invalidated. Otherwise it is the
    /// *O(*n*)* operation.
    ///
    /// Note that the result of this function is not equal to graphemes count in the line, because
    /// some graphemes can fill more than 1 column and some having zero width (control characters).
    /// For example german `ö` have width equal 1, but `老` have width 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// assert_eq!(Text::from("line").columns(), 4);
    /// /// Second line fills 7 columns
    /// assert_eq!(Text::from("c1\n老hello\r\ntext").columns(), 7);
    /// ```
    pub fn columns(&self) -> usize {
        self.raw.width()
    }

    /// Count of lines in this text.
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
    ///
    /// assert!(empty.is_empty());
    /// ```
    ///
    /// ```
    /// # use yatui_text::Text;
    /// let not_empty = Text::from("h");
    ///
    /// assert!(!not_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Modify text like [`String`] in place with a given closure.
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
    ///
    /// assert_eq!(text.as_str(), "HI LOOK");
    /// ```
    pub fn modify<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut String) -> R,
    {
        self.raw.modify(f)
    }

    /// Appends a given `char` to the end of this `Text`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use yatui_text::Text;
    /// let mut text = Text::from("abc");
    /// text.push('1');
    ///
    /// assert_eq!(text.as_str(), "abc1");
    /// ```
    pub fn push(&mut self, ch: char) {
        self.modify(|s| s.push(ch));
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

    /// Removing all graphemes from this text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// let mut text = Text::from("hello");
    /// text.clear();
    ///
    /// assert!(text.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.modify(|s| s.clear())
    }

    /// Extracts a string slice containing the entire `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Text;
    /// let s = Text::from("foo");
    ///
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
        Self { raw: s.into() }
    }
}

impl From<String> for Text {
    /// Converts the given [`String`] into [`Text`].
    ///
    /// No heap allocation is performed, and the string is not copied.
    fn from(s: String) -> Self {
        Self { raw: s.into() }
    }
}

impl From<char> for Text {
    /// Converts [`char`] into [`Text`] with dynamic allocation an owned [`String`].
    fn from(c: char) -> Self {
        String::from(c).into()
    }
}

impl Deref for Text {
    type Target = str;

    /// Immutable access to inner `str`.
    ///
    /// Mutable access to inner `str` through `DerefMut` trait is not possible because `Text` keeps
    /// track of the size of inner string in terminal way: columns and lines. Mutable access to
    /// `str` can invalidate this size in `Text`. The only way to get access to mutable str is
    /// `[Text::modify]` which invalidates the cached size.
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
