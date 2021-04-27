//! Application structure
mod logic;

use crate::backend::{Backend, Termion};
use crate::error::{Result, Error};
use crate::widget::Widget;
use logic::AppLogic;

use std::sync::Mutex;
use once_cell::sync::OnceCell;
use std::time::Duration;

pub struct App {
    backend: Mutex<Option<Box<dyn Backend>>>,
    logic: Mutex<AppLogic>
}

pub struct AppInstance {
    inner: &'static App
}

impl App {
    fn new() -> Self {
        Self {
            backend: Mutex::new(None),
            logic: Mutex::new(AppLogic::new())
        }
    }
    /// Get an wrapper for reference to global application instance
    pub fn instance() -> AppInstance {
        static INSTANCE: OnceCell<App> = OnceCell::new();
        let app = INSTANCE.get_or_init(|| {
           App::new()
        });
        AppInstance { inner: app }
    }
    fn run(&'static self) {
        loop {
            let logic = self.logic.lock().unwrap();
            let mut backend = self.backend.lock().unwrap();
            logic.step(backend.as_mut().unwrap().as_mut());
            std::thread::sleep(Duration::from_secs(2));
        }
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
    pub fn run(&self) -> Result<()> {
        let instance = self.inner;
        if instance.backend.lock().unwrap().is_none() {
            return Err(Error::AppNotInit);
        }
        instance.run();
        Ok(())
    }
    /// Add widget
    /// Temporary function
    pub fn add_widget(&self, widget: Box<dyn Widget>) {
        self.inner.logic.lock().unwrap().add_widget(widget);
    }
}

