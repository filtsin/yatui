//! Application structure
mod compositor;

use self::compositor::Compositor;
use crate::backend::{Backend, Termion};
use crate::error::{Error, Result};
use crate::widget::Widget;

use once_cell::sync::OnceCell;
use std::sync::Mutex;

use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::{sleep, Duration};

pub struct App {
    backend: Mutex<Option<Box<dyn Backend>>>,
    compositor: Mutex<Compositor>,
}

pub struct AppInstance {
    inner: &'static App,
}

impl App {
    fn new() -> Self {
        Self {
            backend: Mutex::new(None),
            compositor: Mutex::new(Compositor::new()),
        }
    }
    /// Get a wrapper for reference to global application instance
    pub fn instance() -> AppInstance {
        static INSTANCE: OnceCell<App> = OnceCell::new();
        let app = INSTANCE.get_or_init(|| App::new());
        AppInstance { inner: app }
    }
    fn run(&'static self, rt: &Runtime) {
        let local = LocalSet::new();
        local.block_on(rt, async {
            loop {
                let compositor = self.compositor.lock().unwrap();
                let mut backend = self.backend.lock().unwrap();
                compositor.step(backend.as_mut().unwrap().as_mut());
                sleep(Duration::from_millis(1)).await;
            }
        });
    }
}

impl AppInstance {
    /// Set backend manually
    pub fn set_backend(&self, backend: Box<dyn Backend>) {
        *self.inner.backend.lock().unwrap() = Some(backend);
    }
    /// Call set_backend with default value: Termion with stdout bind
    pub fn init(&self) -> Result<()> {
        let terminal = Termion::new(std::io::stdout())?;
        self.set_backend(Box::new(terminal));
        Ok(())
    }
    /// Run an event loop
    pub fn run(&self, rt: &Runtime) -> Result<()> {
        let instance = self.inner;
        if instance.backend.lock().unwrap().is_none() {
            return Err(Error::AppNotInit);
        }
        instance.run(rt);
        Ok(())
    }
    /// Add widget
    /// Temporary function
    pub fn add_widget(&self, widget: Box<dyn Widget>) {
        self.inner.compositor.lock().unwrap().add_widget(widget);
    }
}
