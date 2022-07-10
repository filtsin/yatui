pub mod children;
mod solver;

use std::collections::{hash_map::Keys, HashMap};

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

    Component::builder()
        .draw_fn(basic_draw(children.clone()))
        .layout_fn(basic_layout(line_layout_fn, children.clone()))
        .size_fn(line_size_fn(children))
        .build()
}

pub fn column<S>(children: S) -> Component
where
    S: Into<State<Children>>,
{
    let children = children.into();

    Component::builder()
        .draw_fn(basic_draw(children.clone()))
        .layout_fn(basic_layout(column_layout_fn, children.clone()))
        .size_fn(column_size_fn(children))
        .build()
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
            solver.clear();
            solver.merge_childs(&children.data.borrow());
            layout_fn(&mut solver, context);
        }

        solver.suggest_size(region.size());

        for child in children.data.borrow_mut().iter_mut() {
            solver.merge_size_from_child(child);
        }

        let (changes, vars) = solver.get_changes();

        for (variable, value) in changes {
            if let Some((child_idx, field)) = vars.get(variable) {
                let mut data = children.data.borrow_mut();
                let mut child = data.get_mut(*child_idx).unwrap();

                let value = *value as Index;

                match field {
                    ElementPart::LeftX => child.change_region().left_x(value),
                    ElementPart::LeftY => child.change_region().left_y(value),
                    ElementPart::RightX => child.change_region().right_x(value),
                    ElementPart::RightY => child.change_region().right_y(value),
                }
            }
        }

        children.data.borrow_mut().iter_mut().for_each(|v| v.layout(context));
    })
}

fn basic_draw(children: State<Children>) -> DrawFn {
    cb!(move |mut buf, context| {
        let children = context.get(&children);
        children.data.borrow_mut().iter_mut().for_each(|v| v.draw(&mut buf, context));
    })
}

fn column_size_fn(children: State<Children>) -> SizeFn {
    cb!(move |context| {
        let mut size = Size::default();

        for child in context.get(&children).data.borrow_mut().iter_mut() {
            size.add_assign_width_size(child.update_size(context));
        }

        size
    })
}

fn line_size_fn(children: State<Children>) -> SizeFn {
    cb!(move |context| {
        let mut size = Size::default();

        for child in context.get(&children).data.borrow_mut().iter_mut() {
            size.add_assign_height_size(child.update_size(context));
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
