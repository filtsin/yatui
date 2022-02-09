pub mod controller;

pub enum Event {
    Controller(controller::Event),
}
