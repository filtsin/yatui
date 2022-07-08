mod cb;
pub mod layout;
pub mod subscription;

pub use self::{
    cb::{Cb, DrawFn, LayoutFn, SizeFn},
    subscription::Subscription,
};
use crate::{
    cb,
    compositor::context::Context,
    state::State,
    terminal::{buffer::MappedBuffer, region::Region, size::Size},
};

pub struct Component {
    draw_fn: DrawFn,
    pub layout_fn: Option<LayoutFn>,
    pub size_fn: Option<SizeFn>,
    sub: Subscription,
}

impl Component {
    pub fn new(draw_fn: DrawFn) -> Self {
        Self { draw_fn, layout_fn: None, size_fn: None, sub: Subscription::new() }
    }

    pub fn draw(&mut self, buf: MappedBuffer<'_>, context: Context<'_>) {
        (self.draw_fn.f)(buf, context)
    }

    pub fn layout(&mut self, region: Region, context: Context<'_>) {
        if let Some(layout_fn) = &mut self.layout_fn {
            (layout_fn.f)(region, context);
        }
    }

    pub fn size_hint(&mut self, context: Context<'_>) -> Size {
        match self.size_fn {
            Some(ref mut size_fn) => (size_fn.f)(context),
            None => Size::min(),
        }
    }

    pub fn have_changes(&self, context: Context<'_>) -> bool {
        self.sub.data().iter().any(|&x| context.is_changed_id(x))
    }
}

pub fn text<S>(content: S) -> Component
where
    S: Into<State<String>>,
{
    let state = content.into();
    let state2 = state.clone();

    let mut component = Component::new(cb!(move |mut buf, context| {
        let content = context.get(&state);
        buf.write_line(content, 0);
    }));

    component.size_fn = Some(cb!(move |context| {
        let content = context.get(&state2);
        Size::new(content.len().try_into().unwrap(), 1)
    }));

    component
}
