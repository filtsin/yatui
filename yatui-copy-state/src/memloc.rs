use std::{
    any::Any,
    cell::{Cell, RefCell},
    ops::{Deref, DerefMut},
};

/// Memory location allocated on the heap by `Alloc`. This memory location never deallocated, so
/// `epoch` counter used by allocator to reuse this memory location to the new value by
/// incrementing it.
///
/// Used for type-erased interior mutability, similar to `RefCell` but with dynamic type
/// checking.
///
/// Differences from `RefCell` are:
/// - Works with type-erased content (can store only 'static type)
/// - Requires dynamic type checking when borrowing
/// - Value can be missing
#[derive(Default)]
pub(crate) struct MemLoc {
    value: RefCell<Option<Box<dyn Any>>>,
    epoche: Cell<usize>,
}

/// It is a reference to `MemoryLocation` with the **freezed** epoch. If `epoch` of this reference does
/// not equal to `epoch` in `mem` then this reference is not valid anymore.
pub(crate) struct MemLocRef {
    mem: &'static MemLoc,
    epoche: usize,
}

#[derive(Debug)]
pub(crate) enum MemLocBorrowError {
    EpocheMismatch,
    AlreadyBorrowed,
    EmptyLocation,
    TypeMismatch,
}

impl MemLocRef {
    pub(crate) fn try_borrow<T: 'static>(
        &self,
    ) -> Result<impl Deref<Target = T> + '_, MemLocBorrowError> {
        if self.epoche != self.mem.epoche.get() {
            return Err(MemLocBorrowError::EpocheMismatch);
        }

        let inref = std::cell::Ref::filter_map(
            self.mem.value.try_borrow().map_err(|_| MemLocBorrowError::AlreadyBorrowed)?,
            |opt| opt.as_ref(),
        )
        .map_err(|_| MemLocBorrowError::EmptyLocation)?;

        std::cell::Ref::filter_map(inref, |any| any.downcast_ref::<T>())
            .map_err(|_| MemLocBorrowError::TypeMismatch)
    }

    pub(crate) fn try_borrow_mut<T: 'static>(
        &self,
    ) -> Result<impl DerefMut<Target = T> + '_, MemLocBorrowError> {
        if self.epoche != self.mem.epoche.get() {
            return Err(MemLocBorrowError::EpocheMismatch);
        }

        let inref_mut = std::cell::RefMut::filter_map(
            self.mem.value.try_borrow_mut().map_err(|_| MemLocBorrowError::AlreadyBorrowed)?,
            |opt| opt.as_mut(),
        )
        .map_err(|_| MemLocBorrowError::EmptyLocation)?;

        std::cell::RefMut::filter_map(inref_mut, |any| any.downcast_mut::<T>())
            .map_err(|_| MemLocBorrowError::TypeMismatch)
    }
}

impl MemLoc {
    pub(crate) fn reference(&'static self) -> MemLocRef {
        MemLocRef { mem: self, epoche: self.epoche.get() }
    }

    pub(crate) fn alloc_inplace<T: 'static>(&self, value: T) {
        let mut inref = self
            .value
            .try_borrow_mut()
            .expect("TODO: Better panic message with explanation what happened");
        *inref = Some(Box::new(value));
        self.epoche.update(|v| v + 1);
    }

    pub(crate) fn free(&self) {
        let mut inref = self
            .value
            .try_borrow_mut()
            .expect("TODO: Better panic message with explanation what happened");
        *inref = None;
    }
}
