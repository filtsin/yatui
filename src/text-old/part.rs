use std::iter::{Fuse, FusedIterator};

use super::{mask::StyleInfo, styled_str::StyledStr, Grapheme, GraphemeIter, Style, Text};

#[derive(Debug, Eq, PartialEq)]
pub enum Part<'a> {
    Str(&'a str, usize, Style),
    NewLine,
}

#[derive(Debug, Clone)]
pub struct PartIter<'a, I, S> {
    data: &'a str,

    graphemes: I,
    styles: S,

    cur_style: Option<StyleInfo>,
    next_grapheme: Option<Grapheme<'a>>,
}

pub fn parts<S: Iterator<Item = StyleInfo>>(
    s: &str,
    mut styles: S,
) -> PartIter<GraphemeIter<'_>, S> {
    let cur_style = styles.next();
    PartIter {
        data: s,
        graphemes: Text::create_graphemes(s),
        styles,
        cur_style,
        next_grapheme: None,
    }
}

impl<'a, I, S> Iterator for PartIter<'a, I, S>
where
    I: Iterator<Item = Grapheme<'a>>,
    S: Iterator<Item = StyleInfo>,
{
    type Item = Part<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_max_part()
    }
}

impl<'a, I, S> FusedIterator for PartIter<'a, I, S>
where
    I: Iterator<Item = Grapheme<'a>>,
    S: Iterator<Item = StyleInfo>,
{
}

impl<'a, I, S> PartIter<'a, I, S>
where
    I: Iterator<Item = Grapheme<'a>>,
    S: Iterator<Item = StyleInfo>,
{
    fn get_max_part(&mut self) -> Option<Part<'a>> {
        // 1. Get first non-zero width grapheme.
        let mut start_grapheme = self.next_grapheme.take().or_else(|| self.graphemes.next())?;

        while start_grapheme.is_zero_width() {
            if start_grapheme.is_new_line() {
                return Some(Part::NewLine);
            } else {
                start_grapheme = self.graphemes.next()?;
            }
        }

        // 2. We could skip some styles because of 0-width graphemes, find new style range
        // for current `start_grapheme`.
        while self.cur_style.filter(|v| v.range.end < start_grapheme.index()).is_some() {
            let old_range = self.cur_style.unwrap();
            self.cur_style = self.styles.next();
            if let Some(style) = self.cur_style {
                assert!(
                    style.range.start > old_range.range.end,
                    "Ranges should be in ascending order and do not overlap"
                );
            }
        }

        // 3. Find maximum range for current `start_grapheme`.
        let (style, range) = if let Some(cur_style) = self.cur_style {
            if start_grapheme.index() < cur_style.range.start {
                (Style::default(), start_grapheme.index() + 1..cur_style.range.start)
            } else {
                (cur_style.style, start_grapheme.index() + 1..cur_style.range.end + 1)
            }
        } else {
            (Style::default(), 0..usize::MAX)
        };

        let mut end_grapheme = start_grapheme.clone();
        let mut width: usize = start_grapheme.width();

        // 4. Check graphemes only in the founded at previous stage. If grapheme is zero width
        // save it to the next iteration and return maximum string slice.
        for _ in range {
            if let Some(cur_grapheme) = self.graphemes.next() {
                if cur_grapheme.is_zero_width() {
                    self.next_grapheme = Some(cur_grapheme);
                    break;
                }
                width += cur_grapheme.width();
                end_grapheme = cur_grapheme;
            } else {
                break;
            }
        }

        let s = &self.data[start_grapheme.start()..=end_grapheme.end()];

        Some(Part::Str(s, width, style))
    }
}
