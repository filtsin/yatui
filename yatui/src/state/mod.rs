pub mod controller;

use std::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use crate::{
    app::Handle,
    compositor::event::{ControllerAdd, ControllerEvent},
};

pub use controller::Controller;

pub fn mut_state<T>(value: T) -> State<T>
where
    T: Send,
{
    let my_id = reserve_id();

    let event = ControllerAdd::new(value, my_id);
    Handle::state_event(ControllerEvent::Add(event));

    State::Pointer(ControllerPointer::new(my_id))
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub fn reserve_id() -> usize {
    NEXT_ID.fetch_add(1, Relaxed)
}

#[derive(Debug)]
pub enum State<T> {
    Value(T),
    Pointer(ControllerPointer<T>),
}

#[derive(Debug)]
pub struct ControllerPointer<T> {
    id: usize,
    marker: PhantomData<T>,
}

impl<T> ControllerPointer<T> {
    pub fn new(id: usize) -> Self {
        Self { id, marker: PhantomData }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<T> State<T> {
    pub fn set(&mut self, v: T)
    where
        T: Send,
    {
        match self {
            State::Value(_) => *self = State::Value(v),
            State::Pointer(pointer) => {
                let event = ControllerAdd::new(v, pointer.id);
                Handle::state_event(ControllerEvent::Set(event));
            }
        }
    }

    pub fn try_clone(&self) -> Option<State<T>> {
        match self {
            Self::Value(_) => None,
            Self::Pointer(_) => Some(self.clone()),
        }
    }
}

impl<T> Clone for ControllerPointer<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, marker: self.marker }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Value(_) => panic!("Can not clone state with value"),
            Self::Pointer(pointer) => Self::Pointer(pointer.clone()),
        }
    }
}
