use crate::{
    backend::Backend,
    error::Result,
    terminal::{buffer::Buffer, cursor::Cursor},
};

use termion::{
    clear, cursor,
    raw::{IntoRawMode, RawTerminal},
};

use std::io::{BufWriter, Write};

pub struct Termion<W: Write> {
    output: RawTerminal<BufWriter<W>>,
}

impl<W: Write> Termion<W> {
    pub fn new(output: W) -> Result<Termion<W>> {
        let output = BufWriter::with_capacity(5_000_000, output).into_raw_mode()?;
        Ok(Termion { output })
    }
}

impl<W: Write + Send> Backend for Termion<W> {
    fn get_size(&self) -> Result<Cursor> {
        let (column, row) = termion::terminal_size()?;
        Ok(Cursor::new(column, row))
    }

    fn move_cursor(&mut self, pos: Cursor) {
        let based1_pos = pos.next_row().next_column();
        write!(self.output, "{}", cursor::Goto(based1_pos.row(), based1_pos.column())).unwrap();
    }

    fn hide_cursor(&mut self) {
        write!(self.output, "{}", cursor::Hide).unwrap()
    }

    fn show_cursor(&mut self) {
        write!(self.output, "{}", cursor::Show).unwrap()
    }

    fn clear_screen(&mut self) {
        write!(self.output, "{}", clear::All).unwrap();
    }

    fn draw(&mut self, buffer: &Buffer) {
        write!(self.output, "{}", buffer).unwrap();
    }

    fn flush(&mut self) {
        self.output.flush().unwrap();
    }
}
