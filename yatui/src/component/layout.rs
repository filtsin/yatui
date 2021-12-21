use crate::state::State;

use super::Component;

pub struct Layout {
    childs: State<Vec<Component>>,
}

pub struct LayoutInfo {}
