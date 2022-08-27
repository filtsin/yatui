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
    terminal::{buffer::MappedBuffer, Region, Size},
};
use derive_more::From;

pub struct Component {
    draw_fn: DrawFn,
    layout_fn: Option<LayoutFn>,
    size: Option<ComponentSize>,
    sub: Subscription,
}

#[derive(From)]
pub enum ComponentSize {
    Value(Size),
    Fn(SizeFn),
}

impl Component {
    fn new(draw_fn: DrawFn) -> Self {
        Self { draw_fn, layout_fn: None, size: None, sub: Subscription::new() }
    }

    pub fn builder() -> ComponentBuilder {
        ComponentBuilder::default()
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
        match self.size {
            Some(ref mut size) => size.get_size(context),
            None => Size::min(),
        }
    }

    pub fn have_changes(&self, context: Context<'_>) -> bool {
        self.sub.data().iter().any(|&x| context.is_changed_id(x))
    }
}

impl ComponentSize {
    pub fn get_size(&mut self, context: Context<'_>) -> Size {
        match &mut *self {
            ComponentSize::Value(v) => *v,
            ComponentSize::Fn(f) => (f.f)(context),
        }
    }
}

#[derive(Default)]
pub struct ComponentBuilder {
    draw_fn: Option<DrawFn>,
    layout_fn: Option<LayoutFn>,
    size: Option<ComponentSize>,
}

impl ComponentBuilder {
    pub fn draw_fn<F>(mut self, f: F) -> Self
    where
        F: Into<DrawFn>,
    {
        self.draw_fn = Some(f.into());
        self
    }

    pub fn layout_fn(mut self, f: LayoutFn) -> Self {
        self.layout_fn = Some(f);
        self
    }

    pub fn size_fn(mut self, f: SizeFn) -> Self {
        self.size = Some(f.into());
        self
    }

    pub fn size_value(mut self, size: Size) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn build(self) -> Component {
        let draw = self.draw_fn.unwrap_or_else(|| cb!(|_, _| {}));

        let mut component = Component::new(draw);
        component.layout_fn = self.layout_fn;
        component.size = self.size;

        component
    }
}

pub fn text<S>(content: S) -> Component
where
    S: Into<State<String>>,
{
    let state = content.into();
    let state2 = state.clone();

    let draw_fn: DrawFn = cb!(move |mut buf, context| {
        let content = context.get(&state);
        buf.write_line(content, 0);
    });

    let size_fn: SizeFn = cb!(move |context| {
        let content = context.get(&state2);
        Size::new(content.len().try_into().unwrap(), 1)
    });

    Component::builder().draw_fn(draw_fn).size_fn(size_fn).build()
}
