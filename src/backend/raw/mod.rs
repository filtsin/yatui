mod cell;
mod terminal;

use crate::{
    backend::Backend,
    error::Result,
    terminal::{buffer::Buffer, Cursor, Index, Size},
};

use self::terminal::Terminal;

/// It is raw backend
#[derive(Default)]
pub struct Raw {
    terminal: Terminal,
    cursor_show: bool,
}

impl Raw {
    pub fn new(width: Index, height: Index) -> Self {
        Self { terminal: Terminal::new(width.into(), height.into()), cursor_show: true }
    }
}

impl Backend for Raw {
    fn get_size(&self) -> Result<Size> {
        let w: Index = self.terminal.width().try_into().unwrap();
        let h: Index = self.terminal.height().try_into().unwrap();
        Ok(Size::new(w, h))
    }

    fn move_cursor(&mut self, pos: Cursor) {
        self.terminal.move_cursor(pos.column().into(), pos.row().into());
    }

    fn hide_cursor(&mut self) {
        self.cursor_show = false;
    }

    fn show_cursor(&mut self) {
        self.cursor_show = true;
    }

    fn clear_screen(&mut self) {
        self.terminal.fill(" ");
    }

    fn draw(&mut self, buffer: &Buffer) {
        // nothing here NOW
    }

    fn flush(&mut self) {
        // nothing here
    }
}
