use crate::{backend::Backend, error::Result, terminal::cursor::Cursor};

#[derive(Default)]
pub struct StringB {
    output: String,
    size: Cursor,
}

impl StringB {
    pub fn new() -> StringB {
        Self { output: String::new(), ..Default::default() }
    }
}

impl Backend for StringB {
    fn get_size(&self) -> Result<Cursor> {
        Ok(self.size)
    }

    fn move_cursor(&mut self, pos: Cursor) {
        todo!()
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

    fn draw(&mut self, buffer: &crate::terminal::buffer::Buffer) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }
}
