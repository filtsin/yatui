use std::fmt::Debug;

use crate::{
    component::{Component, WidgetSize},
    compositor::context::Context,
    terminal::{cursor::Cursor, region::Region},
};

#[derive(Debug)]
pub struct Child {
    pub(crate) component: Component,
    size: WidgetSize,
    pub(crate) region: Region,
}

impl Child {
    pub fn new(component: Component) -> Self {
        Self {
            component,
            size: WidgetSize::min(),
            region: Region::new(Cursor::new(0, 0), Cursor::new(0, 0)),
        }
    }

    pub fn size(&self) -> WidgetSize {
        self.size
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = region;
    }

    pub(crate) fn update_size(&mut self, new_size: WidgetSize) {
        self.size = new_size;
    }
}
