use std::collections::HashSet;

#[derive(Default)]
pub struct Watcher {
    changes: HashSet<usize>,
}

impl Watcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: usize) {
        self.changes.insert(id);
    }

    pub fn remove_all(&mut self) {
        self.changes.clear();
    }

    pub fn contains(&self, id: usize) -> bool {
        self.changes.contains(&id)
    }
}
