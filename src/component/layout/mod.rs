pub mod child;
mod column;
mod line;
mod solver;

use std::fmt::Debug;

use crate::cassowary::Constraint;

use crate::{
    component::WidgetSize,
    compositor::context::Context,
    error::Error,
    terminal::{buffer::MappedBuffer, cursor::Index, region::Region},
};

use self::{
    child::Child,
    solver::{ElementPart, Solver},
};

use log::info;

use super::{canvas::Canvas, Component};

pub use column::column;
pub use line::line;
pub use solver::Element;

type LayoutFn = dyn Fn(LayoutSystem<'_>, Context<'_>);

pub struct Layout {
    childs: Vec<Child>,
    solver: Solver,

    layout_fn: Box<LayoutFn>,
    size: WidgetSize,

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
            size: WidgetSize::min(),
            last_region: None,
        };

        res.solver.merge_childs(&res.childs);
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
            let new_buffer = buffer.map(region);
            child.component.draw(new_buffer, context);
        }
    }

    pub fn size_hint(&self, context: Context<'_>) -> WidgetSize {
        self.size
    }

    pub fn calc_size(&mut self, context: Context<'_>) -> WidgetSize {
        let mut result = WidgetSize::default();
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

    pub fn childs(&self) -> &Vec<Child> {
        &self.childs
    }
}

pub struct LayoutSystem<'a> {
    solver: &'a mut Solver,
}

impl<'a> LayoutSystem<'a> {
    pub fn elements(&self) -> &Vec<Element> {
        self.solver.elements()
    }

    pub fn add_constraint(&mut self, constraint: Constraint) -> Result<(), Error> {
        self.solver.add_custom_constraint(constraint).map_err(Error::from)
    }

    pub fn add_constraints(&mut self, constraints: Vec<Constraint>) -> Result<(), Error> {
        constraints.into_iter().try_for_each(|v| self.add_constraint(v))
    }
}

impl Debug for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layout").finish()
    }
}
