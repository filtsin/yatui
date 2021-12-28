use crate::{
    component::size_hint::WidgetSize, compositor::context::Context, state::State,
    terminal::buffer::MappedBuffer,
};

use super::{size_hint::SizeHint, Component};

type CanvasFn = dyn FnMut(MappedBuffer<'_>, Context<'_>);
type SizeFn = dyn Fn(Context<'_>) -> SizeHint;

pub struct Canvas {
    draw_fn: Box<CanvasFn>,
    size_fn: Option<Box<SizeFn>>,
}

impl Canvas {
    pub fn new<F>(draw_fn: F) -> Self
    where
        F: FnMut(MappedBuffer<'_>, Context<'_>) + 'static,
    {
        Self { draw_fn: Box::new(draw_fn), size_fn: None }
    }

    pub fn set_size_fn<F>(&mut self, size_fn: F)
    where
        F: Fn(Context<'_>) -> SizeHint + 'static,
    {
        self.size_fn = Some(Box::new(size_fn));
    }

    pub fn set_size_value(&mut self, value: SizeHint) {
        self.set_size_fn(move |_| value);
    }

    pub fn draw(&mut self, buf: MappedBuffer<'_>, context: Context<'_>) {
        (self.draw_fn)(buf, context);
    }

    pub fn size_hint(&self, context: Context<'_>) -> SizeHint {
        match &self.size_fn {
            Some(v) => v(context),
            None => SizeHint::new_max(WidgetSize::max()),
        }
    }
}

// Example
pub fn text<S, U>(content: S) -> Component
where
    S: Into<State<U>>,
    U: AsRef<str> + 'static,
{
    let state = content.into();
    let canvas = Canvas::new(move |buf: MappedBuffer<'_>, context: Context<'_>| {
        let content = context.get(&state);
        buf.with_state(0).write_text(content.as_ref());
    });

    canvas.into()
}
