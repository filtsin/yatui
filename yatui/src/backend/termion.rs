use crate::backend::Backend;
use crate::error::Result;

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

use termion::clear;
use termion::cursor;

use std::io::BufWriter;

use std::fs::File;
use std::io::{Read, Stdout, Write};

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
    fn get_size(&self) -> Result<(u16, u16)> {
        termion::terminal_size().map_err(|e| e.into())
    }

    fn move_cursor(&mut self, pos: (u16, u16)) {
        write!(self.output, "{}", cursor::Goto(pos.0, pos.1)).unwrap();
    }

    fn clear_screen(&mut self) {
        write!(self.output, "{}", clear::All).unwrap();
        self.move_cursor((1, 1));
    }

    fn draw(&mut self, str: &str) {
        write!(self.output, "{}", str).unwrap();
    }

    fn flush(&mut self) {
        self.output.flush().unwrap();
    }
}
