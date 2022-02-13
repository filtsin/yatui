use yatui::terminal::{
    cursor::{Cursor, Index},
    region::Region,
    size::Size,
};

#[test]
fn create() {
    let v = Region::new(Cursor::new(0, 0), Cursor::new(1, 1));

    assert_eq!(v.left_top(), Cursor::new(0, 0));
    assert_eq!(v.right_bottom(), Cursor::new(1, 1));
}

#[test]
fn with_wrong_right_bottom_should_not_be_created() {
    // (0, 0)        (1, 0)
    //  ---------------
    //  |             |
    //  |             |
    //  |             |
    //  |             |
    //  |             |
    //  ---------------
    // (0, 1)        (1, 1)

    let _ = Region::try_new(Cursor::new(0, 1), Cursor::new(0, 0)).is_none();
    let _ = Region::try_new(Cursor::new(0, 1), Cursor::new(1, 0)).is_none();

    let _ = Region::try_new(Cursor::new(1, 0), Cursor::new(0, 0)).is_none();
    let _ = Region::try_new(Cursor::new(1, 0), Cursor::new(0, 1)).is_none();

    let _ = Region::try_new(Cursor::new(1, 1), Cursor::new(0, 0)).is_none();
    let _ = Region::try_new(Cursor::new(1, 1), Cursor::new(0, 1)).is_none();
    let _ = Region::try_new(Cursor::new(1, 1), Cursor::new(1, 0)).is_none();
}

#[test]
fn create_right_region() {
    // (0, 0)        (1, 0)
    //  ---------------
    //  |             |
    //  |             |
    //  |             |
    //  |             |
    //  |             |
    //  ---------------
    // (0, 1)        (1, 1)

    let _ = Region::try_new(Cursor::new(0, 0), Cursor::new(0, 0)).is_some();
    let _ = Region::try_new(Cursor::new(1, 0), Cursor::new(1, 0)).is_some();
    let _ = Region::try_new(Cursor::new(0, 1), Cursor::new(0, 1)).is_some();
    let _ = Region::try_new(Cursor::new(1, 1), Cursor::new(1, 1)).is_some();

    let _ = Region::try_new(Cursor::new(0, 0), Cursor::new(1, 0)).is_some();
    let _ = Region::try_new(Cursor::new(0, 0), Cursor::new(0, 1)).is_some();
    let _ = Region::try_new(Cursor::new(0, 0), Cursor::new(1, 1)).is_some();

    let _ = Region::try_new(Cursor::new(1, 0), Cursor::new(1, 1)).is_some();

    let _ = Region::try_new(Cursor::new(0, 1), Cursor::new(1, 1)).is_some();
}

#[test]
fn create_with_size() {
    let region = Region::with_size(Cursor::new(1, 0), Size::new(3, 4));

    assert_eq!(region.left_top(), Cursor::new(1, 0));
    assert_eq!(region.right_bottom(), Cursor::new(3, 3));
}

#[test]
#[should_panic]
fn create_with_zero_size_should_panic() {
    let _ = Region::with_size(Cursor::new(1, 0), Size::new(0, 1));
}

#[test]
#[should_panic]
fn create_with_size_overflow_should_panic() {
    let _ = Region::with_size(Cursor::new(Index::MAX, Index::MAX), Size::new(2, 2));
}

#[test]
fn size() {
    let region = Region::with_size(Cursor::new(0, 0), Size::new(3, 3));

    assert_eq!(region.width(), 3);
    assert_eq!(region.height(), 3);
    assert_eq!(region.area(), 9);
}

#[test]
fn region_iter() {
    let region_iter = Region::new(Cursor::new(0, 0), Cursor::new(1, 1)).into_iter();

    let result = vec![Cursor::new(0, 0), Cursor::new(1, 0), Cursor::new(0, 1), Cursor::new(1, 1)];
    let points: Vec<Cursor> = region_iter.collect();

    assert_eq!(points, result);

    let region_iter = Region::new(Cursor::new(0, 0), Cursor::new(0, 0)).into_iter();
    let result = vec![Cursor::new(0, 0)];
    let points: Vec<Cursor> = region_iter.collect();

    assert_eq!(points, result);
}
