use crate::terminal::{Index, Size};

use super::{
    mask::{Iter, StyleInfo},
    Text,
};

pub trait StyledStr {
    type Iter: IntoIterator<Item = StyleInfo>;

    fn str(&self) -> &str;
    fn styles_iter(&self) -> Self::Iter;
    fn size(&self) -> Size;
}

impl StyledStr for &str {
    type Iter = std::iter::Empty<StyleInfo>;

    fn str(&self) -> &str {
        self
    }

    fn styles_iter(&self) -> Self::Iter {
        std::iter::empty()
    }

    fn size(&self) -> Size {
        use crate::text::raw_text::RawText;
        let raw_size = RawText::compute_size(self);
        let width = raw_size.columns.try_into().unwrap_or(Index::MAX);
        let height = raw_size.lines.try_into().unwrap_or(Index::MAX);
        Size::new(width, height)
    }
}

impl<'a> StyledStr for &'a Text {
    type Iter = Iter<'a>;

    fn str(&self) -> &str {
        self.as_str()
    }

    fn styles_iter(&self) -> Self::Iter {
        self.mask().iter()
    }

    fn size(&self) -> Size {
        let width = self.columns().try_into().unwrap_or(Index::MAX);
        let height = self.lines().try_into().unwrap_or(Index::MAX);
        Size::new(width, height)
    }
}
