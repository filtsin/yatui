#![allow(dead_code)] // TODO: dev stage
#![allow(unused)] // TODO: dev stage

pub mod idx_range;
pub mod mask;
pub mod style;

pub use idx_range::IdxRange;
pub use mask::Mask;
pub use style::{Color, Modifier, Style};
