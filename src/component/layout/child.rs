use std::fmt::Debug;

use crate::{
    component::Component,
    compositor::context::Context,
    terminal::{cursor::Cursor, region::Region, size::Size},
};

#[derive(Debug)]
pub struct Child {
    pub(crate) component: Component,
    size: Size,
    pub(crate) region: Region,
}

impl Child {
    pub fn new(component: Component) -> Self {
        Self {
            component,
            size: Size::min(),
            region: Region::new(Cursor::new(0, 0), Cursor::new(0, 0)),
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = region;
    }

    pub(crate) fn update_size(&mut self, new_size: Size) {
        self.size = new_size;
    }
}
