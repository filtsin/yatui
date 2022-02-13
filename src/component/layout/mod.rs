pub mod child;
mod column;
mod line;
mod solver;

use std::{
    collections::{hash_map::Keys, HashMap},
    fmt::Debug,
};

use crate::{cassowary::Constraint, terminal::cursor::Cursor};

use self::{
    child::Child,
    solver::{ElementPart, Solver},
};

use crate::{
    compositor::context::Context,
    error::Error,
    terminal::{buffer::MappedBuffer, cursor::Index, region::Region, size::Size},
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
    size: Size,

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
            size: Size::min(),
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

        for (i, child) in self.childs.iter_mut().enumerate() {
            let size_hint = child.component.size_hint(context);
            child.update_size(size_hint);
            self.solver.merge_size_from_child(child, i);
        }

        self.solver.suggest_size(region.width(), region.height());
        self.last_region = Some(region);

        let (changes, vars) = self.solver.get_changes();

        info!("Changes: {:?}\nVars: {:?}", changes, vars);

        let mut changer = RegionChanger::default();

        for (variable, value) in changes {
            if let Some((child_idx, field)) = vars.get(variable) {
                let id = *child_idx;

                let child = self.childs.get_mut(id).unwrap();

                changer.save(id, child.region());

                let value = *value as Index;

                match field {
                    ElementPart::LeftX => changer.change_left_x(id, value),
                    ElementPart::LeftY => changer.change_left_y(id, value),
                    ElementPart::RightX => changer.change_right_x(id, value),
                    ElementPart::RightY => changer.change_right_y(id, value),
                }
            }
        }

        for change in changer.changes() {
            let child = self.childs.get_mut(*change).unwrap();
            if let Some(new_region) = changer.get_region(*change) {
                child.update_region(Some(new_region));
            }
            info!("Region: {:?}", child.region());
        }
    }

    pub fn draw(&mut self, mut buffer: MappedBuffer<'_>, context: Context<'_>) {
        for (i, child) in self.childs.iter_mut().enumerate() {
            if let Some(region) = child.region {
                info!("Child {}, region = {:?}", i, region);
                let new_buffer = buffer.map(region);
                child.component.draw(new_buffer, context);
            }
        }
    }

    pub fn size_hint(&self, context: Context<'_>) -> Size {
        self.size
    }

    pub fn calc_size(&mut self, context: Context<'_>) -> Size {
        let mut result = Size::default();
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

#[derive(Default)]
struct RegionChanger {
    actions: HashMap<usize, (Cursor, Cursor)>,
}

impl RegionChanger {
    fn new() -> Self {
        Self::default()
    }

    fn save(&mut self, id: usize, region: Option<Region>) {
        if self.actions.contains_key(&id) {
            return;
        }

        let (left_top, right_bottom) = match region {
            Some(region) => (region.left_top(), region.right_bottom()),
            None => (Cursor::default(), Cursor::default()),
        };

        self.actions.insert(id, (left_top, right_bottom));
    }

    fn change_left_x(&mut self, id: usize, new_x: Index) {
        let value = self.actions.entry(id).or_default();
        value.0.set_column(new_x);
    }

    fn change_left_y(&mut self, id: usize, new_x: Index) {
        let value = self.actions.entry(id).or_default();
        value.0.set_row(new_x);
    }

    fn change_right_x(&mut self, id: usize, new_x: Index) {
        let value = self.actions.entry(id).or_default();
        value.1.set_column(new_x);
    }

    fn change_right_y(&mut self, id: usize, new_x: Index) {
        let value = self.actions.entry(id).or_default();
        value.1.set_row(new_x);
    }

    fn get_region(&self, id: usize) -> Option<Region> {
        let value = self.actions.get(&id).unwrap();
        Region::try_new(value.0, value.1)
    }

    fn changes(&self) -> Keys<'_, usize, (Cursor, Cursor)> {
        self.actions.keys()
    }
}
