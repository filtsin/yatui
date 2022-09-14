mod grapheme;
pub mod mask;
pub mod part;
mod raw_text;
mod style;
mod styled_str;
pub(crate) mod utils;

pub use grapheme::Grapheme;
pub use mask::Mask;
pub use style::{Color, Modifier, Style};
pub use styled_str::StyledStr;

use raw_text::RawText;
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;
use utils::get_graphemes_info;

use std::{
    borrow::{Borrow, Cow},
    cmp::{Eq, PartialEq},
    iter::Enumerate,
    ops::{Add, AddAssign, Index, RangeBounds},
};

use self::{grapheme::GraphemeInfo, mask::StyleInfo};

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Text {
    raw: RawText,
    mask: Mask,
}

#[derive(Clone)]
pub struct GraphemeIter<'a> {
    inner: Enumerate<GraphemeIndices<'a>>,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        self.raw.as_str()
    }

    /// Return parts of `Text`.
    ///
    /// This method allow iterate over graphemes of this `Text` and change mask at the same time.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// let (graphemes, mask) = text.parts();
    /// ```
    pub fn parts(&mut self) -> (GraphemeIter<'_>, &'_ mut Mask) {
        (Self::create_graphemes(self.raw.as_str()), &mut self.mask)
    }

    /// Return iterator over graphemes for this `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let text: Text = "y\u{0306}ello老".into();
    ///
    /// assert_eq!(text.graphemes().collect::<Vec<_>>(), vec!["y\u{0306}", "e", "l", "l", "o", "老"]);
    /// ```
    pub fn graphemes(&self) -> GraphemeIter<'_> {
        Self::create_graphemes(self.as_str())
    }

    /// Modify text in place with a given closure. Closure can return value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.modify(|string| {
    ///     *string = string.replace("el", "i ");
    ///     string.push_str("ok");
    ///     string.make_ascii_uppercase();
    /// });
    /// assert_eq!(text.as_str(), "HI LOOK");
    /// assert_eq!(text.columns(), 7);
    /// ```
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hi".into();
    /// let vec: Vec<u8> = text.modify(|string| std::mem::take(string).into_bytes());
    ///
    /// assert_eq!(text.as_str(), "");
    /// assert_eq!(vec, [104, 105]);
    /// ```
    pub fn modify<F, R>(&mut self, mut f: F) -> R
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
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.push_str(" world");
    ///
    /// assert_eq!(text.as_str(), "hello world");
    /// ```
    pub fn push_str(&mut self, s: &str) {
        self.modify(|string| {
            string.push_str(s);
        });
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
    /// assert_eq!(text.as_str(), "ello");
    /// ```
    pub fn remove(&mut self, grapheme_idx: usize) {
        self.replace_range(grapheme_idx..=grapheme_idx, "");
    }

    /// Remove the specified `range` in the text, and replaces it with the given string.
    ///
    /// The given string doesn't need to be the same length as the range. Be careful, because
    /// mask don't change. Method looks like
    /// [std::replace_range][std::string::String::replace_range]
    /// but `range` in the std points to [`char`] boundaries, but in this method to `grapheme`
    /// boundaries. `range` contains startings point and end point of text graphemes.
    ///
    /// If you want to replace some text with mask look on
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
    /// text.mask_mut().add(1..=3, Style::new().bg(Color::Red));
    /// text.mask_mut().add(4..=4, Style::new().bg(Color::Yellow));
    /// text.replace_range(1..=3, " new content ");
    ///
    /// assert_eq!(text.as_str(), "h new content o");
    /// assert_eq!(
    ///     text.mask().clone().into_vec(),
    ///     vec![(1..=3, Style::new().bg(Color::Red)), (4..=4, Style::new().bg(Color::Yellow))]
    /// );
    /// ```
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        let (g1, g2) = get_graphemes_info(self.graphemes(), range);
        self.modify(|string| {
            string.replace_range(g1.bytes_to(g2), replace_with);
        });
    }

    /// Remove the specified `range` in the text, and replaces it with the given string.
    /// All styles in mask in the `range` will be removed and shifted so that the old text retains
    /// its styles. Look the difference between this method
    /// and [replace_range](Self::replace_range).
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
    /// text.mask_mut().add(1..=3, Style::new().bg(Color::Red));
    /// text.mask_mut().add(4..=4, Style::new().bg(Color::Yellow));
    /// text.replace_range_polite(1..=3, " new content ");
    ///
    /// assert_eq!(text.as_str(), "h new content o");
    /// assert_eq!(text.mask().clone().into_vec(), vec![(14..=14, Style::new().bg(Color::Yellow))]);
    /// // So the grapheme 'o' saved style after replacing string because it is not in range.
    /// ```
    pub fn replace_range_polite<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        let (g1, g2) = get_graphemes_info(self.graphemes(), range);

        self.modify(|string| {
            string.replace_range(g1.bytes_to(g2), replace_with);
        });

        self.mask_mut().remove(g1.index()..=g2.index());

        let old_len = g2.index() - g1.index() + 1;
        let new_len = Self::create_graphemes(replace_with).count();
        let range = g2.index() + 1..;

        if old_len < new_len {
            self.mask_mut().shift_add(range, new_len - old_len);
        } else {
            self.mask_mut().shift_sub(range, old_len - new_len);
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
    /// let mut text: Text = "foo".into();
    /// text.pop();
    ///
    /// assert_eq!(text.as_str(), "fo");
    /// ```
    pub fn pop(&mut self) {
        if let Some(g) = self.graphemes().last() {
            let info = g.info();
            self.modify(|string| {
                string.replace_range(info.bytes_range(), "");
            });
        }
    }

    /// Appends the given `char` to the end of this `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "foo".into();
    /// text.push('1');
    ///
    /// assert_eq!(text.as_str(), "foo1");
    /// ```
    pub fn push(&mut self, c: char) {
        self.modify(|string| {
            string.push(c);
        });
    }

    /// Inserts a string slice into this `Text` at `grapheme` index.
    ///
    /// # Panics
    ///
    /// Panics if `grapheme_idx` is larger than the `Text` len (in graphemes).
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "老y\u{0306}f".into();
    /// text.insert_str(2, "bar");
    ///
    /// assert_eq!(text.as_str(), "老y\u{0306}barf");
    /// ```
    pub fn insert_str(&mut self, grapheme_idx: usize, s: &str) {
        self.modify(|string| {
            let offset = if grapheme_idx == 0 {
                0
            } else {
                Self::create_graphemes(string).nth(grapheme_idx - 1).unwrap().info().end() + 1
            };

            string.insert_str(offset, s);
        });
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
    /// # use yatui::text::Text;
    /// let mut text: Text = "y\u{0306} - is not y".into();
    /// text.retain(|s| s != "y");
    ///
    /// assert_eq!(text.as_str(), "y\u{0306} - is not ");
    /// ```
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "y\u{0306}老🧡пh".into();
    /// let keep = [true, false, false, true, true];
    /// let mut iter = keep.iter();
    /// text.retain(|_| *iter.next().unwrap());
    ///
    /// assert_eq!(text.as_str(), "y\u{0306}пh");
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&str) -> bool,
    {
        let mut vec = self.modify(|string| std::mem::take(string).into_bytes());

        let mut byte_pos = 0;
        let mut shift = 0;

        while let Some(g) = {
            // Safety: we moving by graphemes in valid UTF-8 string so it is valid UTF-8
            let str = unsafe { std::str::from_utf8_unchecked(&vec[byte_pos..]) };
            Self::create_graphemes(str).next()
        } {
            let result = f(g.data());
            let g = g.info();

            if !result {
                shift += g.len();
            } else if shift > 0 {
                for i in 0..g.len() {
                    vec[byte_pos - shift + i] = vec[byte_pos + i];
                }
            }
            byte_pos += g.len();
        }

        // Safety: new len is less than capacity, all elements are initialized. Vec is valid
        // UTF-8 because old string was valid UTF-8 and all we did was move some graphemes.
        unsafe {
            vec.set_len(byte_pos - shift);
            self.modify(|string| *string = String::from_utf8_unchecked(vec));
        };
    }

    /// Split the `Text` into two at the given `grapheme` index.
    ///
    /// Returns a newly allocated `Text`. `self` contains graphemes `[0, at)`, and the returned
    /// `Text` contains graphemes `[at, len)`. This method is polite. This means
    /// that new `Text` will save his styles like in the original `Text`.
    /// Wherein original mask of `self` doesn't change.
    ///
    /// # Panics
    ///
    /// Panics if `at` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::{Text, Style, Color};
    /// let mut text: Text = "foobar".into();
    /// text.mask_mut().add(3..=5, Style::new().fg(Color::Red)); // bar is red
    /// let mut new_text = text.split_off_polite(3);
    ///
    /// assert_eq!(text.as_str(), "foo");
    /// assert_eq!(new_text.as_str(), "bar");
    /// assert_eq!(text.mask().clone().into_vec(), vec![(3..=5, Style::new().fg(Color::Red))]);
    /// assert_eq!(new_text.mask().clone().into_vec(), vec![(0..=2, Style::new().fg(Color::Red))]);
    /// ```
    pub fn split_off_polite(&mut self, at: usize) -> Text {
        let mut mask = self.mask.clone();
        mask.remove(..at);
        mask.shift_sub(at.., at);

        let g = self.graphemes().nth(at).unwrap().info();

        let new_string: String = self.modify(|string| string.split_off(g.start()));
        Text { raw: new_string.into(), mask }
    }

    /// Shortens this `Text` to the specified length in graphemes.
    ///
    /// If `new_len` is greater that the current length this is no-op. Possibly you want to use
    /// [truncate_columns](Self::truncate_columns). Styles remain untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "y\u{0306}ellow".into();
    /// text.truncate(1);
    ///
    /// assert_eq!(text.as_str(), "y\u{0306}");
    /// ```
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "hello".into();
    /// text.truncate(100);
    ///
    /// assert_eq!(text.as_str(), "hello");
    /// ```
    pub fn truncate(&mut self, new_len: usize) {
        if let Some(pos) = self.graphemes().nth(new_len).map(|g| g.info().start()) {
            self.modify(|string| {
                string.truncate(pos);
            });
        }
    }

    /// Shortens this `Text` to the specified lines count.
    ///
    /// If `new_lines` is greater than the current lines count this is no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "foo\nbar\r\n!".into();
    /// text.truncate_lines(1);
    ///
    /// assert_eq!(text.as_str(), "foo");
    /// ```
    pub fn truncate_lines(&mut self, new_lines: usize) {
        // <= because we want to remove \n in the end of text if it is present there.
        if new_lines <= self.lines() {
            let mut bytes = 0;
            let mut lines = 0;

            for g in self.graphemes() {
                if g == "\n" || g == "\r\n" {
                    lines += 1;
                }

                if lines == new_lines {
                    break;
                }

                bytes += g.len();
            }

            // TODO: We do not need to recalculate columns and lines, update lines manually
            self.modify(|string| {
                string.truncate(bytes);
            });
        }
    }

    /// Shortens this `Text` to the specified columns count.
    ///
    /// If `new_columns` is greater than the current max columns this is no-op.
    /// This method can change multiple lines of `Text`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "text\ntext\r\ntext very big".into();
    /// text.truncate_columns(4);
    ///
    /// assert_eq!(text.as_str(), "text\ntext\r\ntext");
    /// ```
    pub fn truncate_columns(&mut self, new_columns: usize) {
        if new_columns < self.columns() {
            let mut count = 0;
            self.retain(|g| {
                count += 1;

                if g == "\n" || g == "\r\n" {
                    count = 0;
                }

                count <= new_columns
            });
        }
    }

    /// Returns the length of this `Text` in `graphemes`.
    /// It is *O(*n*)* operation!
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::Text;
    /// let mut text: Text = "y\u{0306}ö老п\r\n".into();
    /// assert_eq!(text.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.graphemes().count()
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
        self.modify(|string| {
            string.reserve(additional);
        });
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
        self.modify(|string| {
            string.reserve_exact(additional);
        });
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
    /// text.mask_mut().add(.., Style::new().bg(Color::Red));
    /// text.clear();
    ///
    /// assert!(text.is_empty());
    /// assert!(!text.mask_mut().is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.modify(|string| {
            string.clear();
        });
    }

    /// Removing all graphemes and mask from the text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui::text::{Text, Color, Style};
    /// let mut text: Text = "hello".into();
    /// text.mask_mut().add(.., Style::new().bg(Color::Red));
    /// text.clear_all();
    ///
    /// assert!(text.is_empty());
    /// assert!(text.mask().is_empty());
    /// ```
    pub fn clear_all(&mut self) {
        self.clear();
        self.mask_mut().clear();
    }

    /// Mask of this `Text`
    pub fn mask(&self) -> &Mask {
        &self.mask
    }

    /// Mask of this `Text`
    pub fn mask_mut(&mut self) -> &mut Mask {
        &mut self.mask
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
        self.raw.as_str().len() == 0
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

    pub fn create_graphemes(s: &str) -> GraphemeIter<'_> {
        GraphemeIter::new(UnicodeSegmentation::grapheme_indices(s, true).enumerate())
    }
}

