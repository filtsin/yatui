pub mod canvas;
pub mod layout;
pub mod size_hint;

use canvas::Canvas;
use layout::Layout;

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
