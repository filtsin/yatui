//! Backend trait

mod termion;

use crate::{error::Result, terminal::cursor::Index};

pub use self::termion::Termion;

pub trait Backend: Send {
    fn get_size(&self) -> Result<(Index, Index)>;
    fn move_cursor(&mut self, pos: (Index, Index));
    fn clear_screen(&mut self);

    fn draw(&mut self, str: &str);
    fn flush(&mut self);
}
