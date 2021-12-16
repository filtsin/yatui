use crate::{layout::wrapper::LayoutWrapper, terminal::region::Region};

use super::{Child, Layout, LayoutDirection, LayoutInfo};

// TODO: change name
pub struct AutoLayoutInner {
    direction: LayoutDirection,
}

pub type AutoLayout = LayoutWrapper<AutoLayoutInner>;

impl AutoLayout {
    pub fn horizontal() -> Self {
        let inner = AutoLayoutInner { direction: LayoutDirection::Horizontal };
        Self::new(inner)
    }

    pub fn vertical() -> Self {
        let inner = AutoLayoutInner { direction: LayoutDirection::Vertical };
        Self::new(inner)
    }
}

impl Layout for AutoLayoutInner {
    fn layout(&self, region: Region, info: LayoutInfo) {
        todo!()
    }
}
