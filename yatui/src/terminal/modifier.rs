/// Modifier of [super::character::Character]. Allows to change color and style
#[derive(Debug, Copy, Clone, Default)]
pub struct Modifier {
    color: Option<Color>,
    // just for prototype, TODO: replace it for applying multiple styles
    style: Option<Style>,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Color {
    Black,
    White,
    Rgb(u8, u8, u8),
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Style {
    Bold,
    Italic,
}
