//! Backend trait

mod raw;
mod termion;

use crate::{
    error::Result,
    terminal::{Cursor, Size},
    text::Style,
};

pub use self::{raw::Raw, termion::Termion};

pub trait Backend {
    fn get_size(&self) -> Result<Size>;
    fn move_cursor(&mut self, pos: Cursor);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn clear_screen(&mut self);

    fn draw(&mut self, s: &str, style: Style);
    fn flush(&mut self);
}
