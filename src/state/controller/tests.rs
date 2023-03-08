use std::{cell::Cell, mem::needs_drop, rc::Rc};

use super::{Controller, Data};

struct DropValue {
    dropped: Rc<Cell<bool>>,
}

impl Drop for DropValue {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

fn create_value() -> (Rc<Cell<bool>>, DropValue) {
    let dropped = Rc::new(Cell::new(false));
    (dropped.clone(), DropValue { dropped })
}

#[test]
fn data_should_call_drop_inner_value() {
    let (dropped, value) = create_value();

    let data = Data::new(value);
    drop(data);

    assert!(dropped.get());
}

#[test]
fn data_cast_to_correct_type_should_return_reference() {
    let data = Data::new(451_i32);

    let inner_value = data.cast::<i32>();

    assert_eq!(*inner_value, 451);
}

#[test]
#[should_panic]
fn data_cast_to_incorrect_type_should_panic() {
    let data = Data::new(451_i32);

    let inner_value = data.cast::<String>();
}

#[test]
fn controller_push_value() {
    let mut controller = Controller::new();

    controller.push(5);
    controller.push("String");
    controller.push(Box::new(""));

    assert_eq!(controller.len(), 3);
}

#[test]
fn controller_get_exists_value_should_return_some() {
    let mut controller = Controller::new();

    controller.push(5);

    assert_eq!(controller.get::<i32>(0), Some(&5));
}

#[test]
fn controller_get_non_exists_value_should_return_none() {
    let mut controller = Controller::new();

    assert_eq!(controller.get::<i32>(0), None);
}

#[test]
#[should_panic]
fn controller_get_incorrect_type_should_panic() {
    let mut controller = Controller::new();

    controller.push(5);

    controller.get::<String>(0);
}

#[test]
fn controller_remove_from_should_drop_values() {
    let (dropped, value) = create_value();
    let mut controller = Controller::new();

    controller.push(5);
    let idx = controller.push(value);
    controller.remove_from(idx);

    assert_eq!(controller.len(), 1);
    assert!(dropped.get());
}

#[test]
#[should_panic]
fn controller_remove_from_should_panic_if_from_bigger_then_length() {
    let mut controller = Controller::new();
    controller.remove_from(0);
}
