use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
};

use crate::{
    terminal::{buffer::MappedBuffer, cursor::Index, region::Region},
    widget::{SizeHint, Widget},
};

use super::{Child, Layout, LayoutInfo};

/// Wrapper for custom layout with common logic. Probably you don't want to use this
/// on your code.
#[derive(Default)]
pub struct LayoutWrapper<T> {
    childs: Vec<Child>,
    last_region: Region,

    padding: Index,
    cached_size: Cell<Option<SizeHint>>,

    inner: T,
}

pub struct LayoutWrapperBuilder<T> {
    wrapper: LayoutWrapper<T>,
}

impl<T> LayoutWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self {
            childs: vec![],
            last_region: Region::default(),
            padding: Index::default(),
            inner,
            cached_size: Cell::new(None),
        }
    }

    pub fn add_widget<U>(&mut self, widget: U)
    where
        U: Widget + Send + 'static,
    {
        self.childs.push(Child::new(widget));
        self.invalidate();
    }

    pub fn remove_widget(&mut self, index: usize) -> Option<Box<dyn Widget + Send>> {
        if index >= self.childs.len() {
            return None;
        }

        self.invalidate();
        Some(self.childs.remove(index).widget)
    }

    pub fn set_padding(&mut self, padding: Index) {
        self.padding = padding;
    }

    fn draw_childs(&mut self, region: Region) {
        todo!()
    }

    fn update_size(&self) -> SizeHint {
        let mut result = SizeHint::zero();

        for child in &self.childs {
            result += child.update_size();
        }

        self.cached_size.replace(Some(result));
        result
    }

    fn invalidate(&mut self) {
        self.last_region = Region::default();
        self.cached_size.replace(None);
    }
}

impl<T> Widget for LayoutWrapper<T>
where
    T: Layout,
{
    fn draw(&mut self, buf: MappedBuffer<'_>) {
        // If the region has changed
        if buf.region() != self.last_region {
            // TODO: check padding value
            self.last_region = buf.region();
            self.inner.layout(
                buf.region(),
                LayoutInfo::new(self.childs.as_mut_slice(), self.cached_size.get().unwrap()),
            );
        }

        self.draw_childs(buf.padding(self.padding).region())
    }

    fn size_hint(&self) -> SizeHint {
        if self.cached_size.get().is_none() {
            self.update_size();
        } else {
            self.childs.iter().for_each(|v| {
                if v.size_changed() {
                    let cached_size = self.cached_size.get().unwrap();

                    let widget_cached_size = v.cached_size();
                    let widget_new_size = v.update_size();

                    if widget_new_size > widget_cached_size {
                        self.cached_size
                            .replace(Some(cached_size + (widget_new_size - widget_cached_size)));
                    } else {
                        self.cached_size
                            .replace(Some(cached_size - (widget_cached_size - widget_new_size)));
                    }
                }
            });
        }

        self.cached_size.get().unwrap()
    }

    fn take_focus(&mut self) {
        todo!()
    }

    fn need_redraw(&self) -> bool {
        todo!()
    }

    fn size_changed(&self) -> bool {
        self.cached_size.get().is_none() || self.childs.iter().any(|v| v.size_changed())
    }
}

impl<T> Deref for LayoutWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for LayoutWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
