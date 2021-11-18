use super::modifier::Modifier;

/// Character in terminal cell with modifiers
#[derive(Debug, Default, Copy, Clone)]
pub struct Character {
    symbol: char,
    modifier: Modifier,
}
