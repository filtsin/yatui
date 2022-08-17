use std::{
    cell::{Ref, RefMut},
    ops::Deref,
    rc::Rc,
};

use crate::{
    compositor::Watcher,
    state::{controller::Id, Controller, State},
    terminal::Size,
};

#[derive(Clone, Copy)]
pub struct Context<'a> {
    controller: &'a Controller,
    watcher: &'a Watcher,
    size: Size,
}

impl<'a> Context<'a> {
    pub(crate) fn new(controller: &'a Controller, watcher: &'a Watcher, size: Size) -> Self {
        Self { controller, watcher, size }
    }

    pub fn size(self) -> Size {
        self.size
    }

    pub fn get<'b: 'a, T>(self, state: &'b State<T>) -> &'a T {
        match state {
            State::Value(v) => &v.pointer,
            State::Pointer(pointer) => self.controller.get(pointer.id()).map::<T>(),
        }
    }

    pub fn ref_count<T>(self, state: &State<T>) -> usize {
        match state {
            State::Value(value) => Rc::strong_count(&value.pointer),
            State::Pointer(pointer) => self.controller.ref_count(pointer.id()),
        }
    }

    pub fn is_changed<T>(self, state: &State<T>) -> bool {
        match state {
            State::Value(_) => false,
            State::Pointer(pointer) => self.is_changed_id(pointer.id()),
        }
    }

    pub fn is_changed_id(self, id: Id) -> bool {
        self.watcher.contains(id)
    }
}
