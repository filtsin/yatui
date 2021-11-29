//! Application structure
pub mod page;

use self::page::{Id, Page};
use crate::{
    backend::Backend,
    compositor::{event::Event, Compositor},
};
use once_cell::sync::OnceCell;
use std::sync::RwLock;
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot::channel,
    },
    task::LocalSet,
    time::{sleep, Duration},
};

pub struct App<B> {
    compositor: Compositor<B>,
    queue: UnboundedReceiver<Event>,
}

#[derive(Debug, Clone)]
pub struct Handle {
    sender: Option<UnboundedSender<Event>>,
}

impl<B> App<B> {
    pub fn new(backend: B) -> Self {
        let mut queue = Handle::mut_instance().write().unwrap();

        let (tx, rx) = unbounded_channel();

        queue.sender = Some(tx);

        App { compositor: Compositor::new(backend), queue: rx }
    }
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn run(mut self, rt: &Runtime) {
        let local = LocalSet::new();
        local.block_on(rt, self.main_loop());
    }
    async fn main_loop(&mut self) {
        self.compositor.draw();
        sleep(Duration::from_millis(100)).await;
    }
}

impl Handle {
    /// Returns `Id` of new page. If it is the first call of function, then the `set_active` will be
    /// call on the result.
    /// # Panics
    /// Panics if `App` is not created
    pub async fn add_page(page: Page) -> Id {
        let (tx, rx) = channel();

        Handle::send(Event::AddPage(page, tx));

        rx.await.unwrap()
    }

    /// It is ok to call `set_active` in current active page and continue work
    /// after that on this page that will be inactive in compositor
    /// # Panics
    /// Panics if `App` is not created or id is not registered by `add_page` function
    pub async fn set_active(id: Id) {
        Handle::send(Event::SetActive(id));
    }
}

impl Handle {
    const fn new() -> Self {
        Self { sender: None }
    }

    fn instance() -> Self {
        Handle::mut_instance().read().unwrap().clone()
    }

    fn send(event: Event) {
        // Other side of channel is always open while programm is alive
        Self::instance().sender.unwrap().send(event).unwrap();
    }

    fn mut_instance() -> &'static RwLock<Self> {
        static INSTANCE: OnceCell<RwLock<Handle>> = OnceCell::new();

        INSTANCE.get_or_init(|| RwLock::new(Self::new()))
    }
}
