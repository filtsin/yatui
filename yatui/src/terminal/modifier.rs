use bitflags::bitflags;

/// Modifier of [super::character::Character]. Allows to change color and style
#[derive(Debug, Copy, Clone, Default)]
pub struct Modifier {
    color: Option<Color>,
    background: Option<Color>,
    style: Style,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Color {
    Black,
    White,
    Rgb(u8, u8, u8),
}

bitflags! {
    #[derive(Default)]
    pub struct Style : u32 {
        const BOLD = 0b00000001;
        const ITALIC = 0b00000010;
        const UNDERLINE = 0b00000100;
    }
}
