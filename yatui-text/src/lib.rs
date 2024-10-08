#![allow(dead_code)] // TODO: dev stage
#![allow(unused)] // TODO: dev stage

pub mod mask;
mod raw;
pub mod style;
pub mod text;

pub use mask::{IdxRange, Mask};
pub use style::{Color, Modifier, Style};
pub use text::Text;
