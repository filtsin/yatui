use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    allocator::Allocator,
    memloc::{MemLoc, MemLocBorrowError, MemLocRef},
};

pub struct Controller {
    owned: Vec<&'static MemLoc>,
    allocator: Allocator,
}

pub struct CopyState<U> {
    mem_ref: MemLocRef,
    marker: PhantomData<U>,
}

pub enum CopyStateBorrowErr {
    StateDropped,
    AlreadyBorrowed,
}

impl<T: 'static> CopyState<T> {
    pub fn try_borrow(&self) -> Result<impl Deref<Target = T> + '_, CopyStateBorrowErr> {
        Ok(self.mem_ref.try_borrow()?)
    }

    pub fn try_borrow_mut(&self) -> Result<impl DerefMut<Target = T> + '_, CopyStateBorrowErr> {
        Ok(self.mem_ref.try_borrow_mut()?)
    }
}

impl Controller {
    pub fn new(allocator: Allocator) -> Self {
        Self { owned: vec![], allocator }
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> CopyState<T> {
        let mem = self.allocator.alloc(value);
        let mem_ref = mem.reference();
        self.owned.push(mem);

        CopyState { mem_ref, marker: PhantomData }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        for mem in &self.owned {
            self.allocator.free(mem)
        }
    }
}

impl From<MemLocBorrowError> for CopyStateBorrowErr {
    fn from(value: MemLocBorrowError) -> Self {
        match value {
            MemLocBorrowError::EpocheMismatch | MemLocBorrowError::EmptyLocation => {
                CopyStateBorrowErr::StateDropped
            }
            MemLocBorrowError::AlreadyBorrowed => CopyStateBorrowErr::AlreadyBorrowed,
            MemLocBorrowError::TypeMismatch => panic!("TODO: Explanation"),
        }
    }
}
