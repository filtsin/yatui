pub mod subscribe;

use std::fmt::Debug;

use crate::{
    component::size_hint::WidgetSize,
    compositor::context::Context,
    state::State,
    terminal::{buffer::MappedBuffer, cursor::Index},
};

use self::subscribe::Subscribe;

use super::{size_hint::SizeHint, Component};

type CanvasFn = dyn FnMut(MappedBuffer<'_>, Context<'_>);
type SizeFn = dyn Fn(Context<'_>) -> SizeHint;

pub struct Canvas {
    draw_fn: Box<CanvasFn>,
    size_fn: Option<Box<SizeFn>>,

    subscribe: Subscribe,
}

impl Canvas {
    pub fn new<F>(draw_fn: F) -> Self
    where
        F: FnMut(MappedBuffer<'_>, Context<'_>) + 'static,
    {
        Self { draw_fn: Box::new(draw_fn), size_fn: None, subscribe: Subscribe::Always }
    }

    pub fn set_size_fn<F>(&mut self, size_fn: F)
    where
        F: Fn(Context<'_>) -> SizeHint + 'static,
    {
        self.size_fn = Some(Box::new(size_fn));
    }

    pub fn add_subscribe<T>(&mut self, state: &State<T>) {
        match state {
            State::Value(_) => { /* nothing here */ }
            State::Pointer(pointer) => self.subscribe.push(pointer.id()),
        }
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

    pub fn need_redraw(&self, context: Context<'_>) -> bool {
        match self.subscribe {
            Subscribe::Always => true,
            Subscribe::Vec(ref vec) => vec.iter().any(|&v| context.is_changed_id(v)),
        }
    }
}

impl Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas").finish()
    }
}

// Example
pub fn text<S>(content: S) -> Component
where
    S: Into<State<String>>,
{
    let state = content.into();
    let state_clone = state.clone();

    let mut canvas = Canvas::new(move |buf: MappedBuffer<'_>, context: Context<'_>| {
        let content = context.get(&state);
        buf.with_state(0).write_text(content.as_ref());
    });

    canvas.add_subscribe(&state_clone);

    let size_fn = move |context: Context<'_>| {
        let context = context.get(&state_clone);
        SizeHint::new_fixed(WidgetSize::new(context.len() as Index, 1))
    };

    canvas.set_size_fn(size_fn);

    canvas.into()
}
