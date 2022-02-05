use crate::{
    component::{size_hint::SizeHint, Component},
    compositor::context::Context,
    terminal::{cursor::Cursor, region::Region},
};

pub struct Child {
    pub(crate) component: Component,
    size: SizeHint,
    region: Region,
}

impl Child {
    pub fn new(component: Component) -> Self {
        Self {
            component,
            size: SizeHint::zero(),
            region: Region::new(Cursor::new(0, 0), Cursor::new(0, 0)),
        }
    }

    pub fn size(&self) -> SizeHint {
        self.size
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = region;
    }

    pub(crate) fn update_size(&mut self, new_size: SizeHint) {
        self.size = new_size;
    }
}
