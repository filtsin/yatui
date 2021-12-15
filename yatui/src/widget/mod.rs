pub mod size_hint;
/// Widget trait
pub mod textline;

pub use size_hint::{SizeHint, WidgetSize};

use crate::terminal::buffer::MappedBuffer;

/// Widget should implement this trait for drawing. It is also implemented
/// by a [`Layout`](crate::layout::Layout)
pub trait Widget {
    /// Only one function that should be implemented by custom widget. Calls on every
    /// cycle of rendering by AppInstance.
    fn draw(&mut self, buf: MappedBuffer<'_>);
    /// Size hint for `[crate::layout::Layout]`
    fn size_hint(&self) -> SizeHint {
        SizeHint::new_min(WidgetSize::new(1, 1))
    }
    /// Allows hide the widget
    fn is_show(&self) -> bool {
        true
    }

    fn take_focus(&mut self) {}

    // If sizes is not changing, widget can say that his content is not changed
    // from last draw, and we can not waste time on draw
    fn need_redraw(&self) -> bool {
        true
    }
}
