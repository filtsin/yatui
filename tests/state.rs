use serial_test::serial;
use std::{mem::size_of, rc::Rc};

use yatui::{
    app::App,
    backend::Raw,
    state::{mut_state, mut_state_with, Pointer, State},
};

use pretty_assertions::assert_eq;

#[test]
fn mut_state_has_pointer_size() {
    assert_eq!(size_of::<Pointer<i32>>(), size_of::<usize>());
}

#[test]
#[serial]
fn mut_state_creation() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);

    app.process_event();

    let context = app.context();
    let state = State::Pointer(state);
    let result = context.get(&state);

    assert_eq!(0, *result);
}

#[test]
#[serial]
fn mut_state_with_creation() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state_with(|| Rc::new(0));

    app.process_event();

    let context = app.context();
    let state = State::Pointer(state);
    let result = context.get(&state);

    assert_eq!(Rc::new(0), *result);
}

#[test]
#[serial]
fn mut_state_set_value() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    state.set(1);

    app.process_event();

    let context = app.context();
    let state = State::Pointer(state);
    let result = context.get(&state);

    assert_eq!(1, *result);
}

#[test]
#[serial]
fn mut_state_set_with_value() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    state.set_with(|| 1 + 1);

    app.process_event();

    let context = app.context();
    let state = State::Pointer(state);
    let result = context.get(&state);

    assert_eq!(2, *result);
}

#[test]
#[serial]
fn mut_state_update_value() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    state.update(|v| *v = 1);

    app.process_event();

    let context = app.context();
    let state = State::Pointer(state);
    let result = context.get(&state);

    assert_eq!(1, *result);
}

#[test]
#[serial]
fn mut_state_clone_increment_ref_counter() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    let state2 = state.clone();

    app.process_event();

    let context = app.context();

    assert_eq!(2, context.ref_count(&State::Pointer(state)));
    assert_eq!(2, context.ref_count(&State::Pointer(state2)));
}

#[test]
#[serial]
fn mut_state_drop_decrement_ref_counter() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    let state2 = state.clone();
    drop(state2);

    app.process_event();

    let context = app.context();

    assert_eq!(1, context.ref_count(&State::Pointer(state)));
}

#[test]
#[serial]
fn mut_state_update_no_changes_ref_counter() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    let _state2 = state.clone(); // Ref counter = 2 now

    state.set(1);

    app.process_event();

    let context = app.context();

    assert_eq!(2, context.ref_count(&State::Pointer(state)));
}

#[test]
#[serial]
fn mut_state_changes_watcher() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = mut_state(0);
    state.set(1);

    let state2 = mut_state(0);
    state2.update(|v| *v = 1);

    let state3 = mut_state(0);
    state3.set_with(|| 1);

    app.process_event();

    let context = app.context();

    assert!(context.is_changed(&State::Pointer(state)));
    assert!(context.is_changed(&State::Pointer(state2)));
    assert!(context.is_changed(&State::Pointer(state3)));
}

#[test]
#[serial]
fn state_value_clone_increment_ref_counter() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = State::Value(Rc::new(0));
    let state2 = state.clone();

    app.process_event();

    let context = app.context();

    assert_eq!(2, context.ref_count(&state));
    assert_eq!(2, context.ref_count(&state2));
}

#[test]
#[serial]
fn state_value_get_by_context() {
    let backend = Raw::default();
    let mut app = App::new(backend);

    let state = State::Value(Rc::new(0));

    app.process_event();

    let context = app.context();
    assert_eq!(0, *context.get(&state));
}
