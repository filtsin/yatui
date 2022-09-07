use yatui::terminal::{Cursor, Index, Region, Size};

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

    assert!(Region::try_new(Cursor::new(0, 1), Cursor::new(0, 0)).is_none());

    assert!(Region::try_new(Cursor::new(0, 1), Cursor::new(1, 0)).is_none());

    assert!(Region::try_new(Cursor::new(1, 0), Cursor::new(0, 0)).is_none());
    assert!(Region::try_new(Cursor::new(1, 0), Cursor::new(0, 1)).is_none());

    assert!(Region::try_new(Cursor::new(1, 1), Cursor::new(0, 0)).is_none());
    assert!(Region::try_new(Cursor::new(1, 1), Cursor::new(0, 1)).is_none());
    assert!(Region::try_new(Cursor::new(1, 1), Cursor::new(1, 0)).is_none());
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

    assert!(Region::try_new(Cursor::new(0, 0), Cursor::new(0, 0)).is_some());
    assert!(Region::try_new(Cursor::new(1, 0), Cursor::new(1, 0)).is_some());
    assert!(Region::try_new(Cursor::new(0, 1), Cursor::new(0, 1)).is_some());
    assert!(Region::try_new(Cursor::new(1, 1), Cursor::new(1, 1)).is_some());

    assert!(Region::try_new(Cursor::new(0, 0), Cursor::new(1, 0)).is_some());
    assert!(Region::try_new(Cursor::new(0, 0), Cursor::new(0, 1)).is_some());
    assert!(Region::try_new(Cursor::new(0, 0), Cursor::new(1, 1)).is_some());

    assert!(Region::try_new(Cursor::new(1, 0), Cursor::new(1, 1)).is_some());

    assert!(Region::try_new(Cursor::new(0, 1), Cursor::new(1, 1)).is_some());
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

#[test]
fn first_line() {
    let region = Region::new(Cursor::new(1, 2), Cursor::new(4, 4));

    let result = Region::new(Cursor::new(1, 2), Cursor::new(4, 2));

    assert_eq!(region.first_line(), result);
}

#[test]
fn last_line() {
    let region = Region::new(Cursor::new(1, 2), Cursor::new(4, 4));

    let result = Region::new(Cursor::new(1, 4), Cursor::new(4, 4));

    assert_eq!(region.last_line(), result);
}

#[test]
fn first_column() {
    let region = Region::new(Cursor::new(1, 2), Cursor::new(4, 4));

    let result = Region::new(Cursor::new(1, 2), Cursor::new(1, 4));

    assert_eq!(region.first_column(), result);
}

#[test]
fn last_column() {
    let region = Region::new(Cursor::new(1, 2), Cursor::new(4, 4));

    let result = Region::new(Cursor::new(4, 2), Cursor::new(4, 4));

    assert_eq!(region.last_column(), result);
}

#[test]
fn line_and_column_with_size_one() {
    let region = Region::with_size(Cursor::new(0, 0), Size::new(1, 1));

    let result = Region::new(Cursor::new(0, 0), Cursor::new(0, 0));

    assert_eq!(region.last_column(), region.first_column());
    assert_eq!(region.last_line(), region.first_line());
    assert_eq!(region.first_line(), region.first_column());
    assert_eq!(region.first_line(), result);
}

#[test]
fn n_line() {
    let region = Region::new(Cursor::new(1, 1), Cursor::new(3, 3));

    let line0 = Region::new(Cursor::new(1, 1), Cursor::new(3, 1));
    let line1 = Region::new(Cursor::new(1, 2), Cursor::new(3, 2));
    let line2 = Region::new(Cursor::new(1, 3), Cursor::new(3, 3));

    assert_eq!(region.n_line(0), Some(line0));
    assert_eq!(region.n_line(1), Some(line1));
    assert_eq!(region.n_line(2), Some(line2));
    assert_eq!(region.n_line(3), None);
}

#[test]
fn n_column() {
    let region = Region::new(Cursor::new(1, 1), Cursor::new(3, 3));

    let column0 = Region::new(Cursor::new(1, 1), Cursor::new(1, 3));
    let column1 = Region::new(Cursor::new(2, 1), Cursor::new(2, 3));
    let column2 = Region::new(Cursor::new(3, 1), Cursor::new(3, 3));

    assert_eq!(region.n_column(0), Some(column0));
    assert_eq!(region.n_column(1), Some(column1));
    assert_eq!(region.n_column(2), Some(column2));
    assert_eq!(region.n_column(3), None);
}
