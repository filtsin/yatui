pub mod child;

use crate::{
    component::size_hint::WidgetSize,
    compositor::context::Context,
    state::State,
    terminal::{buffer::MappedBuffer, region::Region},
};

use self::child::Child;
use super::{size_hint::SizeHint, Component};

type LayoutFn = dyn Fn(Region, LayoutInfo<'_>, Context<'_>);

pub struct Layout {
    childs: Vec<Child>,

    layout_fn: Box<LayoutFn>,
}

pub struct LayoutInfo<'a> {
    pub childs: &'a mut [Child],
    pub cached_size: SizeHint,
}

impl Layout {
    pub fn new<F>(childs: Vec<Child>, layout_fn: F) -> Self
    where
        F: Fn(Region, LayoutInfo<'_>, Context<'_>) + 'static,
    {
        Self { childs, layout_fn: Box::new(layout_fn) }
    }

    pub fn layout(&mut self, region: Region, context: Context<'_>) {
        let info = LayoutInfo {
            childs: self.childs.as_mut_slice(),
            cached_size: SizeHint::new_min(WidgetSize::min()),
        };

        // must update region for every child
        (self.layout_fn)(region, info, context);
    }

    pub fn draw(&mut self, buffer: MappedBuffer<'_>, context: Context<'_>) {}

    pub fn size_hint(&self, context: Context<'_>) -> SizeHint {
        todo!()
    }

    pub(crate) fn calc_size(&mut self) {}
}

pub fn column() {
    todo!()
}
