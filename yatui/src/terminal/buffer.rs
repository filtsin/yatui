use super::region::Region;

/// Global buffer for terminal
pub struct Buffer {
    /// Chars for every column and row, size should be = `region`.width() * `region`.height()
    data: Vec<char>,
    /// Current terminal region
    region: Region
}

/// Mapped buffer to some position of terminal
/// Converts local coordinates of widget to global coordinates of terminal
pub struct MappedBuffer<'a> {
    buffer: &'a mut Buffer,
    mapped_region: Region
}

impl Buffer {
    /// Creates a new buffer for `region`
    pub fn new(region: Region) -> Self {
        let mut data = vec![' '; region.area() as usize];
        Self {
            data,
            region
        }
    }
    /// Updates `region` for current buffer.
    /// Useful for updating buffer in place when resizing terminal
    pub fn update_region(&mut self, region: Region) {
        self.data.resize_with(region.area() as usize, || ' ');
        self.region = region;
    }
}

impl<'a> MappedBuffer<'a> {
    /// Creates a new mapped buffer
    pub fn new(buffer: &'a mut Buffer, mapped_region: Region) -> Self {
        Self { buffer, mapped_region }
    }
    /// Converts local row to the global
    fn global_row(&self, local_row: u8) -> u8 {
        todo!()
    }
    /// Converts local column to the global
    fn global_column(&self, local_column: u8) -> u8 {
        todo!()
    }
}

