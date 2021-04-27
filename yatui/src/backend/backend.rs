//! Backend trait
use crate::error::Result;

pub trait Backend: Send {
    fn get_size(&self) -> Result<(u16, u16)>;
    fn move_cursor(&mut self, pos: (u16, u16));
    fn clear_screen(&mut self);

    fn draw(&mut self, str: &str);
    fn flush(&mut self);
}
