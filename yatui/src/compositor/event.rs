use std::ptr::NonNull;

use crate::state::controller::{CallBack, Data};

pub enum Event {
    Controller(ControllerEvent),
}

pub enum ControllerEvent {
    Add(ControllerAdd),
    Set(ControllerAdd),

    // Inc ref counter
    Subscribe(usize),
    // Dec ref counter
    Unsubscribe(usize),
}

pub struct ControllerAdd {
    pub id: usize,
    pub data: Data,
    pub destructor: CallBack,
}

// SAFETY: `ControllerAdd` created by `Send` object and we do not copy result in multiple threads
// (using it only in ui thread)
unsafe impl Send for ControllerAdd {}

impl ControllerAdd {
    pub fn new<T>(value: T, id: usize) -> Self
    where
        T: Send,
    {
        let data = NonNull::new(Box::into_raw(Box::new(value)) as *mut u8).unwrap();

        let destructor = Box::new(|v: NonNull<u8>| unsafe {
            Box::from_raw(v.cast::<T>().as_ptr());
        });

        Self { id, data, destructor }
    }
}
