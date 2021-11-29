use crate::terminal::buffer::MappedBuffer;

/// `Textline` widget
use super::{SizeHint, Widget, WidgetSize};
use crate::terminal::cursor::Index;

#[derive(Debug)]
pub struct TextLine {
    text: String,
}

impl TextLine {
    pub fn new(text: String) -> Self {
        Self { text }
    }
    pub fn text(&mut self, text: String) {
        self.text = text;
    }
}

impl Widget for TextLine {
    fn draw(&mut self, buf: MappedBuffer<'_>) {
        buf.with_state(0).write_text_overflow(&self.text, "...");
    }
    fn size_hint(&mut self) -> Option<SizeHint> {
        let widget_size = WidgetSize::new(Index::MAX, 1);
        Some(SizeHint::Max(widget_size))
    }
}
