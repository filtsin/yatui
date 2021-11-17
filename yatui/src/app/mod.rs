//! Application structure
use crate::backend::{Backend, Termion};
use once_cell::sync::OnceCell;
use tokio::{runtime::Runtime, sync::Mutex, task::LocalSet};

pub struct App {
    backend: Mutex<Box<dyn Backend>>,
}

pub struct AppInstance {
    inner: &'static App,
}

impl App {
    fn new(backend: Box<dyn Backend>) -> Self {
        Self { backend: Mutex::new(backend) }
    }
    /// Get a wrapper for reference to global application instance
    pub fn instance() -> AppInstance {
        static INSTANCE: OnceCell<App> = OnceCell::new();
        let app = INSTANCE.get_or_init(|| {
            let default_backend = Termion::new(std::io::stdout()).unwrap();
            App::new(Box::new(default_backend))
        });
        AppInstance { inner: app }
    }
    fn run(&'static self, rt: &Runtime) {
        let local = LocalSet::new();
        local.block_on(rt, async {
            loop {
                // Main actions here
            }
        });
    }
}

impl AppInstance {
    /// Run an async event loop
    pub fn run(&self, rt: &Runtime) {
        let instance = self.inner;
        instance.run(rt);
    }
}
