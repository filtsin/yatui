pub(crate) mod controller;
mod create;

use std::rc::Rc;

pub use self::{
    controller::pointer::Pointer,
    create::{mut_state, mut_state_with},
};

pub(crate) use controller::Controller;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State<T> {
    Value(Rc<T>),
    Pointer(Pointer<T>),
}

impl<T> From<Pointer<T>> for State<T> {
    fn from(v: Pointer<T>) -> Self {
        Self::Pointer(v)
    }
}

impl<T> From<T> for State<T> {
    fn from(v: T) -> Self {
        Self::Value(Rc::new(v))
    }
}

impl From<&str> for State<String> {
    fn from(v: &str) -> Self {
        Self::Value(Rc::new(v.to_owned()))
    }
}
