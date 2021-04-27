pub struct Style {
    background: Option<Color>,
    text: Option<Color>
}

pub enum Color {
    Black,
    White
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            text: None
        }
    }
}
