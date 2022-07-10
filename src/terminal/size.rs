use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::terminal::cursor::Index;

/// Width and height of widget
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Default, Copy, Clone)]
pub struct Size {
    w: Index,
    h: Index,
}

impl Size {
    /// Construct new [`Size`]
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

    pub fn area(&self) -> usize {
        self.w as usize * self.h as usize
    }

    #[must_use]
    pub fn add_width_size(self, rhs: Self) -> Self {
        Self { w: self.w.saturating_add(rhs.w), h: std::cmp::max(self.h, rhs.h) }
    }

    #[must_use]
    pub fn add_height_size(self, rhs: Self) -> Self {
        Self { w: std::cmp::max(self.w, rhs.w), h: self.h.saturating_add(rhs.h) }
    }

    pub fn add_assign_width_size(&mut self, rhs: Self) {
        *self = self.add_width_size(rhs);
    }

    pub fn add_assign_height_size(&mut self, rhs: Self) {
        *self = self.add_height_size(rhs);
    }
}

// impl Add for Size {
//     type Output = Size;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Self::Output { w: self.w.saturating_add(rhs.w), h: self.h.saturating_add(rhs.h) }
//     }
// }
//
// impl AddAssign for Size {
//     fn add_assign(&mut self, rhs: Self) {
//         self.w = self.w.saturating_add(rhs.w);
//         self.h = self.h.saturating_add(rhs.h);
//     }
// }
//
// impl Sub for Size {
//     type Output = Size;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self::Output { w: self.w.saturating_sub(rhs.w), h: self.h.saturating_sub(rhs.h) }
//     }
// }
//
// impl SubAssign for Size {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.w = self.w.saturating_sub(rhs.w);
//         self.h = self.h.saturating_sub(rhs.h);
//     }
// }
