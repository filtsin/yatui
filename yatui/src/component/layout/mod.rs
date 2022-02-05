pub mod child;
mod line;
pub mod solver;

use cassowary::Constraint;

use crate::{
    component::size_hint::WidgetSize,
    compositor::context::Context,
    error::Error,
    terminal::{buffer::MappedBuffer, region::Region},
};

use self::{
    child::Child,
    solver::{Element, Solver},
};

use super::{canvas::Canvas, size_hint::SizeHint, Component};

pub use line::line;

type LayoutFn = dyn Fn(LayoutSystem<'_>, Context<'_>);

pub struct Layout {
    childs: Vec<Child>,
    solver: Solver,

    layout_fn: Box<LayoutFn>,
    size: SizeHint,
}

impl Layout {
    pub fn new<F>(childs: Vec<Child>, layout_fn: F) -> Self
    where
        F: Fn(LayoutSystem<'_>, Context<'_>) + 'static,
    {
        let mut res = Self {
            childs,
            solver: Solver::new(),
            layout_fn: Box::new(layout_fn),
            size: SizeHint::new_max(WidgetSize::min()),
        };
        res.solver.merge_childs(&res.childs);
        res
    }

    pub fn layout(&mut self, region: Region, context: Context<'_>) {
        self.solver.suggest_size(region.width(), region.height());
        let system = LayoutSystem { solver: &mut self.solver };
        // Must add all additional equations into system
        (self.layout_fn)(system, context);
    }

    pub fn draw(&mut self, buffer: MappedBuffer<'_>, context: Context<'_>) {}

    pub fn size_hint(&self, context: Context<'_>) -> SizeHint {
        self.size
    }

    pub fn calc_size(&mut self, context: Context<'_>) -> SizeHint {
        let mut result = SizeHint::default();
        for child in self.childs.iter_mut() {
            let new_size = match &mut child.component {
                Component::Canvas(c) => c.size_hint(context),
                Component::Layout(l) => l.calc_size(context),
            };
            child.update_size(new_size);
            result += new_size;
        }

        self.size = result;
        result
    }
}

pub struct LayoutSystem<'a> {
    solver: &'a mut Solver,
}

impl<'a> LayoutSystem<'a> {
    pub fn get(&self, index: usize) -> Option<&Element> {
        self.solver.get(index)
    }

    pub fn len(&self) -> usize {
        self.solver.element_len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, constraint: Constraint) -> Result<(), Error> {
        self.solver.add_custom_constraint(constraint).map_err(Error::from)
    }
}
