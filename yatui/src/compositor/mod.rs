pub(crate) mod event;

use std::collections::HashMap;

use self::event::Event;
use crate::{
    app::page::{Id, Page},
    backend::Backend,
    terminal::{
        buffer::{Buffer, MappedBuffer},
        cursor::{Cursor, Index},
        region::Region,
    },
    widget::Widget,
};

#[derive(Debug)]
pub(crate) struct Compositor<B> {
    backend: B,
    pages: HashMap<Id, Page>,
    active: Option<Id>,

    buffer: Buffer,

    next_id: u32,
}

impl<B> Compositor<B> {
    pub(crate) fn new(backend: B) -> Self {
        Self {
            backend,
            pages: HashMap::new(),
            active: None,
            next_id: 0,
            buffer: Buffer::new(Cursor::new(20, 20)),
        }
    }
}

impl<B> Compositor<B>
where
    B: Backend,
{
    pub(crate) fn draw(&mut self) {
        if let Some(id) = self.active {
            let size = self.buffer.get_size();
            let (w, h) = (size.row(), size.column());
            let mapped_region = Region::new(Cursor::default(), Cursor::new(w, h));
            let mapped_buffer = MappedBuffer::new(&mut self.buffer, mapped_region);

            let page = self.pages.get_mut(&id).unwrap();
            page.main_widget.draw(mapped_buffer);

            self.backend.hide_cursor();
            self.backend.clear_screen();
            self.backend.draw(&self.buffer);
            self.backend.flush();
        }
    }
    pub(crate) async fn process_event(&mut self, event: Event) {
        match event {
            Event::SetActive(id) => self.set_active(id),
            Event::AddPage(page, channel) => channel.send(self.add_page(page)).unwrap(),
            Event::ChangeSize(size) => self.change_size(size),
            Event::Draw => self.draw(),
        };
    }

    pub(crate) fn add_page(&mut self, page: Page) -> Id {
        let id = Id::new(self.next_id);
        self.next_id += 1;

        self.pages.insert(id, page);

        if self.active.is_none() {
            self.active = Some(id);
        }

        id
    }

    fn set_active(&mut self, id: Id) {
        if self.pages.get(&id).is_some() {
            self.active = Some(id);
            return;
        }

        panic!("No widget with {:?} id", id);
    }

    fn change_size(&mut self, size: Cursor) {
        self.buffer.update_size(size);
    }
}
