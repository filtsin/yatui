pub mod child;

use crate::{
    component::size_hint::WidgetSize,
    compositor::context::Context,
    state::State,
    terminal::{buffer::MappedBuffer, region::Region},
};

use self::child::Child;
use super::{canvas::Canvas, size_hint::SizeHint, Component};

type LayoutFn = dyn Fn(Region, LayoutInfo<'_>, Context<'_>);

pub struct Layout {
    childs: Vec<Child>,

    layout_fn: Box<LayoutFn>,
    size: SizeHint,
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
        Self { childs, layout_fn: Box::new(layout_fn), size: SizeHint::new_max(WidgetSize::min()) }
    }

    pub fn layout(&mut self, region: Region, context: Context<'_>) {
        let info = LayoutInfo {
            childs: self.childs.as_mut_slice(),
            cached_size: SizeHint::new_min(WidgetSize::min()),
        };

        // must update region for every child
        (self.layout_fn)(region, info, context);
        // Here childs regions updated
    }

    pub fn draw(&mut self, buffer: MappedBuffer<'_>, context: Context<'_>) {}

    pub fn size_hint(&self, context: Context<'_>) -> SizeHint {
        self.size
    }

    pub fn calc_size(&mut self, context: Context<'_>) -> SizeHint {
        let mut result = SizeHint::default();
        for Child { component, .. } in self.childs.iter_mut() {
            match component {
                Component::Canvas(c) => result += c.size_hint(context),
                Component::Layout(l) => result += l.calc_size(context),
            }
        }

        self.size = result;
        result
    }
}

pub fn column() {
    todo!()
}
