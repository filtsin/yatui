//! Backend trait

mod termion;

use crate::{
    error::Result,
    terminal::{buffer::Buffer, cursor::Index},
};

pub use self::termion::Termion;

pub trait Backend {
    fn get_size(&self) -> Result<(Index, Index)>;
    fn move_cursor(&mut self, pos: (Index, Index));
    fn clear_screen(&mut self);

    fn draw(&mut self, buffer: Buffer);
    fn flush(&mut self);
}
