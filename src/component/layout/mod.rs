pub mod children;
mod solver;

use std::collections::{hash_map::Keys, HashMap};

use log::info;

use self::{
    children::{Child, Children},
    solver::{ElementPart, Solver},
};
use crate::{
    cassowary::*,
    cb,
    compositor::context::Context,
    state::State,
    terminal::{
        buffer::MappedBuffer,
        cursor::{Cursor, Index},
        region::Region,
        size::Size,
    },
};

use super::{cb::SizeFn, text, Cb, Component, DrawFn, LayoutFn};

pub fn line<S>(children: S) -> Component
where
    S: Into<State<Children>>,
{
    let children = children.into();

    let mut component = Component::new(basic_draw(children.clone()));
    component.layout_fn = Some(basic_layout(line_layout_fn, children.clone()));
    component.size_fn = Some(basic_size_hint(children));
    component
}

pub fn column<S>(children: S) -> Component
where
    S: Into<State<Children>>,
{
    let children = children.into();

    let mut component = Component::new(basic_draw(children.clone()));
    component.layout_fn = Some(basic_layout(column_layout_fn, children.clone()));
    component.size_fn = Some(basic_size_hint(children));
    component
}

fn basic_layout<F>(mut layout_fn: F, children: State<Children>) -> LayoutFn
where
    F: FnMut(&mut Solver, Context<'_>) + 'static,
{
    let mut solver = Solver::new();
    let mut last_region = Region::default();

    cb!(move |region, context| {
        let is_changed = context.is_changed(&children);
        let children = context.get(&children);

        if is_changed || last_region == Region::default() {
            info!("First call or region changed, merge childs");
            solver.clear();
            solver.merge_childs(&children.data.borrow());
            layout_fn(&mut solver, context);
        }

        solver.suggest_size(region.size());
        info!("Suggested region {:#?}", region);

        for child in children.data.borrow_mut().iter_mut() {
            solver.merge_size_from_child(child);
        }

        let (changes, vars) = solver.get_changes();

        for (variable, value) in changes {
            if let Some((child_idx, field)) = vars.get(variable) {
                info!("Have change: {:?}, field = {:#?}, value = {:?}", child_idx, field, value);

                let mut data = children.data.borrow_mut();
                let mut child = data.get_mut(*child_idx).unwrap();
                let mut transcation = child.start_transcation();

                let value = *value as Index;

                match field {
                    ElementPart::LeftX => transcation.change_left_x(value),
                    ElementPart::LeftY => transcation.change_left_y(value),
                    ElementPart::RightX => transcation.change_right_x(value),
                    ElementPart::RightY => transcation.change_right_y(value),
                }
            }
        }

        children.layout_all(context);
    })
}

fn basic_draw(children: State<Children>) -> DrawFn {
    cb!(move |mut buf, context| {
        let children = context.get(&children);

        info!("Draw children");

        for child in children.data.borrow_mut().iter_mut() {
            info!("Region = {:#?}", child.region());
            info!("Size = {:#?}", child.size());
            let mapped_buf = buf.map(child.region().unwrap());
            child.component.draw(mapped_buf, context);
        }
    })
}

fn basic_size_hint(children: State<Children>) -> SizeFn {
    cb!(move |context| {
        let mut size = Size::default();

        let children = context.get(&children);

        for child in children.data.borrow_mut().iter_mut() {
            let new_size = child.component.size_hint(context);
            child.update_size(new_size);
            size += new_size;
        }

        size
    })
}

fn column_layout_fn(solver: &mut Solver, context: Context<'_>) {
    let mut constraints = vec![];

    for elements in solver.elements().windows(2) {
        let cur = &elements[0];
        let next = &elements[1];

        let next_element_start_from_prev_x = (cur.right_x + 1.0) | EQ(REQUIRED) | next.left_x;
        let elements_y_are_equal = cur.left_y | EQ(REQUIRED) | next.left_y;

        constraints.push(next_element_start_from_prev_x);
        constraints.push(elements_y_are_equal);
    }

    solver.add_constraints(constraints).unwrap();
}

fn line_layout_fn(solver: &mut Solver, context: Context<'_>) {
    let mut constraints = vec![];

    for elements in solver.elements().windows(2) {
        let cur = &elements[0];
        let next = &elements[1];

        let next_element_start_from_prev_y = (cur.right_y + 1.0) | EQ(REQUIRED) | next.left_y;
        let elements_x_are_equal = cur.left_x | EQ(REQUIRED) | next.left_x;

        constraints.push(next_element_start_from_prev_y);
        constraints.push(elements_x_are_equal);
    }

    solver.add_constraints(constraints).unwrap();
}
