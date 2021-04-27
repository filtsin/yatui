//! Backend trait

mod backend;
mod termion;

pub use backend::Backend;
pub use self::termion::Termion;
use std::io::{Write, Result};
