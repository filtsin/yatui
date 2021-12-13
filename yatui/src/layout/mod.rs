pub mod common;
pub mod utils;

pub use crate::terminal::cursor::Cursor;
pub use common::CommonLayout;

use crate::terminal::buffer::MappedBuffer;

pub trait Layout {
    fn layout(&mut self, buf: MappedBuffer);
    fn size(&self) -> Cursor;
}
