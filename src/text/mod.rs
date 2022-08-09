mod grapheme;
mod raw_text;
mod style;
mod text_style;

pub use grapheme::Grapheme;
pub use style::{Color, Modifier, Style};
pub use text_style::TextStyle;

use raw_text::RawText;
use unicode_width::UnicodeWidthStr;

use std::{
    borrow::Cow,
    collections::BTreeSet,
    ops::{
        Bound::{self, Excluded, Included, Unbounded},
        Range, RangeBounds, RangeInclusive, RangeTo,
    },
};

use self::grapheme::GraphemeInfo;

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

    /// Return parts of `Text`.
    ///
    /// This method allow iterate over graphemes of this `Text` and change styles at the same time.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// let (graphemes, styles) = text.parts();
    /// ```
    pub fn parts(&mut self) -> (impl Iterator<Item = Grapheme<'_>>, &'_ mut TextStyle) {
        (RawText::create_graphemes(self.raw.as_ref()), &mut self.style)
    }

    /// Appends a given string slice onto the end of this `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.push_str(" world");
    ///
    /// assert_eq!(text.as_ref(), "hello world");
    /// ```
    pub fn push_str(&mut self, string: &str) {
        self.raw.push_str(string);
    }

    /// Remove a `grapheme` from this text at `grapheme_idx` position.
    ///
    /// This is an *O(*n*)* operation.
    ///
    /// # Panics
    ///
    /// Panics if `grapheme_idx` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.remove(0);
    ///
    /// assert_eq!(text.as_ref(), "ello");
    /// ```
    pub fn remove(&mut self, grapheme_idx: usize) {
        self.replace_range(grapheme_idx..=grapheme_idx, "");
    }

    /// Remove the specified `range` in the text, and replaces it with the given string.
    ///
    /// The given string doesn't need to be the same length as the range. Be careful, because
    /// styles don't change. Method looks like [std::replace_range][std::string::String::replace_range]
    /// but `range` in the std points to [`char`] boundaries, but in this method to `grapheme`
    /// boundaries. `range` contains startings point and end point of text graphemes.
    ///
    /// If you want to replace some text with styles clearings look on
    /// [replace_range_polite](Self::replace_range_polite).
    ///
    /// # Panics
    ///
    /// Panics if `range` is out of bounds of text graphemes.
    ///
    /// # Examples
    ///
    /// ```
    /// use yatui::text::{Color, Style, Text};
    ///
    /// let mut text: Text = "hello".into();
    /// text.styles_mut().add(1..=3, Style::new().bg(Color::Red));
    /// text.styles_mut().add(4..=4, Style::new().bg(Color::Yellow));
    /// text.replace_range(1..=3, " new content ");
    ///
    /// assert_eq!(text.as_ref(), "h new content o");
    /// assert_eq!(
    ///     text.styles().iter().collect::<Vec<_>>(),
    ///     vec![(1, 3, Style::new().bg(Color::Red)), (4, 4, Style::new().bg(Color::Yellow))]
    /// );
    /// ```
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        let (g1, g2) = get_graphemes_info(RawText::create_graphemes(self.as_ref()), range);
        self.raw.replace_range(g1.start()..=g2.end(), replace_with);
    }

    /// Remove the specified `range` in the text, and replaces it with the given string.
    /// All styles in the `range` will be removed and shifted so that the old text retains its
    /// styles. Look the difference between this method and [replace_range](Self::replace_range).
    ///
    /// # Panics
    ///
    /// Panics if `range` is out of bounds of text graphemes.
    ///
    /// # Examples
    ///
    /// ```
    /// use yatui::text::{Color, Style, Text};
    ///
    /// let mut text: Text = "hello".into();
    /// text.styles_mut().add(1..=3, Style::new().bg(Color::Red));
    /// text.styles_mut().add(4..=4, Style::new().bg(Color::Yellow));
    /// text.replace_range_polite(1..=3, " new content ");
    ///
    /// assert_eq!(text.as_ref(), "h new content o");
    /// assert_eq!(
    ///     text.styles().iter().collect::<Vec<_>>(),
    ///     vec![(14, 14, Style::new().bg(Color::Yellow))]
    /// );
    /// // So the grapheme 'o' saved style after replacing string
    /// ```
    pub fn replace_range_polite<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        let (g1, g2) = get_graphemes_info(RawText::create_graphemes(self.as_ref()), range);
        self.raw.replace_range(g1.start()..=g2.end(), replace_with);
        self.style.remove(g1.index()..=g2.index());

        let old_len = g2.index() - g1.index() + 1;
        let new_len = UnicodeWidthStr::width(replace_with);

        if old_len > new_len {
            self.styles_mut().negative_shift(g2.index().checked_add(1).unwrap(), old_len - new_len);
        } else {
            self.styles_mut().positive_shift(g2.index().checked_add(1).unwrap(), new_len - old_len);
        }
    }

    /// Removes the last `grapheme` from the text.
    ///
    /// If the text is empty it is noop. Styles remain untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    ///
    /// let mut text: Text = "foo".into();
    /// text.pop();
    ///
    /// assert_eq!(text.as_ref(), "fo");
    /// ```
    pub fn pop(&mut self) {
        if let Some(g) = RawText::create_graphemes(self.as_ref()).last() {
            let info = g.info();
            self.raw.replace_range(info.start()..=info.end(), "");
        }
    }

    /// Remove all matches of `pat` in the `Text`.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn remove_matches(&mut self, pat: &str) {
        todo!()
    }

    /// Retains only the `graphemes` specified by the predicate.
    ///
    /// In other words, remove all graphemes `c` such that `f(c)` returns false. It is the same
    /// method like in std [std::retain](std::string::String::retain) but operates with graphemes
    /// instead of chars.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&str) -> bool {
            todo!()

        }

    /// Split the `Text` into two at the given `grapheme` index.
    ///
    /// Returns a newly allocated `Text`. `self` contains graphemes `[0, at]`, and the returned
    /// `Text` contains graphemes `[at, len).
    ///
    /// # Panics
    ///
    /// Panics if `at` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn split_off(&mut self, at: usize) -> Text {
        todo!()
    }

    /// Shortens this `Text` to the specified length in graphemes.
    ///
    /// If `new_len` is greater that the current length this is no-op. Possibly you want to use
    /// [truncate_columns](Self::truncate_columns).
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn truncate(&mut self, new_len: usize) {
        todo!()
    }

    /// Shortens this `Text` to the specified lines count.
    ///
    /// If `new_lines` is greater than the current lines count this is no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn truncate_lines(&mut self, new_lines: usize) {
        todo!()
    }

    /// Shortens this `Text` to the specified columns count.
    ///
    /// If `new_columns` is greater than the current max columns this is no-op. 
    /// This method can change multiple lines of `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn truncate_columns(&mut self, new_columns: usize) {
        todo!()
    }

    /// Returns the length of this `Text` in `graphemes`.
    /// It is *O(*n*)* operation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "y\u{0306}ö老п\r\n".into();
    /// assert_eq!(text.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        RawText::create_graphemes(self.as_ref()).count()
    }

    /// Check description for [std::reserve](std::string::String::reserve).
    ///
    /// If the `Text` is borrowed it will be transformed to owned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// assert!(text.is_borrowed());
    /// assert_eq!(text.capacity(), 0);
    ///
    /// text.reserve(10);
    /// assert!(text.is_owned());
    /// assert_eq!(text.capacity(), 15);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.raw.reserve(additional);
    }

    /// Check description for [std::reserve_exact](std::string::String::reserve_exact).
    ///
    /// If the `Text` is borrowed it will be transformed to owned.
    /// 
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// assert!(text.is_borrowed());
    /// assert_eq!(text.capacity(), 0);
    ///
    /// text.reserve_exact(10);
    /// assert!(text.is_owned());
    /// assert_eq!(text.capacity(), 15);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.raw.reserve_exact(additional);
    }

    /// Shrinks the capacity of this `Text` to match its length. If the `Text` is borrowed
    /// this is no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.shrink_to_fit();
    /// assert_eq!(text.capacity(), 0);
    ///
    /// text.reserve(100);
    /// text.shrink_to_fit();
    /// assert_eq!(text.capacity(), 5);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.raw.shrink_to_fit();
    }

    /// Shrinks the capacity of this `Text` with a lower bound.
    ///
    /// If the `Text` is borrowed this is a no-op. Check description for the same
    /// [std::shrink_to](std::string::String::shrink_to).
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.shrink_to(10);
    /// assert_eq!(text.capacity(), 0);
    ///
    /// text.reserve(100);
    /// assert!(text.capacity() >= 100);
    ///
    /// text.shrink_to(10);
    /// assert!(text.capacity() >= 10);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.raw.shrink_to(min_capacity);
    }

    /// Returns this `Text`'s capacity, in bytes. If the `Text` is borrowed it will return 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "h".into();
    /// assert_eq!(text.capacity(), 0);
    ///
    /// let mut text: Text = String::with_capacity(10).into();
    /// assert_eq!(text.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.raw.capacity()
    }

    /// Removing all graphemes from the text. Styles remain untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::{Text, Color, Style};
    /// let mut text: Text = "hello".into();
    /// text.styles_mut().add(.., Style::new().bg(Color::Red));
    /// text.clear();
    ///
    /// assert!(text.is_empty());
    /// assert!(!text.styles().is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.raw.clear();
    }

    /// Removing all graphemes and styles from the text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::{Text, Color, Style};
    /// let mut text: Text = "hello".into();
    /// text.styles_mut().add(.., Style::new().bg(Color::Red));
    /// text.clear_all();
    ///
    /// assert!(text.is_empty());
    /// assert!(text.styles().is_empty());
    /// ```
    pub fn clear_all(&mut self) {
        self.raw.clear();
        self.style.clear();
    }

    /// Styles of this `Text`
    pub fn styles(&self) -> &TextStyle {
        &self.style
    }

    /// Styles of this `Text`
    pub fn styles_mut(&mut self) -> &mut TextStyle {
        &mut self.style
    }

    /// Count of lines in text. It is *O(*1*)* operation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let text: Text = "line1\nline2\r\nline3".into();
    /// assert_eq!(text.lines(), 3);
    /// ```
    pub fn lines(&self) -> usize {
        self.raw.lines()
    }

    /// Max count of columns (in the terminal way) in all lines in the text.
    /// It is *O(*1*)* operation.
    ///
    /// Result of this function is not equal to graphemes count, because some graphemes
    /// can fill more than 1 column and some having zero width (control characters). 
    /// For example `ö` have width equal 1, but `老` have width 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let text: Text = "c1\n老hello\r\ntext".into();
    /// assert_eq!(text.columns(), 7);
    /// ```
    pub fn columns(&self) -> usize {
        self.raw.columns()
    }

    /// Returns `true` if this text has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let text: Text = "".into();
    /// assert!(text.is_empty());
    ///
    /// let text: Text = "h".into();
    /// assert!(!text.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.raw.as_ref().len() == 0
    }

    /// Returns `true` if text content is borrowed and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// assert!(text.is_borrowed());
    ///
    /// text.remove(0);
    /// assert!(!text.is_borrowed());
    /// ```
    pub fn is_borrowed(&self) -> bool {
        self.raw.is_borrowed()
    }

    /// Returns `true` if text content is owned and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// assert!(!text.is_owned());
    ///
    /// text.remove(0);
    /// assert!(text.is_owned());
    /// ```
    pub fn is_owned(&self) -> bool {
        self.raw.is_owned()
    }
}

