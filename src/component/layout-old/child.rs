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
    pub(crate) region: Option<Region>,
}

impl Child {
    pub fn new(component: Component) -> Self {
        Self { component, size: Size::min(), region: None }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn region(&self) -> Option<Region> {
        self.region
    }

    pub fn update_region(&mut self, region: Option<Region>) {
        self.region = region;
    }

    pub(crate) fn update_size(&mut self, new_size: Size) {
        self.size = new_size;
    }
}
