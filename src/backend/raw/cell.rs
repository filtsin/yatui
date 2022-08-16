use crate::text::{Modifier, Style};

use bitflags::bitflags;
use compact_str::CompactString;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub(crate) struct Cell {
    pub grapheme: CompactString,
    pub style: Style,
    pub flags: Flag,
}

bitflags! {
    #[derive(Default)]
    pub(crate) struct Flag : u16 {
        const WIDE_START = 0b0000000000000001;
        const WIDE_END = 0b0000000000000010;
        const WIDE_LEADING = 0b0000000000000100;
    }
}

impl Cell {
    pub fn new(s: &str, style: Style) -> Self {
        assert_eq!(UnicodeSegmentation::graphemes(s, true).count(), 1);

        Self { grapheme: s.into(), style, flags: Flag::empty() }
    }

    pub fn wide_end() -> Self {
        Self { grapheme: " ".into(), style: Style::default(), flags: Flag::WIDE_END }
    }

    pub fn wide_leading() -> Self {
        Self { grapheme: " ".into(), style: Style::default(), flags: Flag::WIDE_LEADING }
    }

    pub fn new_str(s: &str) -> Self {
        Self::new(s, Style::default())
    }
}
