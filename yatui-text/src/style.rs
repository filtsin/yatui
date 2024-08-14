use bitflags::bitflags;

/// `Style` of displayed graphemes in terminal way. In terminal you can set [`foreground`] and
/// [`background`] [`colors`]. Also you can change [`modifiers`], e.g. use bold font, underlined,
/// etc. Look at all modifiers in [`modifier`] struct.
///
/// [`foreground`]: Self::fg
/// [`background`]: Self::bg
/// [`colors`]: Color
/// [`modifiers`]: Self::modifier
/// [`modifier`]: Modifier
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default, Hash)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub modifier: Modifier,
}

/// `Color` of displayed graphemes. Full support of
/// [ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) colors
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Color {
    #[default]
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // TODO: Add 24bit colors, may be 8bit too
    Rgb(u8, u8, u8),
}

bitflags! {
    /// `Modifier` of displayed graphemes. Allows to display bold, underlined, etc. text. All
    /// modifiers can be composed with bit-or(|) operation.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
    pub struct Modifier : u16 {
        const BOLD = 0x1;
        const ITALIC = 0x2;
        // TODO: Add more modifiers
    }
}

// TODO: Impl style
impl Style {
    /// Creates empty `Style`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style = Style::new();
    /// assert_eq!(style.fg, None);
    /// assert_eq!(style.bg, None);
    /// assert_eq!(style.modifier, Modifier::empty());
    /// ```
    pub const fn new() -> Self {
        Self { bg: None, fg: None, modifier: Modifier::empty() }
    }

    /// Set foreground color.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style = Style::new().fg(Color::Green);
    /// assert_eq!(style.fg, Some(Color::Green));
    /// ```
    #[must_use = "`Style` object is immutable. All mutation methods return new object"]
    pub const fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Set background color.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style = Style::new().bg(Color::Green);
    /// assert_eq!(style.bg, Some(Color::Green));
    /// ```
    #[must_use = "`Style` object is immutable. All mutation methods return new object"]
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Set modifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style = Style::new().modifier(Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(style.modifier, Modifier::BOLD | Modifier::ITALIC);
    /// ```
    #[must_use = "`Style` object is immutable. All mutation methods return new object"]
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Merge styles from `rhs` to `self`. All styles from `rhs` are applied to `self` if they're
    /// not None. Modifiers are also merge.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style1 = Style::new().fg(Color::Green);
    /// let style2 = Style::new().bg(Color::Yellow);
    /// assert_eq!(style1.merge(style2), Style::new().fg(Color::Green).bg(Color::Yellow));
    /// ```
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style1 = Style::new().fg(Color::Green);
    /// let style2 = Style::new().fg(Color::Yellow);
    /// assert_eq!(style1.merge(style2), Style::new().fg(Color::Yellow));
    /// ```
    ///
    /// ```
    /// # use yatui_text::*;
    /// let style1 = Style::new().modifier(Modifier::BOLD);
    /// let style2 = Style::new().modifier(Modifier::ITALIC);
    /// assert_eq!(style1.merge(style2), Style::new().modifier(Modifier::BOLD | Modifier::ITALIC));
    #[must_use = "`Style` object is immutable. All mutation methods return new object"]
    pub fn merge(mut self, rhs: Style) -> Self {
        if let Some(fg) = rhs.fg {
            self.fg = Some(fg);
        }

        if let Some(bg) = rhs.bg {
            self.bg = Some(bg);
        }

        self.modifier |= rhs.modifier;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_style_sizeof() {
        assert_eq!(std::mem::size_of::<Color>(), 4);
        assert_eq!(std::mem::size_of::<Style>(), 10);
    }
}
