pub(crate) mod controller;
mod create;

pub use self::{controller::pointer::Pointer, create::mut_state};

pub(crate) use controller::Controller;

#[derive(Debug)]
pub enum State<T> {
    Value(T),
    Pointer(Pointer<T>),
}

impl<T> State<T> {
    pub fn set(&mut self, v: T)
    where
        T: Send,
    {
        match self {
            State::Value(_) => *self = State::Value(v),
            State::Pointer(pointer) => pointer.set(v),
        }
    }
}

impl<T> From<Pointer<T>> for State<T> {
    fn from(v: Pointer<T>) -> Self {
        Self::Pointer(v)
    }
}

impl<T> From<T> for State<T> {
    fn from(v: T) -> Self {
        Self::Value(v)
    }
}
