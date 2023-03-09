use crate::{
    backend::Backend,
    error::Result,
    terminal::{Cursor, Size},
    text::Style,
};

use log::info;
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};

use std::io::{BufWriter, Write};

pub struct Termion<W: Write> {
    pub output: AlternateScreen<RawTerminal<BufWriter<W>>>,
}

impl<W: Write> Termion<W> {
    pub fn new(output: W) -> Result<Termion<W>> {
        let output =
            BufWriter::with_capacity(5_000_000, output).into_raw_mode()?.into_alternate_screen()?;
        Ok(Termion { output })
    }
}

impl<W: Write + Send> Backend for Termion<W> {
    fn get_size(&self) -> Result<Size> {
        let (column, row) = termion::terminal_size()?;
        info!("Return size {}, {}", column, row);
        Ok(Size::new(column, row))
    }

    fn move_cursor(&mut self, pos: Cursor) {
        let based1_pos = pos.next_line().next_column();
        write!(self.output, "{}", cursor::Goto(based1_pos.column(), based1_pos.line())).unwrap();
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

    fn draw(&mut self, s: &str, style: Style) {
        // TODO: Affect style
        write!(self.output, "{}", s).unwrap()
    }

    fn flush(&mut self) {
        self.output.flush().unwrap();
    }
}
