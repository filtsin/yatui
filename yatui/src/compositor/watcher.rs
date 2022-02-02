use std::collections::HashSet;

use crate::state::controller::Id;

#[derive(Default)]
pub struct Watcher {
    changes: HashSet<Id>,
}

impl Watcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: Id) {
        self.changes.insert(id);
    }

    pub fn remove_all(&mut self) {
        self.changes.clear();
    }

    pub fn contains(&self, id: Id) -> bool {
        self.changes.contains(&id)
    }

    pub fn size(&self) -> usize {
        self.changes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}
