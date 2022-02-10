use super::{Buffer, MappedStateBuffer};

use crate::{
    error::Error,
    terminal::{
        character::Character,
        cursor::{Cursor, Index},
        modifier::Modifier,
        region::Region,
    },
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
    pub(crate) fn new(buffer: &'a mut Buffer, mapped_region: Region) -> Self {
        Self { buffer, mapped_region }
    }

    pub fn map(&mut self, region: Region) -> MappedBuffer<'_> {
        MappedBuffer { buffer: self.buffer, mapped_region: region }
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

    pub fn write_character<C>(&mut self, c: C, cursor: Cursor)
    where
        C: Into<Character>,
    {
        self.buffer.write_character(c, self.global_cursor(cursor).unwrap());
    }

    pub fn try_write_character<C>(&mut self, c: C, cursor: Cursor) -> Result<(), Error>
    where
        C: Into<Character>,
    {
        let global_cursor = self.global_cursor(cursor).ok_or_else(|| {
            Error::Buffer(format!(
                "Cursor {:?} not included in the mapped region {:?}",
                cursor, self.mapped_region
            ))
        });

        self.buffer.write_character(c, global_cursor?);
        Ok(())
    }

    pub fn fill<C>(&mut self, c: C)
    where
        C: Into<Character>,
    {
        let character = c.into();

        for cursor in self.region().into_iter() {
            println!("{:?}", cursor);
            self.buffer.write_character(character, cursor);
        }
    }

    pub fn get(&self, cursor: Cursor) -> &Character {
        self.buffer.get(self.global_cursor(cursor).unwrap())
    }

    pub fn get_mut(&mut self, cursor: Cursor) -> &mut Character {
        self.buffer.get_mut(self.global_cursor(cursor).unwrap())
    }

    pub fn region(&self) -> Region {
        self.mapped_region
    }

    pub fn padding(self, padding: Index) -> MappedBuffer<'a> {
        todo!()
    }

    // Converts local row to the global
    fn global_row(&self, local_row: Index) -> Option<Index> {
        if local_row <= self.mapped_region.height() {
            Some(local_row + self.mapped_region.left_top().row())
        } else {
            None
        }
    }

    // Converts local column to the global
    fn global_column(&self, local_column: Index) -> Option<Index> {
        if local_column <= self.mapped_region.width() {
            Some(local_column + self.mapped_region.left_top().column())
        } else {
            None
        }
    }

    fn global_cursor(&self, local_cursor: Cursor) -> Option<Cursor> {
        Some(Cursor::new(
            self.global_column(local_cursor.column())?,
            self.global_row(local_cursor.row())?,
        ))
    }
}