impl<'a> GraphemeIter<'a> {
    fn new(inner: Enumerate<GraphemeIndices<'a>>) -> Self {
        Self { inner }
    }
}

impl<'a> Iterator for GraphemeIter<'a> {
    type Item = Grapheme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Grapheme::new)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Text {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<&'static str> for Text {
    fn from(s: &'static str) -> Self {
        Self { raw: s.into(), ..Self::default() }
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self { raw: s.into(), ..Self::default() }
    }
}

impl From<char> for Text {
    fn from(c: char) -> Self {
        String::from(c).into()
    }
}

impl Add<&str> for Text {
    type Output = Text;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.push_str(rhs);
        self
    }
}

impl AddAssign<&str> for Text {
    fn add_assign(&mut self, rhs: &str) {
        self.push_str(rhs);
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

impl std::str::FromStr for Text {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_owned().into())
    }
}

/// Implements the [] on the `Text`.
///
/// It works with `graphemes` in current `Text` and not bytes or chars.
/// This is an *O(*n*)* operation.
///
/// # Examples
///
/// ```
/// # use yatui::text::Text;
/// let text: Text = "老y\u{0306}\r\n".into();
///
/// assert_eq!(&text[0..1], "老");
/// assert_eq!(&text[1..2], "y\u{0306}");
/// assert_eq!(&text[2..3], "\r\n");
/// // let _ = &text[3..4]; <-- panic here
/// ```
impl<R> Index<R> for Text
where
    R: RangeBounds<usize>,
{
    type Output = str;

    fn index(&self, index: R) -> &Self::Output {
        let (g1, g2) = get_graphemes_info(self.graphemes(), index);
        &self.as_str()[g1.start()..=g2.end()]
    }
}

