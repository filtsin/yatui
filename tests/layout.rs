use serial_test::serial;
use yatui::{
    app::App,
    backend::Raw,
    cb,
    component::{
        layout::{column, line, Children},
        Component,
    },
    state::{mut_state_with, State},
    terminal::{Cursor, Index, Region, Size},
};

use pretty_assertions::assert_eq;

fn widget(w: Index, h: Index) -> Component {
    Component::builder().size_fn(cb!(move |_| Size::new(w, h))).build()
}

// TODO: Refactor tests (may be use macros for common code)

#[test]
#[serial]
fn line_elements() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state: State<Children> =
        mut_state_with(|| [widget(1, 1), widget(1, 1), widget(1, 3)]).into();

    app.process_event();

    let mut line = line(state.clone());
    line.size_hint(app.context());
    line.layout(region, app.context());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(0, 1), Cursor::new(0, 1))),
        Some(Region::new(Cursor::new(0, 2), Cursor::new(0, 4))),
    ];

    let layout_regions = app.context().get(&state).get_regions();

    assert_eq!(layout_regions, regions);
}

#[test]
#[serial]
fn column_elements() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state: State<Children> =
        mut_state_with(|| [widget(1, 1), widget(1, 1), widget(3, 1)]).into();

    app.process_event();

    let mut column = column(state.clone());
    column.size_hint(app.context());
    column.layout(region, app.context());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(1, 0), Cursor::new(1, 0))),
        Some(Region::new(Cursor::new(2, 0), Cursor::new(4, 0))),
    ];

    let layout_regions: Vec<Option<Region>> = app.context().get(&state).get_regions();

    assert_eq!(layout_regions, regions);
}

#[test]
#[serial]
fn line_elements_overflow() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state: State<Children> =
        mut_state_with(|| [widget(1, 1), widget(1, 2), widget(1, 3)]).into();

    app.process_event();

    let mut line = line(state.clone());
    line.size_hint(app.context());
    line.layout(region, app.context());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(0, 1), Cursor::new(0, 2))),
        Some(Region::new(Cursor::new(0, 3), Cursor::new(0, 4))),
    ];

    let layout_regions: Vec<Option<Region>> = app.context().get(&state).get_regions();

    assert_eq!(layout_regions, regions);
}

#[test]
#[serial]
fn column_elements_overflow() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state: State<Children> =
        mut_state_with(|| [widget(1, 1), widget(2, 1), widget(3, 1)]).into();

    app.process_event();

    let mut column = column(state.clone());
    column.size_hint(app.context());
    column.layout(region, app.context());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(1, 0), Cursor::new(2, 0))),
        Some(Region::new(Cursor::new(3, 0), Cursor::new(4, 0))),
    ];

    let layout_regions: Vec<Option<Region>> = app.context().get(&state).get_regions();

    assert_eq!(layout_regions, regions);
}

#[test]
#[serial]
fn line_persistent_layout() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state: State<Children> =
        mut_state_with(|| [widget(1, 3), widget(3, 3), widget(2, 3)]).into();

    app.process_event();

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 2))),
        Some(Region::new(Cursor::new(0, 3), Cursor::new(2, 4))),
        None,
    ];

    for _ in 0..100 {
        let mut layout = line(state.clone());
        layout.size_hint(app.context());
        layout.layout(region, app.context());

        let layout_regions = app.context().get(&state).get_regions();

        assert_eq!(layout_regions, regions);
    }
}

#[test]
#[serial]
fn column_persistent_layout() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state: State<Children> =
        mut_state_with(|| [widget(3, 1), widget(3, 3), widget(3, 2)]).into();

    app.process_event();

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(2, 0))),
        Some(Region::new(Cursor::new(3, 0), Cursor::new(4, 2))),
        None,
    ];

    for _ in 0..100 {
        let mut layout = column(state.clone());
        layout.size_hint(app.context());
        layout.layout(region, app.context());

        let layout_regions = app.context().get(&state).get_regions();

        assert_eq!(layout_regions, regions);
    }
}

#[test]
#[serial]
fn line_state_changed_layout() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state = mut_state_with(|| [widget(1, 1), widget(1, 1), widget(1, 1)]);

    app.process_event();

    let mut layout = line(state.clone());
    layout.size_hint(app.context());
    layout.layout(region, app.context());

    state.update(|v| v.push(widget(2, 2)));
    app.process_event();

    layout.size_hint(app.context());
    layout.layout(region, app.context());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(0, 1), Cursor::new(0, 1))),
        Some(Region::new(Cursor::new(0, 2), Cursor::new(0, 2))),
        Some(Region::new(Cursor::new(0, 3), Cursor::new(1, 4))),
    ];

    let state: State<Children> = state.into();
    let layout_regions = app.context().get(&state).get_regions();

    assert_eq!(layout_regions, regions);
}

#[test]
#[serial]
fn column_state_changed_layout() {
    let mut app = App::new(Raw::default());

    let region = Region::from(Size::new(5, 5));

    let state = mut_state_with(|| [widget(1, 1), widget(1, 1), widget(1, 1)]);

    app.process_event();

    let mut layout = column(state.clone());
    layout.size_hint(app.context());
    layout.layout(region, app.context());

    state.update(|v| v.push(widget(2, 2)));
    app.process_event();

    layout.size_hint(app.context());
    layout.layout(region, app.context());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(1, 0), Cursor::new(1, 0))),
        Some(Region::new(Cursor::new(2, 0), Cursor::new(2, 0))),
        Some(Region::new(Cursor::new(3, 0), Cursor::new(4, 1))),
    ];

    let state: State<Children> = state.into();
    let layout_regions = app.context().get(&state).get_regions();

    assert_eq!(layout_regions, regions);
}
