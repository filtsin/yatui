use std::collections::HashMap;

use crate::{
    app::{
        event::Event,
        page::{Id, Page},
    },
    backend::Backend,
    terminal::cursor::Index,
};

#[derive(Debug)]
pub(crate) struct Compositor<B> {
    backend: B,
    pages: HashMap<Id, Page>,
    active: Option<Id>,
    current_index: u32,
    size: (Index, Index),
}

impl<B> Compositor<B> {
    pub(crate) fn new(backend: B) -> Self {
        Self { backend, pages: HashMap::new(), active: None, current_index: 0, size: (0, 0) }
    }
}

impl<B> Compositor<B>
where
    B: Backend,
{
    pub(crate) async fn draw(&mut self) {
        todo!()
    }
    pub(crate) async fn process_event(&mut self, event: Event) {
        match event {
            Event::SetActive(id) => self.set_active(id),
            Event::AddPage(page, channel) => channel.send(self.add_page(page)).unwrap(),
            Event::ChangeSize(size) => self.change_size(size),
            Event::Draw => self.draw().await,
        };
    }

    fn add_page(&mut self, page: Page) -> Id {
        let id = Id::new(self.current_index);
        self.current_index += 1;

        self.pages.insert(id, page).unwrap();

        id
    }

    fn set_active(&mut self, id: Id) {
        if self.pages.get(&id).is_some() {
            self.active = Some(id);
            return;
        }

        panic!("No widget with {:?} id", id);
    }

    fn change_size(&mut self, size: (Index, Index)) {
        self.size = size;
    }
}
