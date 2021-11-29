use std::fmt::{Display, Write};

use super::modifier::Modifier;

/// Character in terminal cell with modifiers
#[derive(Debug, Default, Copy, Clone)]
pub struct Character {
    symbol: char,
    modifier: Modifier,
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Modifiers
        f.write_char(self.symbol)
    }
}
