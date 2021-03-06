use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use super::Pointer;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub fn reserve_id() -> usize {
    NEXT_ID.fetch_add(1, Relaxed)
}

/// Create state from `value`
pub fn mut_state<U, T>(value: U) -> Pointer<T>
where
    U: Send,
    U: Into<T>,
    T: Send,
{
    let my_id = reserve_id();
    Pointer::new(value.into(), my_id)
}

pub fn mut_state_with<F, T, U>(f: F) -> Pointer<U>
where
    F: FnOnce() -> T + Send + 'static,
    T: Into<U>,
{
    let my_id = reserve_id();
    Pointer::new_with(|| f().into(), my_id)
}
