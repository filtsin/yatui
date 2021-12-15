use std::ops::{Deref, DerefMut};

use crate::{
    terminal::{buffer::MappedBuffer, cursor::Index, region::Region},
    widget::{SizeHint, Widget},
};

use super::{Child, Layout};

/// Wrapper for custom layout with common logic. Probably you don't want to use this
/// on your code.
#[derive(Default)]
pub struct LayoutWrapper<T> {
    childs: Vec<Child>,
    last_region: Region,
    padding: Index,

    inner: T,
}

impl<T> LayoutWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { childs: vec![], last_region: Region::default(), padding: Index::default(), inner }
    }

    pub fn add_widget<U>(mut self, widget: U) -> Self
    where
        U: Widget + Send + 'static,
    {
        self.childs.push(Child::new(widget));
        // We added new child, so we need to relayout content
        self.last_region = Region::default();
        self
    }

    pub fn set_padding(mut self, padding: Index) -> Self {
        self.padding = padding;
        self
    }

    fn draw_childs(&mut self, region: Region) {
        todo!()
    }
}

impl<T> Widget for LayoutWrapper<T>
where
    T: Layout,
{
    fn draw(&mut self, buf: MappedBuffer<'_>) {
        // If the region has changed
        if buf.region() != self.last_region {
            self.last_region = buf.region();
            self.inner.layout(buf.region(), self.childs.as_mut_slice());
        }

        self.draw_childs(buf.padding(self.padding).region())
    }

    fn size_hint(&self) -> SizeHint {
        let mut result = SizeHint::default();
        for child in &self.childs {
            result += child.size()
        }
        result
    }

    fn is_show(&self) -> bool {
        true
    }

    fn take_focus(&mut self) {}

    fn need_redraw(&self) -> bool {
        true
    }
}

impl<T> Deref for LayoutWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for LayoutWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
