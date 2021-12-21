//! Application structure

use crate::{
    backend::Backend,
    compositor::{event::Event, Compositor},
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::sync::RwLock;

pub struct App<B> {
    compositor: Compositor<B>,
    queue: Receiver<Event>,
}

#[derive(Debug, Clone)]
pub struct Handle {
    sender: Option<Sender<Event>>,
}

impl<B> App<B> {
    pub fn new(backend: B) -> Self {
        let mut queue = Handle::mut_instance().write().unwrap();

        let (tx, rx) = unbounded();

        queue.sender = Some(tx);

        App { compositor: Compositor::new(backend), queue: rx }
    }
}

impl<B> App<B>
where
    B: Backend,
{
    fn main_loop(&mut self) {
        loop {
            self.compositor.draw();
            // get events from queue
        }
    }
}

impl Handle {}

impl Handle {
    const fn new() -> Self {
        Self { sender: None }
    }

    fn instance() -> Self {
        Handle::mut_instance().read().unwrap().clone()
    }

    fn send(event: Event) {
        // Other side of channel is always open while programm is alive
    }

    fn mut_instance() -> &'static RwLock<Self> {
        static INSTANCE: OnceCell<RwLock<Handle>> = OnceCell::new();

        INSTANCE.get_or_init(|| RwLock::new(Self::new()))
    }
}
