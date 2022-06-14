use std::ptr::NonNull;

use crate::state::controller::{CallBack, Data, Id};

pub struct Event {
    pub id: Id,
    pub action: Action,
}

pub enum Action {
    Insert(Insert),
    Set(Insert),
    Update(Update),

    // Inc ref counter
    Subscribe,
    // Dec ref counter
    Unsubscribe,
}

pub enum Insert {
    Obj(Obj),
    Func(Func),
}

pub struct Obj {
    pub data: Data,
    pub destructor: CallBack,
}

pub struct Func {
    pub callback: Box<dyn FnOnce() -> Data>,
    pub destructor: CallBack,
}

pub struct Update {
    pub callback: CallBack,
}

impl Event {
    pub fn new(id: Id, action: Action) -> Self {
        Self { id, action }
    }
}

impl Action {
    pub fn insert<T>(value: T) -> Self
    where
        T: Into<Insert>,
    {
        Action::Insert(value.into())
    }

    pub fn set<T>(value: T) -> Self
    where
        T: Into<Insert>,
    {
        Action::Set(value.into())
    }

    pub fn update(value: Update) -> Self {
        Action::Update(value)
    }

    pub fn subscribe() -> Self {
        Action::Subscribe
    }

    pub fn unsubscribe() -> Self {
        Action::Unsubscribe
    }
}

// SAFETY: `Obj` created by `Send` object and we do not copy result in multiple threads
// (using it only in ui thread)
unsafe impl Send for Obj {}

impl Obj {
    pub fn new<T>(value: T) -> Self
    where
        T: Send,
    {
        let data = NonNull::new(Box::into_raw(Box::new(value)) as *mut u8).unwrap();

        let destructor = default_callback::<T>();

        Self { data, destructor }
    }
}

// SAFETY: `ControllerFunc` created by `Send` object and we do not copy result in multiple threads
// (using it only in ui thread)
unsafe impl Send for Func {}

impl Func {
    pub fn new<F, T>(f: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let f = Box::new(|| NonNull::new(Box::into_raw(Box::new(f())) as *mut u8).unwrap());

        let destructor = default_callback::<T>();

        Self { callback: f, destructor }
    }
}

impl Update {
    pub fn new<T, F>(callback: F) -> Self
    where
        F: FnOnce(&mut T) + Send + 'static,
    {
        let callback = Box::new(|v: Data| unsafe {
            callback(v.cast::<T>().as_mut());
        });

        Self { callback }
    }
}

fn default_callback<T>() -> CallBack {
    Box::new(|v: Data| unsafe {
        Box::from_raw(v.cast::<T>().as_ptr());
    })
}

impl From<Obj> for Insert {
    fn from(v: Obj) -> Self {
        Self::Obj(v)
    }
}

impl From<Func> for Insert {
    fn from(v: Func) -> Self {
        Self::Func(v)
    }
}
