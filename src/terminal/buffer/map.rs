use super::Buffer;

use crate::{
    error::Error,
    terminal::{
        character::{Character, Characters},
        cursor::{Cursor, Index},
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
        let global_left = self.global_cursor(region.left_top()).unwrap();
        let global_right = self.global_cursor(region.right_bottom()).unwrap();
        MappedBuffer { buffer: self.buffer, mapped_region: Region::new(global_left, global_right) }
    }

    pub fn map_line(&mut self, line: Index) -> MappedBuffer<'_> {
        self.map(Region::new(Cursor::new(0, line), Cursor::new(self.region().width() - 1, line)))
    }

    pub fn map_column(&mut self, column: Index) -> MappedBuffer<'_> {
        self.map(Region::new(
            Cursor::new(column, 0),
            Cursor::new(column, self.region().height() - 1),
        ))
    }

    pub fn write_character<C>(&mut self, c: C, cursor: Cursor)
    where
        C: Into<Character>,
    {
        let character = c.into();
        self.buffer.write_character(character, self.global_cursor(cursor).unwrap());
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
            self.buffer.write_character(character, cursor);
        }
    }

    pub fn write_line<C>(&mut self, c: C, line: Index)
    where
        C: Into<Characters>,
    {
        let new_region =
            Region::new(Cursor::new(0, line), Cursor::new(self.region().width() - 1, line));

        let mut mapped_buffer = self.map(new_region);
        mapped_buffer.clear();

        let mut cursor = Cursor::default();

        for character in c.into().0.into_iter() {
            mapped_buffer.write_character(character, cursor);
            cursor = cursor.next_column();
        }
    }

    pub fn write_column<C>(&mut self, c: C, column: Index)
    where
        C: Into<Characters>,
    {
        let new_region =
            Region::new(Cursor::new(column, 0), Cursor::new(column, self.region().height() - 1));

        let mut mapped_buffer = self.map(new_region);
        mapped_buffer.clear();

        let mut cursor = Cursor::default();

        for character in c.into().0.into_iter() {
            mapped_buffer.write_character(character, cursor);
            cursor = cursor.next_line();
        }
    }

    pub fn clear(&mut self) {
        for cursor in self.region().into_iter() {
            self.buffer.write_character(Character::default(), cursor);
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
        if local_row < self.mapped_region.height() {
            Some(local_row + self.mapped_region.left_top().line())
        } else {
            None
        }
    }

    // Converts local column to the global
    fn global_column(&self, local_column: Index) -> Option<Index> {
        if local_column < self.mapped_region.width() {
            Some(local_column + self.mapped_region.left_top().column())
        } else {
            None
        }
    }

    fn global_cursor(&self, local_cursor: Cursor) -> Option<Cursor> {
        Some(Cursor::new(
            self.global_column(local_cursor.column())?,
            self.global_row(local_cursor.line())?,
        ))
    }
}
