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
    /// Size hint for `[crate::layout::Layout]`. This value can be cached. So if you want to
    /// change size of `Widget` you must invalidate the last announced size by
    /// returning `true` from `size_changed` function
    fn size_hint(&self) -> SizeHint {
        SizeHint::new_min(WidgetSize::new(1, 1))
    }
    /// Return true if last announced size (returned from `size_hint`)  was invalidated.
    /// May be called multiple times before getting new size by calling `size_hint`
    fn size_changed(&self) -> bool {
        false
    }

    fn take_focus(&mut self) {}

    // If sizes is not changing, widget can say that his content is not changed
    // from last draw, and we can not waste time on draw
    fn need_redraw(&self) -> bool {
        true
    }
}
