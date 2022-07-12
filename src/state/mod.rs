pub(crate) mod controller;
mod create;

use std::{cell::RefCell, rc::Rc};

use crate::component::{layout::Children, Component};

use self::controller::Id;
pub use self::{
    controller::pointer::Pointer,
    create::{mut_state, mut_state_with},
};

pub(crate) use controller::Controller;

#[derive(Debug, Eq, PartialEq)]
pub enum State<T> {
    Value(InnerValue<T>),
    Pointer(Pointer<T>),
}

pub fn try_get_id_from_state<T>(state: State<T>) -> Option<Id> {
    match state {
        State::Value(_) => None,
        State::Pointer(p) => Some(p.id()),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InnerValue<T> {
    pub(crate) pointer: Rc<T>,
}

impl<T> From<Pointer<T>> for State<T> {
    fn from(v: Pointer<T>) -> Self {
        Self::Pointer(v)
    }
}

impl<T> From<T> for State<T> {
    fn from(v: T) -> Self {
        Self::Value(InnerValue { pointer: Rc::new(v) })
    }
}

impl From<&str> for State<String> {
    fn from(v: &str) -> Self {
        Self::Value(InnerValue { pointer: Rc::new(v.to_owned()) })
    }
}

impl<C> From<C> for State<Children>
where
    C: IntoIterator<Item = Component>,
{
    fn from(v: C) -> Self {
        Self::Value(InnerValue { pointer: Rc::new(Children::new(v)) })
    }
}

impl<T> Clone for InnerValue<T> {
    fn clone(&self) -> Self {
        Self { pointer: self.pointer.clone() }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Value(v) => Self::Value(v.clone()),
            Self::Pointer(p) => Self::Pointer(p.clone()),
        }
    }
}
