pub mod canvas;
pub mod layout;
pub mod size_hint;

use canvas::Canvas;
use layout::Layout;

use crate::compositor::context::Context;

use self::size_hint::SizeHint;

pub enum Component {
    Canvas(Canvas),
    Layout(Layout),
}

impl From<Canvas> for Component {
    fn from(v: Canvas) -> Self {
        Self::Canvas(v)
    }
}

impl From<Layout> for Component {
    fn from(v: Layout) -> Self {
        Self::Layout(v)
    }
}

impl Component {
    pub fn size_hint(&self, context: Context<'_>) -> SizeHint {
        match self {
            Component::Canvas(c) => c.size_hint(context),
            Component::Layout(l) => l.size_hint(context),
        }
    }
}
