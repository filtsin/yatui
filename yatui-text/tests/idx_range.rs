use yatui_text::IdxRange;

#[test]
fn create_from_ops_range() {
    assert_eq!(IdxRange::from(1..3), IdxRange::new(1, 2));
    assert_eq!(IdxRange::from(3..1), IdxRange::new(3, 2));
}

#[test]
#[should_panic]
fn create_from_ops_range_with_zero_on_start_and_end() {
    let _ = IdxRange::from(0..0);
}

#[test]
fn create_from_ops_range_from() {
    assert_eq!(IdxRange::from(1..), IdxRange::new(1, usize::MAX));
}

#[test]
fn create_from_ops_range_full() {
    assert_eq!(IdxRange::from(..), IdxRange::new(0, usize::MAX));
}

#[test]
fn create_from_ops_range_inclusive() {
    assert_eq!(IdxRange::from(1..=3), IdxRange::new(1, 3));
    assert_eq!(IdxRange::from(3..=1), IdxRange::new(3, 1));
}

#[test]
fn create_from_ops_range_to() {
    assert_eq!(IdxRange::from(..3), IdxRange::new(0, 2));
}

#[test]
fn create_from_ops_range_to_inclusive() {
    assert_eq!(IdxRange::from(..=3), IdxRange::new(0, 3));
}
