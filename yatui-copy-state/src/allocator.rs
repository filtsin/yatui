use std::{cell::RefCell, rc::Rc};

use crate::memloc::MemLoc;

/// A pseudo arena allocator for `MemoryLocation` objects.
/// This allocator never returns memory to the system allocator, instead recycling memory
/// locations internally. It does allocate only when the pool is empty.
#[derive(Clone, Default)]
pub struct Allocator {
    arena: Rc<RefCell<Vec<&'static MemLoc>>>,
}

impl Allocator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Allocator {
    pub(crate) fn alloc<T: 'static>(&self, value: T) -> &'static MemLoc {
        let slot = self.arena.borrow_mut().pop();
        if let Some(memory) = slot {
            memory.alloc_inplace(value);
            memory
        } else {
            Box::leak(Box::new(MemLoc::default()))
        }
    }

    pub(crate) fn free(&self, mem: &'static MemLoc) {
        mem.free();
        self.arena.borrow_mut().push(mem);
    }
}
