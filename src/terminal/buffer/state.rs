use std::fmt::Display;

use crate::terminal::{character::Character, cursor::Cursor};

use super::{Buffer, MappedBuffer};

/// Mapped buffer with state
#[derive(Debug)]
pub struct MappedStateBuffer<'a> {
    buffer: MappedBuffer<'a>,
    state: usize,
}

impl<'a> MappedStateBuffer<'a> {
    pub fn new(buffer: MappedBuffer<'a>, state: usize) -> Self {
        Self { buffer, state }
    }

    pub fn without_state(self) -> MappedBuffer<'a> {
        self.buffer
    }
    pub fn write_text(mut self, text: &str) -> Self {
        // TODO: Now it is incorrect version just for debug
        let mut cursor = Cursor::default();
        for c in text.chars() {
            let character = Character::new(c);
            self.buffer.write_character(character, cursor);
            cursor = cursor.next_column();
        }

        self
    }
    pub fn write_text_overflow(&mut self, text: &str, overflow: &str) -> Self {
        todo!()
    }
    pub fn next_row(&mut self) -> Self {
        todo!()
    }
    pub fn next_column(&mut self) -> Self {
        todo!()
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.data.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}
