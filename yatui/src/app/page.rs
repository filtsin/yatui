use crate::layout::Layout;

#[derive(Debug)]
pub struct Page {
    pub(crate) layout: Layout,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Id(u32);

impl Id {
    pub fn new(v: u32) -> Self {
        Self(v)
    }
}
