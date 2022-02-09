pub mod canvas;
pub mod layout;
pub mod size_hint;

use canvas::Canvas;
use layout::Layout;

use crate::{compositor::context::Context, terminal::buffer::MappedBuffer};

use self::size_hint::SizeHint;

#[derive(Debug)]
pub enum Component {
    Canvas(Box<Canvas>),
    Layout(Box<Layout>),
}

impl From<Canvas> for Component {
    fn from(v: Canvas) -> Self {
        Self::Canvas(Box::new(v))
    }
}

impl From<Layout> for Component {
    fn from(v: Layout) -> Self {
        Self::Layout(Box::new(v))
    }
}

impl Component {
    pub fn canvas(self) -> Result<Box<Canvas>, Self> {
        match self {
            Component::Canvas(c) => Ok(c),
            Component::Layout(_) => Err(self),
        }
    }

    pub fn layout(self) -> Result<Box<Layout>, Self> {
        match self {
            Component::Canvas(_) => Err(self),
            Component::Layout(l) => Ok(l),
        }
    }

    pub fn size_hint(&self, context: Context<'_>) -> SizeHint {
        match self {
            Component::Canvas(c) => c.size_hint(context),
            Component::Layout(l) => l.size_hint(context),
        }
    }

    pub fn draw(&mut self, buffer: MappedBuffer<'_>, context: Context<'_>) {
        match self {
            Component::Canvas(c) => c.draw(buffer, context),
            Component::Layout(l) => l.draw(buffer, context),
        }
    }
}
