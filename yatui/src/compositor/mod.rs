pub mod context;
pub(crate) mod event;

use self::{
    context::Context,
    event::{
        controller::{self, Action, Insert, Update},
        Event,
    },
};
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
            Event::Controller(controller::Event { id, action }) => {
                self.controller_action(id, action)
            }
        };
    }

    pub(crate) fn controller_action(&mut self, id: usize, action: Action) {
        match action {
            Action::Insert(v) => self.controller_insert(id, v),
            Action::Set(v) => self.controller_set(id, v),
            Action::Update(v) => self.controller_update(id, v),
            Action::Subscribe(id) => self.controller.subscribe(id),
            Action::Unsubscribe(id) => self.controller.unsubscribe(id),
        }
    }

    pub(crate) fn controller_insert(&mut self, id: usize, insert: Insert) {
        match insert {
            Insert::Obj(obj) => unsafe { self.controller.insert(id, obj.data, obj.destructor) },
            Insert::Func(func) => unsafe {
                self.controller.insert(id, (func.callback)(), func.destructor)
            },
        }
    }

    pub(crate) fn controller_set(&mut self, id: usize, insert: Insert) {
        match insert {
            Insert::Obj(obj) => unsafe { self.controller.set(id, obj.data, obj.destructor) },
            Insert::Func(func) => unsafe {
                self.controller.set(id, (func.callback)(), func.destructor)
            },
        }
    }

    pub(crate) fn controller_update(&mut self, id: usize, update: Update) {
        let old = self.controller.get_raw(id);

        (update.callback)(old)
    }
}
