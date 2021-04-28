use crate::backend::Backend;
use crate::widget::Widget;
use std::rc::Rc;

pub(crate) struct Compositor {
    /// List of registered widgets
    widgets: Vec<Box<dyn Widget>>,
}

impl Compositor {
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }

    pub fn step(&self, backend: &mut dyn Backend) {
        backend.clear_screen();
        for v in self.widgets.iter() {
            backend.draw(v.draw().as_str());
        }
        backend.flush();
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}
