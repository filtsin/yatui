use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
    slice::Iter,
};

/// Character in terminal cell with modifiers
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Character {
    symbol: char,
}

impl Character {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }

    pub fn symbol(self) -> char {
        self.symbol
    }

    pub fn update_symbol(self, symbol: char) -> Self {
        Self { symbol, ..self }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Modifiers
        f.write_char(self.symbol)
    }
}

impl From<char> for Character {
    fn from(s: char) -> Self {
        Self::new(s)
    }
}

impl Default for Character {
    fn default() -> Self {
        Self { symbol: ' ' }
    }
}

#[derive(Debug)]
pub struct Characters(pub Vec<Character>);

impl<S> From<S> for Characters
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let res = s.as_ref().chars().map(Character::new).collect();
        Self(res)
    }
}

impl Deref for Characters {
    type Target = Vec<Character>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Characters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
