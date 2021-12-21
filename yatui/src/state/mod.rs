pub mod controller;

use std::marker::PhantomData;

use self::controller::reserve_id;
pub use controller::Controller;

#[derive(Debug)]
pub enum State<T> {
    Value(T),
    Ref(ControllerPointer<T>),
}

#[derive(Debug)]
pub struct ControllerPointer<T> {
    id: usize,
    marker: PhantomData<T>,
}

pub fn mut_state<T>(value: T) -> State<T> {
    let my_id = reserve_id();
    State::Ref(ControllerPointer::new(my_id))
}

impl<T> ControllerPointer<T> {
    pub fn new(id: usize) -> Self {
        Self { id, marker: PhantomData }
    }
}
