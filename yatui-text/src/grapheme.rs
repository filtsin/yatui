//! Grapheme is sequence of one or more "code points in unicode".
//!
//! In simple terms, a grapheme is what humans perceive as single character in text, which might
//! be
//! - A single "Unicode code point" (like english 'a' or russian 'п')
//! - A combination of "Unicode code points" (like 'ý' written as 'y' + combining acute accent [U+0301])
//!
//! In terminal grapheme typically occupy 1 column width, but some (like emoji) may use 2 columns.
//! Typically grapheme treaded as single unit for cursor movement. Some "control" graphemes occupy
//! 0 columns and does not render in terminal at all (e.g., '\r\n' - is new line)
//!
//! -------
//!
//! You can open this file in any terminal you want (render may be different):
//!
//! - These graphemes occupy 1 column width: a b п ý
//! - These graphemes occupy 2 column width: 虎 🚀

use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// This is a lightweight wrapper around `&str` because "grapheme" does not have upper limit for
/// how many code points: they may contain multiple "Unicode code points" (e.g. a lot of `\u{200d}`
/// [Zero width joiner]), `char` can only represent one "Unicode code point".
///
/// Read what is grapheme in [module documentation](crate::grapheme)
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Grapheme<'a> {
    data: &'a str,
}

// write a simple text
//

impl<'a> Grapheme<'a> {
    /// Creates a new grapheme without any checks.
    /// Internally used while iterating over `unicode_segmentation` iterator so double check is not necessary.
    pub(crate) fn new(data: &'a str) -> Self {
        debug_assert_eq!(UnicodeSegmentation::graphemes(data, true).count(), 1);
        Self { data }
    }

    /// Get a width of this grapheme: in terminal way it how many columns occupied: 0, 1 or 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Grapheme;
    /// let english: Grapheme = "b".try_into().unwrap();
    /// let russian: Grapheme = "п".try_into().unwrap();
    /// let japanese: Grapheme = "ひ".try_into().unwrap();
    /// let control: Grapheme = "\r\n".try_into().unwrap();
    /// assert_eq!(english.width(), 1);
    /// assert_eq!(russian.width(), 1);
    /// assert_eq!(japanese.width(), 2); // CJK occupies 2 columns
    /// assert_eq!(control.width(), 0);
    /// ```
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self.as_ref())
    }

    /// Returns `true` if this grapheme is new line control character (`\n` or `\r\n`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Grapheme;
    /// assert!(Grapheme::try_from("\n").unwrap().is_newline());
    /// assert!(Grapheme::try_from("\r\n").unwrap().is_newline());
    /// assert!(!Grapheme::try_from("b").unwrap().is_newline());
    /// ```
    pub fn is_newline(&self) -> bool {
        let v = 2;
        self.data == "\n" || self.data == "\r\n"
    }

    /// Returns `true` if this grapheme occupy 0 columns of terminal (e.g. control characters).
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::Grapheme;
    /// let g: Grapheme = "b".try_into().unwrap();
    /// assert!(!g.is_transparent());
    /// ```
    ///
    /// ```
    /// # use yatui_text::Grapheme;
    /// let g: Grapheme = "\r\n".try_into().unwrap();
    /// assert!(g.is_transparent());
    /// ```
    pub fn is_transparent(&self) -> bool {
        self.width() == 0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TryFromStrError {
    EmptyStr,
    TooManyGraphemes,
}

impl Display for TryFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromStrError::EmptyStr => Display::fmt("Empty string is not a valid grapheme", f),
            TryFromStrError::TooManyGraphemes => Display::fmt("Too many graphemes in argument", f),
        }
    }
}

impl std::error::Error for TryFromStrError {}

impl<'a> TryFrom<&'a str> for Grapheme<'a> {
    type Error = TryFromStrError;

    /// Try to create grapheme from `value`. If `value` is empty or contains more than 1 grapheme
    /// returns the error If `value` is empty or contains more than 1 grapheme returns the error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::{Grapheme, grapheme::TryFromStrError};
    /// assert!(Grapheme::try_from("b").is_ok());
    /// assert_eq!(Grapheme::try_from(""), Err(TryFromStrError::EmptyStr));
    /// assert_eq!(Grapheme::try_from("bb"), Err(TryFromStrError::TooManyGraphemes));
    /// ```
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut graphemes = UnicodeSegmentation::graphemes(value, true);
        let grapheme = graphemes.next();

        if graphemes.as_str().is_empty() {
            grapheme.map(|data| Self { data }).ok_or(TryFromStrError::EmptyStr)
        } else {
            Err(TryFromStrError::TooManyGraphemes)
        }
    }
}

impl AsRef<str> for Grapheme<'_> {
    fn as_ref(&self) -> &str {
        self.data
    }
}

impl std::ops::Deref for Grapheme<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl PartialEq<&str> for Grapheme<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.data == *other
    }
}
