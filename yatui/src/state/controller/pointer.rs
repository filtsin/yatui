use std::marker::PhantomData;

use crate::{
    app::Handle,
    compositor::event::controller::{Action, Event, Func, Insert, Obj, Update},
};

use super::Id;

#[derive(Debug)]
pub struct Pointer<T> {
    id: Id,
    marker: PhantomData<T>,
}

impl<T> Pointer<T> {
    pub(crate) fn new(value: T, id: Id) -> Self
    where
        T: Send,
    {
        Self::new_inner(Obj::new(value), id)
    }

    pub(crate) fn new_with<F>(f: F, id: Id) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
    {
        Self::new_inner(Func::new(f), id)
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn set(&self, v: T)
    where
        T: Send,
    {
        self.set_inner(Obj::new(v));
    }

    pub fn set_with<F>(&self, f: F)
    where
        F: FnOnce() -> T + Send + 'static,
    {
        self.set_inner(Func::new(f));
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T) + Send + 'static,
    {
        let action = Action::update(Update::new(f));
        self.send_event(action);
    }

    fn new_inner<U>(v: U, id: Id) -> Self
    where
        U: Into<Insert>,
    {
        let action = Action::insert(v.into());
        Handle::state_event(Event::new(id, action));

        Self { id, marker: PhantomData }
    }

    fn set_inner<U>(&self, v: U)
    where
        U: Into<Insert>,
    {
        let action = Action::set(v.into());
        self.send_event(action);
    }

    fn send_event(&self, action: Action) {
        Handle::state_event(Event::new(self.id(), action));
    }
}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        self.send_event(Action::subscribe());
        Self { id: self.id, marker: self.marker }
    }
}

impl<T> Drop for Pointer<T> {
    fn drop(&mut self) {
        self.send_event(Action::unsubscribe());
    }
}

impl<T> PartialEq for Pointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Pointer<T> {}
