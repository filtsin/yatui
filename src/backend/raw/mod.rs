mod cell;
mod terminal;

use std::ops::RangeBounds;

use crate::{
    backend::Backend,
    error::Result,
    terminal::{buffer::Buffer, Cursor, Index, Size},
    text::Style,
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

    pub fn lines_to_vec(&self) -> Vec<String> {
        self.terminal.lines_to_vec()
    }

    pub fn assert_styles<R>(&self, column: R, line: R, style: Style)
    where
        R: RangeBounds<usize>,
    {
        self.terminal.assert_styles(column, line, style);
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

    fn draw(&mut self, s: &str, style: Style) {
        self.terminal.write_str(s, style)
    }

    fn flush(&mut self) {
        // nothing here
    }
}
