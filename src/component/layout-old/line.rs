use crate::cassowary::*;

use crate::{component::Component, compositor::context::Context};

use super::{child::Child, Layout, LayoutSystem};

pub fn line<V>(childs: V) -> Component
where
    V: IntoIterator<Item = Component>,
{
    let layout_fn = |mut system: LayoutSystem<'_>, _: Context<'_>| {
        let mut constraints = vec![];

        for elements in system.elements().windows(2) {
            let cur = &elements[0];
            let next = &elements[1];

            let next_element_start_from_prev_x = (cur.right_x + 1.0) | EQ(REQUIRED) | next.left_x;
            let elements_y_are_equal = cur.left_y | EQ(REQUIRED) | next.left_y;

            constraints.push(next_element_start_from_prev_x);
            constraints.push(elements_y_are_equal);
        }

        system.add_constraints(constraints).unwrap();
    };

    let childs = childs.into_iter().map(Child::new).collect();

    Layout::new(childs, layout_fn).into()
}
