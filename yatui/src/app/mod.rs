//! Application structure
use crate::{
    backend::{Backend, Termion},
    layout::{Layout, LayoutDirection, LayoutType},
    terminal::{
        buffer::{Buffer, MappedBuffer},
        cursor::Cursor,
        region::Region,
    },
    widget::Widget,
};
use once_cell::sync::OnceCell;
use tokio::{runtime::Runtime, sync::Mutex, task::LocalSet};

// TODO: Remove send
pub struct App {
    backend: Mutex<Box<dyn Backend + Send>>,
    main: Mutex<Box<dyn Widget + Send>>,
}

impl App {
    fn new(backend: Box<dyn Backend + Send>) -> Self {
        Self {
            backend: Mutex::new(backend),
            main: Mutex::new(Box::new(Layout::new(
                LayoutDirection::Horizontal,
                LayoutType::Content,
            ))),
        }
    }
    /// Get a wrapper for reference to global application instance
    pub fn instance() -> &'static App {
        static INSTANCE: OnceCell<App> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            // TODO: Change output
            let default_backend = Termion::new(std::io::stdout()).unwrap();
            App::new(Box::new(default_backend))
        })
    }

    pub fn run(&'static self, rt: &Runtime) {
        let local = LocalSet::new();
        let mut buffer = Buffer::new(Region::new(Cursor::new(0, 0), Cursor::new(100, 100)));
        local.block_on(rt, async {
            loop {
                let mut main_lock = self.main.lock().await;
                main_lock.draw(MappedBuffer::new(
                    &mut buffer,
                    Region::new(Cursor::new(0, 0), Cursor::new(100, 100)),
                ));
                // Main actions here
            }
        });
    }
}
