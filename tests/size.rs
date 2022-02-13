use yatui::terminal::{cursor::Index, size::Size};

#[test]
fn add() {
    let size = Size::new(1, 1);
    let size2 = Size::new(5, 5);

    let result = size + size2;

    assert_eq!(result, Size::new(6, 6));
}

#[test]
fn add_overflow_should_be_max() {
    let size = Size::new(Index::MAX - 5, Index::MAX - 4);
    let size2 = Size::new(10, 20);

    let result = size + size2;

    assert_eq!(result, Size::max());
}

#[test]
fn sub() {
    let size = Size::new(5, 5);
    let size2 = Size::new(1, 1);

    let result = size - size2;

    assert_eq!(result, Size::new(4, 4));
}

#[test]
fn sub_overflow_should_be_min() {
    let size = Size::new(5, 5);
    let size2 = Size::new(10, 20);

    let result = size - size2;

    assert_eq!(result, Size::min());
}
