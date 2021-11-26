use crate::{
    terminal::{buffer::MappedBuffer, cursor::Index},
    widget::{SizeHint, Widget},
};

#[derive(Debug)]
#[non_exhaustive]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum LayoutType {
    Content,
    Fixed(usize),
}

pub struct Layout {
    data: Vec<Box<dyn Widget + Send>>,
    direction: LayoutDirection,
    ltype: LayoutType,
}

impl std::fmt::Debug for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layout")
            .field("direction", &self.direction)
            .field("ltype", &self.ltype)
            .finish()
    }
}

impl Layout {
    pub fn new(direction: LayoutDirection, ltype: LayoutType) -> Self {
        Self { data: vec![], direction, ltype }
    }
    pub fn add_widget(&mut self, widget: Box<dyn Widget + Send>) {
        self.data.push(widget);
    }
}

impl Widget for Layout {
    fn draw(&mut self, buf: MappedBuffer<'_>) {
        // just prototype
        if self.data.len() == 1 {
            self.data[0].draw(buf);
        }
    }
}
