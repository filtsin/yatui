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
            State::Pointer(pointer) => {
                let reference = self.controller.get(pointer.id());
                unsafe { &*(reference.data.as_ptr() as *const T) }
            }
        }
    }
}
