#[cfg(test)]
mod tests;

pub mod pointer;

use std::{collections::HashMap, marker::PhantomData, ptr::NonNull};

pub type Id = usize;
pub type Data = NonNull<u8>;
pub type CallBack = Box<dyn FnOnce(Data) + Send>;

#[derive(Default)]
pub struct Controller {
    data: HashMap<Id, ControllerContent>,
}

struct ControllerContent {
    data: Data,
    destructor: Option<CallBack>,
    count: usize,
}

#[derive(Copy, Clone)]
pub struct ControllerRef<'a> {
    data: Data,
    marker: PhantomData<&'a ()>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add new value in `Controller`. Ref counter will be equal 1.
    ///
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

    /// Remove value with `id` in `Controller` ignoring its ref counter
    ///
    /// # Panics
    /// Panics if `id` is not exists in `Controller` or `destructor` panics
    pub fn remove(&mut self, id: usize) {
        self.data
            .remove(&id)
            .unwrap_or_else(|| panic!("Controller panic: key {} is not exists", id));
    }

    /// Increment ref counter
    ///
    /// # Panics
    /// Panics if `id` is not exists in `Controller`
    pub fn subscribe(&mut self, id: usize) {
        self.data.get_mut(&id).unwrap().inc_count();
    }

    /// Decrement ref counter. If ref counter becomes 0, then the value is removed from the
    /// controller.
    ///
    /// # Panics
    /// Panics if `id` is not exists in `Controller` or `destructor` panics
    pub fn unsubscribe(&mut self, id: usize) {
        if self.data.get_mut(&id).unwrap().dec_count() == 0 {
            self.remove(id);
        }
    }

    /// # Panics
    /// Panics if `id` is not exists in `Controller`
    pub fn get(&self, id: usize) -> ControllerRef<'_> {
        ControllerRef { data: self.data.get(&id).unwrap().data, marker: PhantomData }
    }
}

impl ControllerContent {
    fn new(data: Data, destructor: CallBack) -> Self {
        Self { data, destructor: Some(destructor), count: 1 }
    }

    fn inc_count(&mut self) -> usize {
        self.count += 1;
        self.count
    }

    fn dec_count(&mut self) -> usize {
        self.count -= 1;
        self.count
    }
}

impl Drop for ControllerContent {
    fn drop(&mut self) {
        self.destructor.take().unwrap()(self.data);
    }
}

impl<'a> ControllerRef<'a> {
    pub fn map<T>(self) -> &'a T {
        unsafe { &*self.data.cast::<T>().as_ptr() }
    }
}
