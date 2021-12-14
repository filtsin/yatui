pub mod common;

use std::ops::{Deref, DerefMut};

use crate::terminal::region::Region;
use common::{Child, CommonLayout, DefaultLayout};

// Layout is just `Widget` for another Widgets. It is known how to place it's childs
pub struct Layout {
    direction: LayoutDirection,

    inner: DefaultLayout<Layout>,
}

#[derive(Debug)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

impl Layout {
    pub fn horizontal() -> Self {
        Layout { direction: LayoutDirection::Horizontal, inner: DefaultLayout::default() }
    }

    pub fn vertical() -> Self {
        Layout { direction: LayoutDirection::Vertical, inner: DefaultLayout::default() }
    }
}

impl Deref for Layout {
    type Target = DefaultLayout<Layout>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Layout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl CommonLayout for Layout {
    fn layout(region: Region, childs: &mut [Child]) {
        todo!()
    }
}
