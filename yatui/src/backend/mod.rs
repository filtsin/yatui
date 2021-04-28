//! Backend trait

mod backend;
mod termion;

pub use self::termion::Termion;
pub use backend::Backend;
use std::io::{Result, Write};
