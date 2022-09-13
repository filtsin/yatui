use std::iter::{Fuse, FusedIterator};

use super::{
    mask::StyleInfo, styled_str::StyledStr, Grapheme, GraphemeIter, GraphemeWidth, Style, Text,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Part<'a> {
    Str(&'a str, usize, Style),
    NewLine,
}

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
        let mut start_grapheme = self.next_grapheme.take().or_else(|| self.graphemes.next())?;

        // 1. Get first non-zero width grapheme
        while start_grapheme.width() == GraphemeWidth::Zero {
            if Self::is_new_line(&start_grapheme) {
                return Some(Part::NewLine);
            } else {
                start_grapheme = self.graphemes.next()?;
            }
        }

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

        let mut count: usize = start_grapheme.width().into();

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

        for _ in range {
            if let Some(cur_grapheme) = self.graphemes.next() {
                let width = cur_grapheme.width();

                if width == GraphemeWidth::Zero {
                    self.next_grapheme = Some(cur_grapheme);
                    break;
                }

                count += width.num();
                end_grapheme = cur_grapheme;
            } else {
                break;
            }
        }

        let s = &self.data[start_grapheme.start()..=end_grapheme.end()];

        Some(Part::Str(s, count, style))
    }

    fn is_new_line(g: &Grapheme<'_>) -> bool {
        g.data() == "\n" || g.data() == "\r\n"
    }
}
