use std::{cell::RefCell, ops::Deref, rc::Rc};

use log::info;

use crate::{
    component::Component,
    compositor::context::Context,
    terminal::{
        cursor::{Cursor, Index},
        region::Region,
        size::Size,
    },
};

pub struct Children {
    pub(crate) data: RefCell<Vec<Child>>,
}

impl Children {
    pub fn new<C>(v: C) -> Self
    where
        C: IntoIterator<Item = Component>,
    {
        let data = RefCell::new(
            v.into_iter().enumerate().map(|(i, component)| Child::new(component, i)).collect(),
        );
        Self { data }
    }

    pub fn add_component(&mut self, component: Component) {
        let count = self.data.borrow().len();
        self.data.borrow_mut().push(Child::new(component, count));
    }

    pub(crate) fn layout_all(&self, context: Context<'_>) {
        let mut data = self.data.borrow_mut();
        for child in data.iter_mut() {
            child.try_merge_transcation();
            child.layout(context);
        }
    }
}

pub(crate) struct Child {
    pub(crate) component: Component,
    size: Size,
    region: Option<Region>,

    index: usize,
    transaction: Option<ChildRegionTranscation>,
}

impl Child {
    fn new(component: Component, index: usize) -> Self {
        Self { component, size: Size::min(), region: None, index, transaction: None }
    }

    pub fn layout(&mut self, context: Context<'_>) {
        self.component.layout(self.region().unwrap(), context);
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn region(&self) -> Option<Region> {
        self.region
    }

    pub fn update_region(&mut self, region: Region) {
        self.region = Some(region);
    }

    pub fn update_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn my_index(&self) -> usize {
        self.index
    }

    pub(crate) fn start_transcation(&mut self) -> &mut ChildRegionTranscation {
        if self.transaction.is_none() {
            info!("Create new transcation");
            self.transaction = Some(ChildRegionTranscation::new(self));
        }

        self.transaction.as_mut().unwrap()
    }

    pub(crate) fn try_merge_transcation(&mut self) {
        if let Some(transcation) = self.transaction.take() {
            self.region = Some(transcation.build_region());
        }
    }
}

pub(crate) struct ChildRegionTranscation {
    left_top: Cursor,
    right_bottom: Cursor,
}

impl ChildRegionTranscation {
    fn new(child: &Child) -> ChildRegionTranscation {
        let region = child.region().unwrap_or_default();

        Self { left_top: region.left_top(), right_bottom: region.right_bottom() }
    }

    pub fn change_left_x(&mut self, new_x: Index) {
        self.left_top.set_column(new_x);
    }

    pub fn change_left_y(&mut self, new_y: Index) {
        self.left_top.set_row(new_y);
    }

    pub fn change_right_x(&mut self, new_x: Index) {
        self.right_bottom.set_column(new_x);
    }

    pub fn change_right_y(&mut self, new_y: Index) {
        self.right_bottom.set_row(new_y);
    }

    pub fn build_region(&self) -> Region {
        Region::new(self.left_top, self.right_bottom)
    }
}
