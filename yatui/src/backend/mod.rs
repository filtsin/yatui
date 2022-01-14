//! Backend trait

mod string;
mod termion;

use crate::{
    error::Result,
    terminal::{buffer::Buffer, cursor::Cursor},
};

pub use self::{string::StringB, termion::Termion};

pub trait Backend {
    fn get_size(&self) -> Result<Cursor>;
    fn move_cursor(&mut self, pos: Cursor);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn clear_screen(&mut self);

    fn draw(&mut self, buffer: &Buffer);
    fn flush(&mut self);
}
