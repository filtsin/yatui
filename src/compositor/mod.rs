pub mod context;
pub(crate) mod event;
pub(crate) mod watcher;

use self::{
    context::Context,
    event::{
        controller::{self, Action, Insert, Update},
        Event,
    },
    watcher::Watcher,
};
use crate::{
    backend::Backend,
    component::Component,
    state::{controller::Id, Controller},
    terminal::{Cursor, Printer, Region, Size},
};

pub(crate) struct Compositor<B> {
    backend: B,

    root: Option<Component>,
    controller: Controller,
    watcher: Watcher,
}

impl<B> Compositor<B> {
    pub(crate) fn new(backend: B) -> Self {
        Self { backend, root: None, controller: Controller::new(), watcher: Watcher::default() }
    }
}

impl<B> Compositor<B> {
    pub(crate) fn change_root(&mut self, root: Component) {
        self.root = Some(root);
    }
}

impl<B> Compositor<B>
where
    B: Backend,
{
    pub(crate) fn draw(&mut self) {
        // TODO: Do it only in 1st draw attempt
        self.backend.clear_screen();

        if let Some(component) = &mut self.root {
            let current_size = self.backend.get_size().unwrap();

            let context = Context::new(&self.controller, &self.watcher, current_size);

            let mut printer = Printer::new(&mut self.backend);

            component.size_hint(context);
            component.layout(Region::from(current_size), context);
            component.draw(&mut printer, context);

            // self.backend.hide_cursor();
            self.backend.flush();
        }
    }

    pub(crate) fn context(&self) -> Context<'_> {
        Context::new(&self.controller, &self.watcher, self.backend.get_size().unwrap())
    }

    pub(crate) fn process_event(&mut self, event: Event) {
        match event {
            Event::Controller(controller::Event { id, action }) => {
                self.controller_action(id, action)
            }
        };
    }

    pub(crate) fn controller_action(&mut self, id: Id, action: Action) {
        match action {
            Action::Insert(v) => self.controller_insert(id, v),
            Action::Set(v) => self.controller_set(id, v),
            Action::Update(v) => self.controller_update(id, v),
            Action::Subscribe => self.controller.subscribe(id),
            Action::Unsubscribe => self.controller.unsubscribe(id),
        }
    }

    pub(crate) fn controller_insert(&mut self, id: Id, insert: Insert) {
        self.watcher.add(id);

        match insert {
            Insert::Obj(obj) => unsafe { self.controller.insert(id, obj.data, obj.destructor) },
            Insert::Func(func) => unsafe {
                self.controller.insert(id, (func.callback)(), func.destructor)
            },
        }
    }

    pub(crate) fn controller_set(&mut self, id: Id, insert: Insert) {
        self.watcher.add(id);

        match insert {
            Insert::Obj(obj) => unsafe { self.controller.set(id, obj.data, obj.destructor) },
            Insert::Func(func) => unsafe {
                self.controller.set(id, (func.callback)(), func.destructor)
            },
        }
    }

    pub(crate) fn controller_update(&mut self, id: Id, update: Update) {
        self.watcher.add(id);

        let old = self.controller.get_raw(id);

        (update.callback)(old)
    }
}
