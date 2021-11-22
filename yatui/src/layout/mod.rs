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
    data: Vec<Box<dyn Widget>>,
}

impl Widget for Layout {
    fn draw(&mut self, buf: MappedBuffer<'_>) {
        todo!()
    }
}