macro_rules! impl_trait_wrapper {
    (extend, [$($type:ty $(,)?)*]) => {
        $(
            #[allow(clippy::extra_unused_lifetimes)]
            impl<'a> Extend<$type> for Text {
                fn extend<T: IntoIterator<Item = $type>>(&mut self, iter: T) {
                    self.modify(|string| string.extend(iter));
                }
            }
        )*
    };

    (from_iterator, [$($type:ty $(,)?)*]) => {
        $(
            #[allow(clippy::extra_unused_lifetimes)]
            impl<'a> FromIterator<$type> for Text {
                fn from_iter<T: IntoIterator<Item = $type>>(iter: T) -> Self {
                    let mut result = Self::new();
                    result.extend(iter);
                    result
                }
            }
        )*
    };

    (partial_eq, [$($type:ty $(,)?)*]) => {
        $(
            #[allow(clippy::extra_unused_lifetimes)]
            impl<'a> PartialEq<$type> for Text {
                fn eq(&self, other: &$type) -> bool {
                    self.as_str() == *other
                }
            }

            #[allow(clippy::extra_unused_lifetimes)]
            impl<'a> PartialEq<Text> for $type {
                fn eq(&self, other: &Text) -> bool {
                    *self == other.as_str()
                }
            }
        )*
    };
}

impl_trait_wrapper!(
    extend,
    [
        &'a char,
        &'a str,
        Box<str>,
        Cow<'a, str>,
        String,
        char
    ]
);

impl_trait_wrapper!(
    from_iterator,
    [
        &'a char,
        &'a str,
        Box<str>,
        Cow<'a, str>,
        String,
    ]
);

impl_trait_wrapper!(
    partial_eq,
    [
        &'a str,
        Cow<'a, str>,
        String,
    ]
);
