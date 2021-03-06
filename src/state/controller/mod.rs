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

#[derive(Copy, Clone)]
pub struct ControllerRefMut<'a> {
    data: Data,
    marker: PhantomData<&'a mut ()>,
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
    pub unsafe fn insert(&mut self, id: Id, data: Data, destructor: CallBack) {
        if self.data.insert(id, ControllerContent::new(data, destructor)).is_some() {
            panic!("Controller panic: key {} already exists", id);
        }
    }

    /// Replace exists value in `Controller`
    ///
    /// # Safety
    /// 1. `data` must be a valid pointer for both reads and writes
    /// 2. `data` must be properly aligned
    /// 3. `data` must outlive `self` if no `remove` called
    /// # Panics
    /// Panics if `id` already exists in `Controller`
    pub unsafe fn set(&mut self, id: Id, data: Data, destructor: CallBack) {
        let ref_count = self.ref_count(id);

        self.remove(id);
        self.insert(id, data, destructor);
        self.data.get_mut(&id).unwrap().count = ref_count;
    }

    /// Remove value with `id` in `Controller` ignoring its ref counter
    ///
    /// # Panics
    /// Panics if `id` is not exists in `Controller` or `destructor` panics
    pub fn remove(&mut self, id: Id) {
        self.data
            .remove(&id)
            .unwrap_or_else(|| panic!("Controller panic: key {} is not exists", id));
    }

    /// Increment ref counter
    ///
    /// # Panics
    /// Panics if `id` is not exists in `Controller`
    pub fn subscribe(&mut self, id: Id) {
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
    pub fn get(&self, id: Id) -> ControllerRef<'_> {
        ControllerRef { data: self.content(id).data, marker: PhantomData }
    }

    pub fn get_mut(&mut self, id: Id) -> ControllerRefMut<'_> {
        ControllerRefMut { data: self.content(id).data, marker: PhantomData }
    }

    pub fn get_raw(&mut self, id: Id) -> Data {
        self.content(id).data
    }

    pub fn ref_count(&self, id: Id) -> usize {
        self.content(id).count
    }

    fn content(&self, id: Id) -> &ControllerContent {
        self.data.get(&id).unwrap()
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

impl<'a> ControllerRefMut<'a> {
    pub fn map<T>(self) -> &'a mut T {
        unsafe { &mut *self.data.cast::<T>().as_ptr() }
    }
}
