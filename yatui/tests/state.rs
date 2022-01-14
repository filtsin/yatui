use std::mem::size_of;

use yatui::{
    app::App,
    backend::StringB,
    state::{mut_state, Pointer, State},
};

#[test]
fn mut_state_has_pointer_size() {
    assert_eq!(size_of::<Pointer<i32>>(), size_of::<usize>());
}

#[test]
fn mut_state_change_value() {
    let backend = StringB::new();
    let mut app = App::new(backend);

    let mut state = mut_state(0);
    state.set(4);

    app.process_event();

    let context = app.context();
    let state = State::Pointer(state);
    let result = context.get(&state);

    assert_eq!(4, *result);
}

#[test]
fn state_change_value_without_controller() {
    let mut state: State<i32> = 0.into();
    state.set(1);

    let result: State<i32> = 1.into();

    assert_eq!(state, result);
}
