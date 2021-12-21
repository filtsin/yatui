use crate::terminal::buffer::MappedBuffer;

pub struct Canvas {
    draw_fn: Box<dyn FnMut(&MappedBuffer)>,
}

impl Canvas {
    pub fn new<F>(draw_fn: F) -> Self
    where
        F: FnMut(&MappedBuffer) + 'static,
    {
        Self { draw_fn: Box::new(draw_fn) }
    }

    pub fn draw(&mut self, buf: &MappedBuffer) {
        (self.draw_fn)(buf);
    }
}
