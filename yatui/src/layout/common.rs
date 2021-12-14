use std::marker::PhantomData;

use crate::{
    terminal::{buffer::MappedBuffer, region::Region},
    widget::{SizeHint, Widget, WidgetSize},
};

pub struct DefaultLayout<T> {
    childs: Vec<Child>,
    last_region: Region,

    marker: PhantomData<T>,
}

pub trait CommonLayout {
    fn layout(region: Region, childs: &mut [Child]);
}

pub struct Child {
    widget: Box<dyn Widget + Send>,
    region: Region,
}

impl Child {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + Send + 'static,
    {
        Child { widget: Box::new(widget), region: Region::default() }
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = region
    }
}

impl<T> DefaultLayout<T> {
    pub fn new() -> Self {
        Self { childs: vec![], last_region: Region::default(), marker: PhantomData }
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
}

impl<T> Default for DefaultLayout<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Widget for DefaultLayout<T>
where
    T: CommonLayout,
{
    fn draw(&mut self, buf: MappedBuffer<'_>) {
        // If the region has changed
        if buf.region() != self.last_region {
            self.last_region = buf.region();
            T::layout(buf.region(), self.childs.as_mut_slice());
        }
        todo!()
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::Min(WidgetSize::new(1, 1))
    }

    fn is_show(&mut self) -> bool {
        true
    }

    fn take_focus(&mut self) {}

    fn need_redraw(&self) -> bool {
        true
    }
}
