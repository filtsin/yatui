use super::{Buffer, MappedStateBuffer};

use crate::terminal::{
    character::Character,
    cursor::{Cursor, Index},
    modifier::Modifier,
    region::Region,
};

/// Mapped buffer is a safe abstraction over `Buffer`. It contain only specified in `mapped_region`
/// region. You can not write to other positions which not be mapped.
/// Converts local coordinates of widget to global coordinates of terminal.
#[derive(Debug)]
pub struct MappedBuffer<'a> {
    buffer: &'a mut Buffer,
    mapped_region: Region,
}

impl<'a> MappedBuffer<'a> {
    /// Creates a new mapped buffer
    pub fn new(buffer: &'a mut Buffer, mapped_region: Region) -> Self {
        Self { buffer, mapped_region }
    }

    pub fn with_state(self, state: usize) -> MappedStateBuffer<'a> {
        // TODO: Check state for overflow region
        MappedStateBuffer::new(self, state)
    }

    pub fn with_state_cursor(self, cursor: Cursor) -> MappedStateBuffer<'a> {
        todo!()
    }

    // border with specified character. return buffer inside this border
    pub fn draw_border(self, size: usize, c: Character) -> MappedBuffer<'a> {
        todo!()
    }

    // style for all characters
    pub fn set_style(&mut self, style: Modifier) {
        todo!()
    }

    pub fn write_character(&mut self, c: Character, cursor: Cursor) {
        self.buffer.write_in(c, cursor);
    }

    pub fn region(&self) -> Region {
        self.mapped_region
    }

    pub fn padding(self, padding: Index) -> MappedBuffer<'a> {
        todo!()
    }

    // Converts local row to the global
    fn global_row(&self, local_row: Index) -> Index {
        local_row + self.mapped_region.left_top.row()
    }
    // Converts local column to the global
    fn global_column(&self, local_column: Index) -> Index {
        local_column + self.mapped_region.left_top.column()
    }
}
