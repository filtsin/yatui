use bitflags::bitflags;

/// Modifier of [super::character::Character]. Allows to change color and style
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    modifier: Modifier,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Color {
    Black,
    White,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    Yellow,
    Rgb(u8, u8, u8),
}

bitflags! {
    #[derive(Default)]
    pub struct Modifier : u16 {
        const BOLD = 0b00000001;
        const ITALIC = 0b00000010;
        const UNDERLINE = 0b00000100;
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(&mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        *self
    }

    pub fn bg(&mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        *self
    }

    pub fn modifier(&mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        *self
    }

    pub fn clear_fg(&mut self) -> Self {
        self.fg = None;
        *self
    }

    pub fn clear_bg(&mut self) -> Self {
        self.bg = None;
        *self
    }

    pub fn clear_modifier(&mut self) -> Self {
        self.modifier = Modifier::default();
        *self
    }

    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    pub fn get_bg(&self) -> Option<Color> {
        self.bg
    }

    pub fn get_modifier(&self) -> Modifier {
        self.modifier
    }
}
