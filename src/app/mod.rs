use crate::state::controller::Controller;

use tokio::{runtime::Runtime, task::LocalSet};

pub struct App {
    controller: Controller,
    // TODO
}

impl App {
    pub fn new() -> Self {
        Self { controller: Controller::new() }
    }

    pub fn mount(/*f: impl FnMut() -> Component*/) {
        todo!()
    }

    pub fn run(&mut self) {
        let rt = Runtime::new().unwrap();
        let local = LocalSet::new();

        local.block_on(&rt, self.main_loop())
    }

    async fn main_loop(&mut self) {}
}
