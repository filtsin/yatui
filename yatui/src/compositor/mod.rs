use crate::backend::Backend;

#[derive(Debug)]
pub(crate) struct Compositor<B> {
    backend: B,
}

impl<B> Compositor<B> {
    pub(crate) fn new(backend: B) -> Self {
        Self { backend }
    }
    pub(crate) fn process_event() {
        todo!()
    }
}
