use std::cell::RefCell;

use crate::{
    component::Component,
    compositor::context::Context,
    terminal::{
        buffer::MappedBuffer,
        cursor::{Cursor, Index},
        region::Region,
        size::Size,
    },
};

#[derive(Default)]
pub struct Children {
    pub(crate) data: RefCell<Vec<Child>>,
}

pub(crate) struct Child {
    component: Component,
    position: usize,

    region: ChildRegion,
    size_cache: Size,
}

// It is `Region` but sometimes contract `right_bottom` < `left_top` is incorrect
#[derive(Default)]
pub(crate) struct ChildRegion {
    left_top: Cursor,
    right_bottom: Cursor,
}

impl Children {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Component>,
    {
        Self {
            data: RefCell::new(
                iter.into_iter()
                    .enumerate()
                    .map(|(i, component)| Child::new(component, i))
                    .collect(),
            ),
        }
    }
}

impl Child {
    fn new(component: Component, position: usize) -> Self {
        Self { component, position, region: ChildRegion::default(), size_cache: Size::default() }
    }

    pub fn region(&self) -> Option<Region> {
        self.region.try_build_region()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn size(&self) -> Size {
        self.size_cache
    }

    pub fn update_size(&mut self, context: Context<'_>) -> Size {
        self.size_cache = self.component.size_hint(context);
        self.size_cache
    }

    pub fn layout(&mut self, context: Context<'_>) {
        self.component.layout(self.region().unwrap(), context);
    }

    pub fn draw(&mut self, buf: &mut MappedBuffer<'_>, context: Context<'_>) {
        let mapped_buf = buf.map(self.region().unwrap());
        self.component.draw(mapped_buf, context);
    }

    pub fn change_region(&mut self) -> &mut ChildRegion {
        &mut self.region
    }
}

impl ChildRegion {
    fn try_build_region(&self) -> Option<Region> {
        Region::try_new(self.left_top, self.right_bottom)
    }

    pub fn left_x(&mut self, value: Index) {
        self.left_top.set_column(value);
    }

    pub fn right_x(&mut self, value: Index) {
        self.right_bottom.set_column(value);
    }

    pub fn left_y(&mut self, value: Index) {
        self.left_top.set_row(value);
    }

    pub fn right_y(&mut self, value: Index) {
        self.right_bottom.set_row(value);
    }
}
