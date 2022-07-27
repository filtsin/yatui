use bitflags::bitflags;

/// Modifier of [super::character::Character]. Allows to change color and style
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Modifier {
    color: Option<Color>,
    background: Option<Color>,
    style: Style,
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
    pub struct Style : u16 {
        const BOLD = 0b00000001;
        const ITALIC = 0b00000010;
        const UNDERLINE = 0b00000100;
    }
}
