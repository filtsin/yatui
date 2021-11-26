//! Application structure
pub mod event;

use self::event::Event;
use crate::compositor::Compositor;
use once_cell::sync::OnceCell;
use std::sync::RwLock;
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot::channel,
    },
    task::LocalSet,
};

pub struct App<B> {
    compositor: Compositor<B>,
    queue: UnboundedReceiver<Event>,
}

#[derive(Debug, Clone)]
pub struct Queue {
    sender: Option<UnboundedSender<Event>>,
}

impl<B> App<B> {
    pub fn new(backend: B) -> Self {
        let mut queue = Queue::mut_instance().write().unwrap();

        let (tx, rx) = unbounded_channel();

        queue.sender = Some(tx);

        App { compositor: Compositor::new(backend), queue: rx }
    }
    pub fn run(self, rt: &Runtime) {
        let local = LocalSet::new();
        local.block_on(rt, async {});
    }
}

impl Queue {
    const fn new() -> Self {
        Self { sender: None }
    }

    fn instance() -> Self {
        Queue::mut_instance().read().unwrap().clone()
    }

    /// # Panics
    /// Panics if `App` is not created
    pub fn send(event: Event) {
        // Other side of channel is always open while programm is alive
        Self::instance().sender.unwrap().send(event).unwrap();
    }

    /// # Panics
    /// Panics if `App` is not created
    pub async fn async_send(event: Event) {
        let (tx, rx) = channel();

        Queue::send(Event::__AsyncEvent(Box::new(event), tx));

        // Sender should not be dropped by App
        rx.await.unwrap()
    }

    fn mut_instance() -> &'static RwLock<Queue> {
        static INSTANCE: OnceCell<RwLock<Queue>> = OnceCell::new();

        INSTANCE.get_or_init(|| RwLock::new(Queue::new()))
    }
}
