use crate::state::{Controller, State};

#[derive(Clone, Copy)]
pub struct Context<'a> {
    controller: &'a Controller,
}

impl<'a> Context<'a> {
    pub(crate) fn new(controller: &'a Controller) -> Self {
        Self { controller }
    }

    pub fn get<'b: 'a, T>(self, state: &'b State<T>) -> &'a T {
        match state {
            State::Value(v) => v,
            State::Pointer(pointer) => self.controller.get(pointer.id()).map::<T>(),
        }
    }

    pub fn ref_count<T>(self, state: &'_ State<T>) -> usize {
        match state {
            State::Value(_) => 1,
            State::Pointer(pointer) => self.controller.ref_count(pointer.id()),
        }
    }
}
