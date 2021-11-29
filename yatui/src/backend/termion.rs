use crate::{
    backend::Backend,
    error::Result,
    terminal::{buffer::Buffer, cursor::Index},
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
    fn get_size(&self) -> Result<(Index, Index)> {
        termion::terminal_size().map_err(|e| e.into())
    }

    fn move_cursor(&mut self, pos: (Index, Index)) {
        write!(self.output, "{}", cursor::Goto(pos.0, pos.1)).unwrap();
    }

    fn clear_screen(&mut self) {
        write!(self.output, "{}", clear::All).unwrap();
        self.move_cursor((1, 1));
    }

    fn draw(&mut self, buffer: Buffer) {
        write!(self.output, "{}", buffer).unwrap();
    }

    fn flush(&mut self) {
        self.output.flush().unwrap();
    }
}
