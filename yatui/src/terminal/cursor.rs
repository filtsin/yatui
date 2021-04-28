/// Cursor structure points to a row and column of terminal
pub struct Cursor {
    row: u8,
    column: u8
}

impl Cursor {
    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }
}
