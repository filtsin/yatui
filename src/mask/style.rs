use bitflags::bitflags;

/// Style of terminal cell. This structure is used to change foreground color,
/// background color and add modifiers (like italic, underline etc.)
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default, Hash)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    modifier: Modifier,
}

// TODO: Add doc
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    // TODO: Add doc
    #[derive(Default)]
    pub struct Modifier : u16 {
        const BOLD = 1 << 0;
        const ITALIC = 1 << 1;
        const UNDERLINE = 1 << 2;
    }
}

impl Style {
    /// Create empty `Style` with no foreground and background colors, with empty modifiers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Change foreground color
    ///
    /// ```
    /// use yatui::mask::{Style, Color};
    ///
    /// let style = Style::new().fg(Color::Black);
    /// ```
    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Change background color
    ///
    /// ```
    /// use yatui::mask::{Style, Color};
    ///
    /// let style = Style::new().bg(Color::Black);
    /// ```
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Change modifier
    ///
    /// ```
    /// use yatui::mask::{Style, Modifier};
    ///
    /// let style = Style::new().modifier(Modifier::BOLD | Modifier::ITALIC);
    /// ```
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Clear foreground color
    pub fn clear_fg(&mut self) -> Self {
        self.fg = None;
        *self
    }

    /// Clear background color
    pub fn clear_bg(&mut self) -> Self {
        self.bg = None;
        *self
    }

    /// Clear modifier
    pub fn clear_modifier(&mut self) -> Self {
        self.modifier = Modifier::default();
        *self
    }

    /// Replace `self` foreground and background
    /// with `rhs`, add all modifiers from `rhs` to `self`.
    ///
    /// ```
    /// use yatui::mask::{Style, Color, Modifier};
    ///
    /// let style1 = Style::new().bg(Color::Black).fg(Color::Black).modifier(Modifier::BOLD);
    /// let style2 = Style::new().bg(Color::White).fg(Color::White).modifier(Modifier::ITALIC);
    /// let new_style = style1.merge(style2);
    ///
    /// assert_eq!(new_style, style2.modifier(Modifier::BOLD | Modifier::ITALIC));
    /// ```
    pub fn merge(mut self, rhs: Style) -> Self {
        if rhs.fg.is_some() {
            self.fg = rhs.fg;
        }

        if rhs.bg.is_some() {
            self.bg = rhs.bg;
        }

        self.modifier |= rhs.modifier;

        self
    }

    /// Get current foreground color
    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    /// Get current background color
    pub fn get_bg(&self) -> Option<Color> {
        self.bg
    }

    /// Get current modifiers
    pub fn get_modifier(&self) -> Modifier {
        self.modifier
    }
}
