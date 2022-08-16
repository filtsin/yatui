use crate::{
    backend::Backend,
    terminal::{cursor::Cursor, region::Region},
    text::Text,
};

use super::Printer;

pub struct RawPrinter<'a> {
    backend: &'a mut dyn Backend,
}

impl<'a> RawPrinter<'a> {
    pub fn new(backend: &'a mut dyn Backend) -> Self {
        Self { backend }
    }

    pub fn map(&mut self, region: Region) -> Printer<'_> {
        todo!()
    }

    pub fn print(&mut self, text: &Text, pos: Cursor) {
        todo!()
    }

    pub fn print_str(&mut self, text: &str, pos: Cursor) {
        todo!()
    }
}
