use crate::{compositor::context::Context, state::State, terminal::buffer::MappedBuffer};

use super::Component;

type CanvasFn = dyn FnMut(MappedBuffer<'_>, &Context<'_>);

pub struct Canvas {
    draw_fn: Box<CanvasFn>,
}

impl Canvas {
    pub fn new<F>(draw_fn: F) -> Self
    where
        F: FnMut(MappedBuffer<'_>, &Context<'_>) + 'static,
    {
        Self { draw_fn: Box::new(draw_fn) }
    }

    pub fn draw(&mut self, buf: MappedBuffer<'_>, context: &Context<'_>) {
        (self.draw_fn)(buf, context);
    }
}

// Example
pub fn text(content: State<String>) -> Component {
    let canvas = Canvas::new(move |buf: MappedBuffer<'_>, context: &Context<'_>| {
        let content = context.get(&content);
        buf.with_state(0).write_text(content);
    });

    Component::Canvas(canvas)
}
