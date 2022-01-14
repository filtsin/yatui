pub mod context;
pub(crate) mod event;

use self::{context::Context, event::Event};
use crate::{
    backend::Backend,
    component::Component,
    state::Controller,
    terminal::{
        buffer::{Buffer, MappedBuffer},
        cursor::Cursor,
        region::Region,
    },
};

pub(crate) struct Compositor<B> {
    backend: B,
    buffer: Buffer,

    root: Option<Component>,
    controller: Controller,
}

impl<B> Compositor<B> {
    pub(crate) fn new(backend: B) -> Self {
        Self {
            backend,
            buffer: Buffer::new(Cursor::new(20, 20)),
            root: None,
            controller: Controller::new(),
        }
    }
}

impl<B> Compositor<B> {
    pub(crate) fn change_root(&mut self, root: Component) {
        self.root = Some(root);
    }

    pub(crate) fn context(&self) -> Context<'_> {
        Context::new(&self.controller)
    }
}

impl<B> Compositor<B>
where
    B: Backend,
{
    pub(crate) fn draw(&mut self) {
        if let Some(component) = &mut self.root {
            let size = self.buffer.get_size();
            let (w, h) = (size.row(), size.column());
            let mapped_region = Region::new(Cursor::default(), Cursor::new(w, h));
            let mapped_buffer = MappedBuffer::new(&mut self.buffer, mapped_region);

            let context = Context::new(&self.controller);

            match component {
                Component::Canvas(c) => c.draw(mapped_buffer, context),
                Component::Layout(l) => {
                    l.layout(mapped_region, context);
                }
            }

            self.backend.hide_cursor();
            self.backend.clear_screen();

            self.backend.move_cursor(Cursor::new(0, 0));

            self.backend.draw(&self.buffer);
            self.backend.flush();
        }
    }
    pub(crate) fn process_event(&mut self, event: Event) {
        match event {
            Event::Controller(event) => match event {
                event::ControllerEvent::Add(v) => unsafe {
                    self.controller.insert(v.id, v.data, v.destructor);
                },
                event::ControllerEvent::Set(v) => unsafe {
                    self.controller.remove(v.id);
                    self.controller.insert(v.id, v.data, v.destructor);
                },
                event::ControllerEvent::Subscribe(_) => todo!(),
                event::ControllerEvent::Unsubscribe(_) => todo!(),
            },
        };
    }
}
