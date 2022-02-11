use crate::{
    backend::Backend,
    error::Result,
    terminal::{
        buffer::Buffer,
        cursor::{Cursor, Index},
        size::Size,
    },
};

#[derive(Default)]
pub struct Raw {
    output: Buffer,
    pos: Cursor,
}

impl Raw {
    pub fn new(width: Index, height: Index) -> Raw {
        Self { output: Buffer::new(Size::new(width, height)), ..Default::default() }
    }
}

impl Backend for Raw {
    fn get_size(&self) -> Result<Size> {
        Ok(self.output.size())
    }

    fn move_cursor(&mut self, pos: Cursor) {
        self.pos = pos;
    }

    fn hide_cursor(&mut self) {
        todo!()
    }

    fn show_cursor(&mut self) {
        todo!()
    }

    fn clear_screen(&mut self) {
        todo!()
    }

    fn draw(&mut self, buffer: &Buffer) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }
}
