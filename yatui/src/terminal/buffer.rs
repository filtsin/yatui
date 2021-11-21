use super::{
    character::Character,
    cursor::{Cursor, Index},
    modifier::Modifier,
    region::Region,
};

/// Global buffer for terminal
#[derive(Debug)]
pub struct Buffer {
    /// Chars for every column and row, size should be = `region`.width() * `region`.height()
    data: Vec<Character>,
    /// Current terminal region
    region: Region,
}

/// Mapped buffer is a safe abstraction over `Buffer`. It contains only specified in `mapped_region`
/// region. You can not write to other positions which not be mapped. Contains cursor inside
/// which points to the position where last writing happened.
/// Converts local coordinates of widget to global coordinates of terminal.
#[derive(Debug)]
pub struct MappedBuffer<'a> {
    buffer: &'a mut Buffer,
    mapped_region: Region,
    cursor: Option<usize>,
}

impl Buffer {
    /// Creates a new buffer for `region`
    pub fn new(region: Region) -> Self {
        let data = vec![Character::default(); region.area() as usize];
        Self { data, region }
    }
    /// Updates `region` for current buffer.
    /// Useful for updating buffer in place when resizing terminal
    pub fn update_region(&mut self, region: Region) {
        self.data.resize_with(region.area(), Character::default);
        self.region = region;
    }
    /// Write `c` in specified `position`
    pub fn write_in(&mut self, cell: Character, position: Cursor) {
        let index = self.get_index(&position);
        self.data[index] = cell;
    }
    // get index for `data` vec for specified `cursor`
    fn get_index(&self, cursor: &Cursor) -> usize {
        self.region.width() as usize * cursor.row() as usize + cursor.column() as usize
    }
}

impl<'a> MappedBuffer<'a> {
    /// Creates a new mapped buffer
    pub fn new(buffer: &'a mut Buffer, mapped_region: Region) -> Self {
        Self { buffer, mapped_region, cursor: None }
    }

    // write from `cursor` start position
    pub fn write_text(&mut self, text: &str) -> usize {
        todo!()
    }

    // border with specified character. return buffer inside this border
    pub fn draw_border(self, c: Character) -> MappedBuffer<'a> {
        todo!()
    }

    // style for all characters
    pub fn set_style(&mut self, style: Modifier) {
        todo!()
    }

    /// Converts local row to the global
    fn global_row(&self, local_row: Index) -> Index {
        local_row + self.mapped_region.left_top.row()
    }
    /// Converts local column to the global
    fn global_column(&self, local_column: Index) -> Index {
        local_column + self.mapped_region.left_top.column()
    }
}
