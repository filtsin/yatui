/// Widget trait
pub mod textline;

use crate::terminal::{buffer::MappedBuffer, cursor::Index};

/// Widget should implement this trait for drawing. It is also implemented
/// by a [`Layout`](crate::layout::Layout)
pub trait Widget {
    /// Only one function that should be implemented by custom widget. Calls on every
    /// cycle of rendering by AppInstance.
    fn draw(&mut self, buf: MappedBuffer<'_>);
    /// Size hint for `[crate::layout::Layout]`
    fn size_hint(&self) -> SizeHint {
        SizeHint::Min(WidgetSize::new(1, 1))
    }
    /// Allows hide the widget
    fn is_show(&mut self) -> bool {
        true
    }

    fn take_focus(&mut self) {}

    // If sizes is not changing, widget can say that his content is not changed
    // from last draw, and we can not waste time on draw
    fn need_redraw(&self) -> bool {
        true
    }
}

/// Hint for [`Layout`](crate::layout::Layout). [`Layout`](crate::layout::Layout) should not ignore this value
/// and should take into account the wishes of [`Widget`] if required size is present
#[derive(Debug)]
#[non_exhaustive]
pub enum SizeHint {
    /// Widget needs exactly size
    Fixed(WidgetSize),
    /// Widget needs at least specified size
    Min(WidgetSize),
    /// Maximum of widget's size
    Max(WidgetSize),
    /// Min and max value of widget's size
    Range((WidgetSize, WidgetSize)),
}

/// Width and height of widget
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct WidgetSize {
    w: Index,
    h: Index,
}

impl WidgetSize {
    /// Construct new [`WidgetSize`]
    pub fn new(w: Index, h: Index) -> Self {
        Self { w, h }
    }
}
