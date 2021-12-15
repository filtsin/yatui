pub mod auto;
pub mod wrapper;

use crate::{
    terminal::region::Region,
    widget::{SizeHint, Widget, WidgetSize},
};

pub trait Layout {
    fn layout(&self, region: Region, childs: &mut [Child]);
}

#[derive(Debug)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

pub struct Child {
    widget: Box<dyn Widget + Send>,
    region: Region,
}

impl Child {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + Send + 'static,
    {
        Child { widget: Box::new(widget), region: Region::default() }
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = region
    }

    pub fn size(&self) -> SizeHint {
        if self.widget.is_show() {
            self.widget.size_hint()
        } else {
            SizeHint::new_max(WidgetSize::new(0, 0))
        }
    }
}
