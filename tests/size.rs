use yatui::terminal::{Index, Size};

#[test]
fn add_width() {
    let size = Size::new(1, 1);
    let size2 = Size::new(5, 5);

    let result = size.add_width_size(size2);

    assert_eq!(result, Size::new(6, 5));
}

#[test]
fn add_width_overflow_should_be_max() {
    let size = Size::new(Index::MAX - 5, 20);
    let size2 = Size::new(10, 20);

    let result = size.add_width_size(size2);

    assert_eq!(result, Size::new(Index::MAX, 20));
}

#[test]
fn add_height() {
    let size = Size::new(1, 1);
    let size2 = Size::new(5, 5);

    let result = size.add_height_size(size2);

    assert_eq!(result, Size::new(5, 6));
}

#[test]
fn add_height_overflow_should_be_max() {
    let size = Size::new(20, Index::MAX - 5);
    let size2 = Size::new(20, 10);

    let result = size.add_height_size(size2);

    assert_eq!(result, Size::new(20, Index::MAX));
}
