use crate::{compositor::context::Context, state::State, terminal::region::Region};

use super::{size_hint::SizeHint, Component};

type LayoutFn = dyn Fn(Region, LayoutInfo<'_>);

pub struct Layout {
    childs: State<Vec<Component>>,

    layout_fn: Box<LayoutFn>,
}

pub struct LayoutInfo<'a> {
    pub components: &'a [Component],
    pub cached_size: SizeHint,
}

impl Layout {
    pub fn new<F>(childs: State<Vec<Component>>, layout_fn: F) -> Self
    where
        F: Fn(Region, LayoutInfo<'_>) + 'static,
    {
        Self { childs, layout_fn: Box::new(layout_fn) }
    }

    pub fn layout(&self, region: Region, context: Context) {
        let info =
            LayoutInfo { components: context.get(&self.childs).as_slice(), cached_size: todo!() };

        (*self.layout_fn)(region, info);
    }
}
