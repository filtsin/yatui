use crate::cassowary::*;

use crate::{component::Component, compositor::context::Context};

use super::{child::Child, Layout, LayoutSystem};

pub fn line<V>(childs: V) -> Component
where
    V: IntoIterator<Item = Component>,
{
    let layout_fn = |mut system: LayoutSystem<'_>, _: Context<'_>| {
        if system.is_empty() || system.len() == 1 {
            return;
        }

        for i in 0..system.len() - 1 {
            let prev = system.get(i).unwrap();
            let next = system.get(i + 1).unwrap();

            let constraint_x = prev.right_x | EQ(REQUIRED) | next.left_x;
            let constraint_y = prev.left_y | EQ(REQUIRED) | next.left_y;

            system.add(constraint_x).unwrap();
            system.add(constraint_y).unwrap();
        }
    };

    let childs = childs.into_iter().map(Child::new).collect();

    Layout::new(childs, layout_fn).into()
}
