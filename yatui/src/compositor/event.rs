use std::ptr::NonNull;

use crate::state::controller::{CallBack, Data};

pub(crate) enum Event {
    Controller(ControllerEvent),
}

pub(crate) enum ControllerEvent {
    Add(ControllerAdd),
    Set(ControllerAdd),

    Subscribe(usize),
    Unsubscribe(usize),
}

pub(crate) struct ControllerAdd {
    id: usize,
    data: Data,
    destructor: CallBack,
}

unsafe impl Send for ControllerAdd {}

impl ControllerAdd {
    pub fn new<T, F>(value: T, id: usize) -> Self
    where
        T: Send,
    {
        let data = NonNull::new(Box::into_raw(Box::new(value)) as *mut u8).unwrap();

        let destructor = Box::new(|v: NonNull<u8>| unsafe {
            Box::from_raw(v.as_ptr() as *mut T);
        });

        Self { id, data, destructor }
    }
}
