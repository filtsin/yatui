/// Widget trait
use crate::terminal::{buffer::MappedBuffer, cursor::Index};

pub trait Widget {
    fn draw(&self, buf: MappedBuffer<'_>);
    fn need_size(&self) -> (Index, Index);
    fn min_size(&self) -> (Index, Index);
    fn is_show(&self) -> bool;
}
