pub(crate) mod event;

use std::collections::HashMap;

use self::event::Event;
use crate::{
    backend::Backend,
    terminal::{
        buffer::{Buffer, MappedBuffer},
        cursor::{Cursor, Index},
        region::Region,
    },
};

#[derive(Debug)]
pub(crate) struct Compositor<B> {
    backend: B,

    buffer: Buffer,

    next_id: u32,
}

impl<B> Compositor<B> {
    pub(crate) fn new(backend: B) -> Self {
        todo!()
    }
}

impl<B> Compositor<B>
where
    B: Backend,
{
    pub(crate) fn draw(&mut self) {
        // if let Some(id) = self.active {
        //     let size = self.buffer.get_size();
        //     let (w, h) = (size.row(), size.column());
        //     let mapped_region = Region::new(Cursor::default(), Cursor::new(w, h));
        //     let mapped_buffer = MappedBuffer::new(&mut self.buffer, mapped_region);
        //
        //     let page = self.pages.get_mut(&id).unwrap();
        //     page.main_widget.draw(mapped_buffer);
        //
        //     self.backend.hide_cursor();
        //     self.backend.clear_screen();
        //     self.backend.draw(&self.buffer);
        //     self.backend.flush();
        // }
    }
    pub(crate) fn process_event(&mut self, event: Event) {
        match event {
            _ => {}
        };
    }
}
