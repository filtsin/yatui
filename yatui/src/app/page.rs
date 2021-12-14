use std::fmt::Debug;

use crate::widget::Widget;

pub struct Page {
    pub main_widget: Box<dyn Widget + Send>,
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{layout}}")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Id(u32);

impl Id {
    pub fn new(v: u32) -> Self {
        Self(v)
    }
}
