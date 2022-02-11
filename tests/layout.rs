use yatui::{
    app::App,
    backend::Raw,
    component::{
        canvas::Canvas,
        layout::{column, line},
        Component,
    },
    terminal::{
        cursor::{Cursor, Index},
        region::Region,
        size::Size,
    },
};

use pretty_assertions::assert_eq;

macro_rules! context {
    () => {
        App::new(Raw::default()).context()
    };
}

fn widget(w: Index, h: Index) -> Component {
    let mut canvas = Canvas::new(|_, _| {});
    canvas.set_size_value(Size::new(w, h));
    canvas.into()
}

#[test]
fn line_elements() {
    let region = Region::from(Size::new(5, 5));

    let mut layout = line([widget(1, 1), widget(1, 1), widget(3, 1)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(1, 0), Cursor::new(1, 0))),
        Some(Region::new(Cursor::new(2, 0), Cursor::new(4, 0))),
    ];

    let layout_regions: Vec<Option<Region>> =
        layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn column_elements() {
    let region = Region::from(Size::new(5, 5));

    let mut layout = column([widget(1, 1), widget(1, 1), widget(1, 3)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(0, 1), Cursor::new(0, 1))),
        Some(Region::new(Cursor::new(0, 2), Cursor::new(0, 4))),
    ];

    let layout_regions: Vec<Option<Region>> =
        layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn line_elements_overflow() {
    let region = Region::from(Size::new(5, 5));

    let mut layout = line([widget(1, 1), widget(3, 1), widget(2, 1)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(1, 0), Cursor::new(3, 0))),
        Some(Region::new(Cursor::new(4, 0), Cursor::new(4, 0))),
    ];

    let layout_regions: Vec<Option<Region>> =
        layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn column_elements_overflow() {
    let region = Region::from(Size::new(5, 5));

    let mut layout = column([widget(1, 1), widget(1, 3), widget(1, 2)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 0))),
        Some(Region::new(Cursor::new(0, 1), Cursor::new(0, 3))),
        Some(Region::new(Cursor::new(0, 4), Cursor::new(0, 4))),
    ];

    let layout_regions: Vec<Option<Region>> =
        layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn line_persistent_layout() {
    let region = Region::from(Size::new(5, 5));

    let mut layout = line([widget(3, 1), widget(3, 3), widget(3, 2)]).layout().unwrap();

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(2, 0))),
        Some(Region::new(Cursor::new(3, 0), Cursor::new(4, 2))),
        None,
    ];

    for _ in 0..100 {
        layout.layout(region, context!());

        let layout_regions: Vec<Option<Region>> =
            layout.childs().iter().map(|child| child.region()).collect();

        assert_eq!(layout_regions, regions);
    }
}

#[test]
fn column_persistent_layout() {
    let region = Region::from(Size::new(5, 5));

    let mut layout = column([widget(1, 3), widget(3, 3), widget(2, 3)]).layout().unwrap();

    let regions = vec![
        Some(Region::new(Cursor::new(0, 0), Cursor::new(0, 2))),
        Some(Region::new(Cursor::new(0, 3), Cursor::new(2, 4))),
        None,
    ];

    for _ in 0..100 {
        layout.layout(region, context!());

        let layout_regions: Vec<Option<Region>> =
            layout.childs().iter().map(|child| child.region()).collect();

        assert_eq!(layout_regions, regions);
    }
}

