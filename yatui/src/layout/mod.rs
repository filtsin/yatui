pub mod auto;
pub mod wrapper;

use std::cell::Cell;

use crate::{
    terminal::{cursor::Index, region::Region},
    widget::{SizeHint, Widget, WidgetSize},
};

pub trait Layout {
    fn layout(&self, region: Region, info: LayoutInfo<'_>);
}

#[derive(Debug)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

pub struct Child {
    widget: Box<dyn Widget + Send>,
    region: Region,
    size: Cell<SizeHint>,
}

pub struct LayoutInfo<'a> {
    childs: &'a mut [Child],
    cached_size: SizeHint,
}

impl Child {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + Send + 'static,
    {
        let size_hint = widget.size_hint();
        Child { widget: Box::new(widget), region: Region::default(), size: Cell::new(size_hint) }
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = region;
    }

    pub(crate) fn update_size(&self) -> SizeHint {
        if self.size_changed() {
            self.size.replace(self.widget.size_hint());
        }
        self.size.get()
    }

    pub(crate) fn size_changed(&self) -> bool {
        self.widget.size_changed()
    }

    pub(crate) fn cached_size(&self) -> SizeHint {
        self.size.get()
    }
}

impl<'a> LayoutInfo<'a> {
    pub fn new(childs: &'a mut [Child], size: SizeHint) -> Self {
        Self { childs, cached_size: size }
    }

    pub fn childs(&mut self) -> &mut [Child] {
        self.childs
    }

    pub fn size(&self) -> SizeHint {
        self.cached_size
    }
}
