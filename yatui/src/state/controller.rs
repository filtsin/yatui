use std::{collections::HashMap, marker::PhantomData, ptr::NonNull};

pub type Data = NonNull<u8>;
pub type CallBack = Box<dyn FnOnce(Data) + Send>;

#[derive(Default)]
pub struct Controller {
    data: HashMap<usize, ControllerContent>,
}

struct ControllerContent {
    data: Data,
    destructor: CallBack,
}

pub struct ControllerRef<'a> {
    pub data: NonNull<u8>,
    marker: PhantomData<&'a ()>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Safety
    /// 1. `data` must be a valid pointer for both reads and writes
    /// 2. `data` must be properly aligned
    /// 3. `data` must outlive `self` if no `remove` called
    /// # Panics
    /// Panics if `id` already exists in `Controller`
    pub unsafe fn insert(&mut self, id: usize, data: Data, destructor: CallBack) {
        if self.data.insert(id, ControllerContent::new(data, destructor)).is_some() {
            panic!("Controller panic: key {} already exists", id);
        }
    }

    /// # Panics
    /// Panics if `id` is not exists in `Controller`
    pub fn remove(&mut self, id: usize) {
        self.data
            .remove(&id)
            .expect(format!("Controller panic: key {} is not exists", id).as_str())
            .drop();
    }

    pub fn get(&self, id: usize) -> ControllerRef<'_> {
        ControllerRef { data: self.data.get(&id).unwrap().data, marker: PhantomData }
    }
}

impl ControllerContent {
    fn new(data: Data, destructor: CallBack) -> Self {
        Self { data, destructor }
    }

    fn drop(self) {
        (self.destructor)(self.data);
    }
}
