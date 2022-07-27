use yatui::terminal::text::Str;

#[test]
fn create() {
    let str: Str = "hello".into();
    let string: Str = "hello".to_owned().into();

    assert!(str.is_borrowed());
    assert!(string.is_owned());
}
