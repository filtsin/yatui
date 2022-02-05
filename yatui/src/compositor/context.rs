use std::rc::Rc;

use crate::{
    compositor::Watcher,
    state::{controller::Id, Controller, State},
    terminal::cursor::{Cursor, Index},
};

#[derive(Clone, Copy)]
pub struct Context<'a> {
    controller: &'a Controller,
    watcher: &'a Watcher,
    size: Cursor,
}

impl<'a> Context<'a> {
    pub(crate) fn new(controller: &'a Controller, watcher: &'a Watcher, size: Cursor) -> Self {
        Self { controller, watcher, size }
    }

    pub fn size(self) -> Cursor {
        self.size
    }

    pub fn get<'b: 'a, T>(self, state: &'b State<T>) -> &'a T {
        match state {
            State::Value(v) => v,
            State::Pointer(pointer) => self.controller.get(pointer.id()).map::<T>(),
        }
    }

    pub fn ref_count<T>(self, state: &'_ State<T>) -> usize {
        match state {
            State::Value(pointer) => Rc::strong_count(pointer),
            State::Pointer(pointer) => self.controller.ref_count(pointer.id()),
        }
    }

    pub fn is_changed<T>(self, state: &'_ State<T>) -> bool {
        match state {
            State::Value(_) => false,
            State::Pointer(pointer) => self.is_changed_id(pointer.id()),
        }
    }

    pub fn is_changed_id(self, id: Id) -> bool {
        self.watcher.contains(id)
    }
}
