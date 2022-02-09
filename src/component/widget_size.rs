use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::terminal::cursor::Index;

/// Width and height of widget
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Default, Copy, Clone)]
pub struct WidgetSize {
    w: Index,
    h: Index,
}

impl WidgetSize {
    /// Construct new [`WidgetSize`]
    pub fn new(w: Index, h: Index) -> Self {
        Self { w, h }
    }

    pub fn max() -> Self {
        Self::new(Index::MAX, Index::MAX)
    }

    pub fn min() -> Self {
        Self::default()
    }

    pub fn width(&self) -> Index {
        self.w
    }

    pub fn height(&self) -> Index {
        self.h
    }
}

impl Add for WidgetSize {
    type Output = WidgetSize;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { w: self.w.saturating_add(rhs.w), h: self.h.saturating_add(rhs.h) }
    }
}

impl AddAssign for WidgetSize {
    fn add_assign(&mut self, rhs: Self) {
        self.w = self.w.saturating_add(rhs.w);
        self.h = self.h.saturating_add(rhs.h);
    }
}

impl Sub for WidgetSize {
    type Output = WidgetSize;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { w: self.w.saturating_sub(rhs.w), h: self.h.saturating_sub(rhs.h) }
    }
}

impl SubAssign for WidgetSize {
    fn sub_assign(&mut self, rhs: Self) {
        self.w = self.w.saturating_sub(rhs.w);
        self.h = self.h.saturating_sub(rhs.h);
    }
}
