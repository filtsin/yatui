use super::{Cursor, Layout};
use crate::{terminal::buffer::MappedBuffer, widget::Widget};

pub struct CommonLayout {
    childs: Vec<Child>,
    direction: LayoutDirection,
}

//#[derive(Debug)]
pub enum Child {
    Widget(Box<dyn Widget + Send>),
    Layout(Box<dyn Layout + Send>),
}

#[derive(Debug)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

impl CommonLayout {
    pub fn horizontal() -> Self {
        CommonLayout { childs: vec![], direction: LayoutDirection::Horizontal }
    }

    pub fn vertical() -> Self {
        CommonLayout { childs: vec![], direction: LayoutDirection::Vertical }
    }

    pub fn add_child(mut self, child: Child) -> Self {
        self.childs.push(child);
        self
    }

    pub fn add_widget<T>(self, widget: T) -> Self
    where
        T: Widget + Send + 'static,
    {
        self.add_child(Child::Widget(Box::new(widget)))
    }

    pub fn add_layout<T>(self, layout: T) -> Self
    where
        T: Layout + Send + 'static,
    {
        self.add_child(Child::Layout(Box::new(layout)))
    }
}

impl Layout for CommonLayout {
    fn layout(&mut self, buf: MappedBuffer) {
        todo!()
    }

    fn size(&self) -> Cursor {
        todo!()
    }
}
