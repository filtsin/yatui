use std::ops::RangeInclusive;

use yatui::{
    app::App,
    backend::Raw,
    component::{
        canvas::Canvas,
        layout::{column, line},
        size_hint::{SizeHint, WidgetSize},
        Component,
    },
    terminal::{
        cursor::{Cursor, Index},
        region::Region,
    },
};

macro_rules! context {
    () => {
        App::new(Raw::new(Cursor::default())).context()
    };
}

fn widget(w: Index, h: Index) -> Component {
    widget_range((w, h)..=(w, h))
}

fn widget_range(range: RangeInclusive<(Index, Index)>) -> Component {
    let mut canvas = Canvas::new(|_, _| {});
    canvas.set_size_value(SizeHint::new(
        WidgetSize::new(range.start().0, range.start().1),
        WidgetSize::new(range.end().0, range.end().1),
    ));
    canvas.into()
}

#[test]
fn line_elements() {
    let region = Region::from(WidgetSize::new(5, 5));

    let mut layout = line([widget(1, 1), widget(1, 1), widget(3, 1)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Region::new(Cursor::new(0, 0), Cursor::new(1, 1)),
        Region::new(Cursor::new(0, 1), Cursor::new(1, 2)),
        Region::new(Cursor::new(0, 2), Cursor::new(1, 5)),
    ];

    let layout_regions: Vec<Region> = layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn column_elements() {
    let region = Region::from(WidgetSize::new(5, 5));

    let mut layout = column([widget(1, 1), widget(1, 1), widget(1, 3)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Region::new(Cursor::new(0, 0), Cursor::new(1, 1)),
        Region::new(Cursor::new(1, 0), Cursor::new(2, 1)),
        Region::new(Cursor::new(2, 0), Cursor::new(5, 1)),
    ];

    let layout_regions: Vec<Region> = layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn line_elements_overflow() {
    let region = Region::from(WidgetSize::new(5, 5));

    let mut layout = line([widget(1, 1), widget(3, 1), widget(2, 1)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Region::new(Cursor::new(0, 0), Cursor::new(1, 1)),
        Region::new(Cursor::new(0, 1), Cursor::new(1, 4)),
        Region::new(Cursor::new(0, 4), Cursor::new(1, 5)),
    ];

    let layout_regions: Vec<Region> = layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}

#[test]
fn column_elements_overflow() {
    let region = Region::from(WidgetSize::new(5, 5));

    let mut layout = column([widget(1, 1), widget(1, 3), widget(1, 2)]).layout().unwrap();
    layout.layout(region, context!());

    let regions = vec![
        Region::new(Cursor::new(0, 0), Cursor::new(1, 1)),
        Region::new(Cursor::new(1, 0), Cursor::new(4, 1)),
        Region::new(Cursor::new(4, 0), Cursor::new(5, 1)),
    ];

    let layout_regions: Vec<Region> = layout.childs().iter().map(|child| child.region()).collect();

    assert_eq!(layout_regions, regions);
}
