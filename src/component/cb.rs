use crate::{
    compositor::context::Context,
    terminal::{buffer::MappedBuffer, region::Region, size::Size},
};
use std::ops::{Deref, DerefMut};

pub struct Cb<F: ?Sized> {
    pub(crate) f: Box<F>,
}

impl<F: ?Sized> Cb<F> {
    pub fn new(f: Box<F>) -> Self {
        Self { f }
    }
}

pub type DrawFn = Cb<dyn FnMut(MappedBuffer<'_>, Context<'_>)>;
pub type LayoutFn = Cb<dyn FnMut(Region, Context<'_>)>;
pub type SizeFn = Cb<dyn FnMut(Context<'_>) -> Size>;

#[macro_export]
macro_rules! cb {
    ($code:expr) => {
        $crate::component::Cb::new(Box::new($code))
    };
}
