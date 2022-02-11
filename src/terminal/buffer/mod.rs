mod map;
mod state;

use std::fmt::Display;

pub use map::MappedBuffer;
pub use state::MappedStateBuffer;

use crate::terminal::{
    character::Character,
    cursor::{Cursor, Index},
    region::Region,
};

/// Global buffer for terminal
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Buffer {
    /// Chars for every column and row, size should be = `region`.width() * `region`.height()
    data: Vec<Character>,
    /// Current terminal region, left_top is always 0
    region: Region,
}

impl Buffer {
    /// Creates a new buffer from `last_pos`
    pub fn new(size: Cursor) -> Self {
        let region = Region::new(Cursor::default(), size);
        let data = vec![Character::default(); region.area() as usize];
        Self { data, region }
    }

    pub fn map(&mut self, region: Region) -> MappedBuffer<'_> {
        MappedBuffer::new(self, region)
    }

    pub fn full_map(&mut self) -> MappedBuffer<'_> {
        let right_cursor = self.region.right_bottom().prev_row().prev_column();
        self.map(Region::new(Cursor::default(), right_cursor))
    }

    /// Updates `region` for current buffer.
    /// Useful for updating buffer in place when resizing terminal
    pub fn resize(&mut self, last_pos: Cursor) {
        let region = Region::new(Cursor::default(), last_pos);
        self.data.resize_with(region.area(), Character::default);
        self.region = region;
    }

    /// Write `c` in specified `position`
    pub fn write_character<C>(&mut self, c: C, position: Cursor)
    where
        C: Into<Character>,
    {
        let index = self.get_index(&position);
        self.data[index] = c.into();
    }

    pub fn get(&self, cursor: Cursor) -> &Character {
        &self.as_ref()[self.get_index(&cursor)]
    }

    pub fn get_mut(&mut self, cursor: Cursor) -> &mut Character {
        let index = self.get_index(&cursor);
        &mut self.as_mut()[index]
    }

    pub fn absorb(self) -> Vec<Character> {
        self.data
    }

    /// Returns current size of buffer
    pub fn size(&self) -> Cursor {
        Cursor::new(self.region.width(), self.region.height())
    }

    // get index for `data` vec for specified `cursor`
    pub fn get_index(&self, cursor: &Cursor) -> usize {
        self.region.width() as usize * cursor.row() as usize + cursor.column() as usize
    }
}

impl AsRef<[Character]> for Buffer {
    fn as_ref(&self) -> &[Character] {
        self.data.as_ref()
    }
}

impl AsMut<[Character]> for Buffer {
    fn as_mut(&mut self) -> &mut [Character] {
        self.data.as_mut()
    }
}

impl<S> From<Vec<S>> for Buffer
where
    S: AsRef<str>,
{
    fn from(vec: Vec<S>) -> Self {
        let h = vec.len() as Index;
        let w = vec.iter().map(|v| v.as_ref().chars().count()).max().unwrap() as Index;

        println!("{}-{}", w, h);

        let mut res = Self::new(Cursor::new(w, h));
        let mut mapped = res.full_map();

        for (line, text) in vec.iter().enumerate() {
            println!("{:?} : {:?}", line, text.as_ref());
            mapped.write_line(text, line as Index);
        }

        res
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, el) in self.data.iter().enumerate() {
            write!(f, "{}", el)?;
            if i == self.region.right_bottom().column().into() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
