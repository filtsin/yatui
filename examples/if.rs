use std::cell::RefCell;

use yatui::{app::App, backend::Termion};

use yatui::{
    cb,
    component::{text, Component, DrawFn, SizeFn},
    state::{mut_state, State},
};

fn if_state<S, S2>(check: S, first: S2, second: S2) -> Component
where
    S: Into<State<bool>>,
    S2: Into<State<RefCell<Component>>>,
{
    let state = check.into();
    let state2 = state.clone();

    let first_state = first.into();
    let second_state = second.into();

    let first_state2 = first_state.clone();
    let second_state2 = second_state.clone();

    let draw_fn: DrawFn = cb!(move |printer, context| {
        let cond = context.get(&state);
        if *cond {
            let mut first = context.get(&first_state).borrow_mut();
            first.draw(printer, context);
        } else {
            let mut second = context.get(&second_state).borrow_mut();
            second.draw(printer, context);
        }
    });

    let size_fn: SizeFn = cb!(move |context| {
        let cond = context.get(&state2);
        if *cond {
            let mut first = context.get(&first_state2).borrow_mut();
            first.size_hint(context)
        } else {
            let mut second = context.get(&second_state2).borrow_mut();
            second.size_hint(context)
        }
    });

    Component::builder().draw_fn(draw_fn).size_fn(size_fn).build()
}

fn main() {
    let termion = Termion::new(std::io::stdout()).unwrap();
    let mut app = App::new(termion);

    let bool = mut_state(false);

    let entry = if_state(bool.clone(), text("Condition is true"), text("Condition is false"));

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        // Update here will re-render entry
        bool.set(true);
    });

    app.mount(entry);
    app.run();
}
