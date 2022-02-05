pub mod child;
mod line;
mod solver;

use cassowary::Constraint;

use crate::{
    component::size_hint::WidgetSize,
    compositor::context::Context,
    error::Error,
    terminal::{buffer::MappedBuffer, cursor::Index, region::Region},
};

use self::{
    child::Child,
    solver::{Element, ElementPart, Solver},
};

use log::info;

use super::{canvas::Canvas, size_hint::SizeHint, Component};

pub use line::line;

type LayoutFn = dyn Fn(LayoutSystem<'_>, Context<'_>);

pub struct Layout {
    childs: Vec<Child>,
    solver: Solver,

    layout_fn: Box<LayoutFn>,
    size: SizeHint,

    last_region: Option<Region>,
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
            last_region: None,
        };
        res.solver.merge_childs(&res.childs);

        info!("Creation completed");

        res
    }

    pub fn layout(&mut self, region: Region, context: Context<'_>) {
        if self.last_region.is_none() {
            let system = LayoutSystem { solver: &mut self.solver };
            // Must add all additional equations into system
            (self.layout_fn)(system, context);
        }

        if self.last_region.is_some() && self.last_region.unwrap() == region {
            return;
        }

        for (i, child) in self.childs.iter_mut().enumerate() {
            let size_hint = child.component.size_hint(context);
            child.update_size(size_hint);
            self.solver.merge_size_from_child(child, i);
        }

        self.solver.suggest_size(region.width(), region.height());
        self.last_region = Some(region);

        let (changes, vars) = self.solver.get_changes();

        info!("Changes: {:?}\nVars: {:?}", changes, vars);

        for (variable, value) in changes {
            if let Some((child_idx, field)) = vars.get(variable) {
                let child = self.childs.get_mut(*child_idx).unwrap();
                let value = *value as Index;

                match field {
                    ElementPart::LeftX => child.region.left_top.set_column(value),
                    ElementPart::LeftY => child.region.left_top.set_row(value),
                    ElementPart::RightX => child.region.right_bottom.set_column(value),
                    ElementPart::RightY => child.region.right_bottom.set_row(value),
                }
            }
        }
    }

    pub fn draw(&mut self, mut buffer: MappedBuffer<'_>, context: Context<'_>) {
        for (i, child) in self.childs.iter_mut().enumerate() {
            let region = child.region;
            info!("Child {}, region = {:?}", i, region);
            let new_buffer = buffer.map_region(region);
            child.component.draw(new_buffer, context);
        }
    }

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
