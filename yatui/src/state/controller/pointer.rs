use std::marker::PhantomData;

use crate::{
    app::Handle,
    compositor::event::{ControllerAdd, ControllerEvent},
};

#[derive(Debug)]
pub struct Pointer<T> {
    id: usize,
    marker: PhantomData<T>,
}

impl<T> Pointer<T> {
    pub(crate) fn new(value: T, id: usize) -> Self
    where
        T: Send,
    {
        let event = ControllerAdd::new(value, id);
        Handle::state_event(ControllerEvent::Add(event));

        Self { id, marker: PhantomData }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set(&mut self, v: T)
    where
        T: Send,
    {
        let event = ControllerAdd::new(v, self.id);
        Handle::state_event(ControllerEvent::Set(event));
    }
}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        Handle::state_event(ControllerEvent::Subscribe(self.id()));
        Self { id: self.id, marker: self.marker }
    }
}

impl<T> Drop for Pointer<T> {
    fn drop(&mut self) {
        Handle::state_event(ControllerEvent::Unsubscribe(self.id()));
    }
}
