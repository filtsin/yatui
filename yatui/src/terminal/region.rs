use super::cursor::Cursor;

/// Region represents an area in the terminal
pub struct Region {
    left_top: Cursor,
    right_bottom: Cursor
}

impl Region {
    pub fn new(left_top: Cursor, right_bottom: Cursor) -> Self {
        Self { left_top, right_bottom }
    }
    ///
    pub fn width() -> u8 {
        todo!()
    }
}
