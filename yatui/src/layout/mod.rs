use crate::terminal::buffer::MappedBuffer;

pub trait Layout {
    fn draw(&self, buf: MappedBuffer);
    fn direction() -> LayoutDirection;
    fn ltype() -> LayoutType;
}

pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

pub enum LayoutType {
    Content,
    Fixed(usize),
}