fn get_graphemes_info<'a, I, R>(iter: I, range: R) -> (GraphemeInfo, GraphemeInfo)
where
    I: IntoIterator<Item = Grapheme<'a>>,
    R: RangeBounds<usize>,
{
    let (g1, g2) = iter_bound(iter, range);
    (g1.info(), g2.info())
}

fn iter_bound<I, R, K>(iter: I, range: R) -> (K, K)
where
    I: IntoIterator<Item = K>,
    R: RangeBounds<usize>,
    K: Clone,
{
    let mut iter = iter.into_iter();

    let start_idx = match range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n.checked_add(1).unwrap(),
        Unbounded => 0,
    };

    let left = iter.nth(start_idx).unwrap();

    let right = if range.end_bound() == Unbounded {
        iter.last().unwrap()
    } else {
        let end_idx = match range.end_bound() {
            Included(&n) => n,
            Excluded(&n) => n.checked_sub(1).unwrap(),
            Unbounded => unreachable!(),
        };

        assert!(start_idx <= end_idx);

        if end_idx == start_idx { left.clone() } else { iter.nth(end_idx - start_idx - 1).unwrap() }
    };

    (left, right)
}

fn bound_to_range<R: RangeBounds<usize>>(range: R) -> RangeInclusive<usize> {
    let start = match range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n.checked_add(1).unwrap(),
        Unbounded => 0,
    };

    let end = match range.end_bound() {
        Included(&n) => n,
        Excluded(&n) => n.checked_sub(1).unwrap(),
        Unbounded => usize::MAX,
    };

    assert!(start <= end);

    start..=end
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
