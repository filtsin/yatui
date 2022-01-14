//! Application structure

use crate::{
    backend::Backend,
    component::Component,
    compositor::{
        context::Context,
        event::{ControllerEvent, Event},
        Compositor,
    },
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::{sync::RwLock, thread::sleep, time::Duration};

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

    pub fn mount(&mut self, root: Component) {
        self.compositor.change_root(root);
    }

    pub fn context(&self) -> Context<'_> {
        self.compositor.context()
    }
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn run(&mut self) {
        self.main_loop();
    }

    pub fn process_event(&mut self) {
        while let Ok(event) = self.queue.try_recv() {
            self.compositor.process_event(event);
        }
        self.compositor.draw();
    }

    fn main_loop(&mut self) {
        loop {
            self.process_event();
            sleep(Duration::from_millis(10));
        }
    }
}

impl Handle {
    const fn new() -> Self {
        Self { sender: None }
    }

    pub(crate) fn state_event(event: ControllerEvent) {
        Handle::send(Event::Controller(event));
    }

    fn instance() -> Self {
        Handle::mut_instance().read().unwrap().clone()
    }

    fn send(event: Event) {
        Handle::instance().sender.unwrap().send(event).unwrap()
    }

    fn mut_instance() -> &'static RwLock<Self> {
        static INSTANCE: OnceCell<RwLock<Handle>> = OnceCell::new();

        INSTANCE.get_or_init(|| RwLock::new(Self::new()))
    }
}
