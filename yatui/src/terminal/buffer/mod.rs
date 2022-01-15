mod map;
mod state;

pub use map::MappedBuffer;
pub use state::MappedStateBuffer;

use crate::terminal::{character::Character, cursor::Cursor, region::Region};

/// Global buffer for terminal
#[derive(Debug, Default)]
pub struct Buffer {
    /// Chars for every column and row, size should be = `region`.width() * `region`.height()
    pub data: Vec<Character>,
    /// Current terminal region, left_top is always 0
    region: Region,
}

impl Buffer {
    /// Creates a new buffer from `last_pos`
    pub fn new(last_pos: Cursor) -> Self {
        let region = Region::new(Cursor::default(), last_pos);
        let data = vec![Character::default(); region.area() as usize];
        Self { data, region }
    }
    /// Updates `region` for current buffer.
    /// Useful for updating buffer in place when resizing terminal
    pub fn update_size(&mut self, last_pos: Cursor) {
        let region = Region::new(Cursor::default(), last_pos);
        self.data.resize_with(region.area(), Character::default);
        self.region = region;
    }
    /// Write `c` in specified `position`
    pub fn write_in(&mut self, c: Character, position: Cursor) {
        let index = self.get_index(&position);
        self.data[index] = c;
    }
    /// Returns current size of buffer
    pub fn get_size(&self) -> Cursor {
        Cursor::new(self.region.width(), self.region.height())
    }
    // get index for `data` vec for specified `cursor`
    fn get_index(&self, cursor: &Cursor) -> usize {
        self.region.width() as usize * cursor.row() as usize + cursor.column() as usize
    }
}
