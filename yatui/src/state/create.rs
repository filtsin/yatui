use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use super::Pointer;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub fn reserve_id() -> usize {
    NEXT_ID.fetch_add(1, Relaxed)
}

/// Create state from `value`
pub fn mut_state<T>(value: T) -> Pointer<T>
where
    T: Send,
{
    let my_id = reserve_id();
    Pointer::new(value, my_id)
}
