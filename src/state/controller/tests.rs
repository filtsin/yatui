use std::{cell::RefCell, rc::Rc};

use super::{CallBack, Controller, Data};

struct DropValue {
    dropped: Option<Rc<RefCell<bool>>>,
}

impl<'a> Drop for DropValue {
    fn drop(&mut self) {
        if let Some(dropped) = self.dropped.as_mut() {
            *dropped.borrow_mut() = true;
        }
    }
}

fn get_data<T>(data: T) -> Data {
    Data::new(Box::into_raw(Box::new(data)) as *mut u8).unwrap()
}

impl<'a> DropValue {
    fn empty() -> Data {
        get_data(Self { dropped: None })
    }

    fn with(v: Rc<RefCell<bool>>) -> Data {
        get_data(Self { dropped: Some(v) })
    }

    fn destructor() -> CallBack {
        Box::new(|v| unsafe {
            Box::from_raw(v.cast::<Self>().as_ptr());
        })
    }
}

#[test]
#[should_panic]
fn insert_already_exists_should_panic() {
    let mut controller = Controller::default();
    unsafe {
        controller.insert(1, DropValue::empty(), DropValue::destructor());
        controller.insert(1, DropValue::empty(), DropValue::destructor());
    }
}

#[test]
#[should_panic]
fn remove_not_exists_should_panic() {
    let mut controller = Controller::default();
    controller.remove(1);
}

#[test]
#[should_panic]
fn subscribe_not_exists_should_panic() {
    let mut controller = Controller::default();
    controller.subscribe(1);
}

#[test]
#[should_panic]
fn unsubscribe_not_exists_should_panic() {
    let mut controller = Controller::default();
    controller.unsubscribe(1);
}

#[test]
#[should_panic]
fn get_not_exists_should_panic() {
    let controller = Controller::default();
    controller.get(1);
}

#[test]
fn remove_should_drop_value() {
    let mut dropped = Rc::new(RefCell::new(false));

    let mut controller = Controller::default();

    unsafe {
        controller.insert(1, DropValue::with(dropped.clone()), DropValue::destructor());
    }

    controller.remove(1);

    assert!(*dropped.borrow());
}

#[test]
fn drop_should_drop_value() {
    let mut dropped = Rc::new(RefCell::new(false));

    let mut controller = Controller::default();

    unsafe {
        controller.insert(1, DropValue::with(dropped.clone()), DropValue::destructor());
    }

    drop(controller);

    assert!(*dropped.borrow());
}

#[test]
fn unsubscribe_should_drop_value() {
    let mut dropped = Rc::new(RefCell::new(false));

    let mut controller = Controller::default();

    unsafe {
        controller.insert(1, DropValue::with(dropped.clone()), DropValue::destructor());
    }

    controller.subscribe(1); // Ref counter = 2

    controller.unsubscribe(1); // Ref counter = 1
    assert!(!*dropped.borrow());

    controller.unsubscribe(1); // Ref counter = 0
    assert!(*dropped.borrow());
}
