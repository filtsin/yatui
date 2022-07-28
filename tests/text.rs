use yatui::terminal::{
    style::{Color, Style},
    text::{Str, Text},
};

use pretty_assertions::assert_eq;

#[test]
fn create_borrowed_and_owned_strings() {
    let str: Str = "hello".into();
    let string: Str = "hello".to_owned().into();

    assert!(str.is_borrowed());
    assert!(string.is_owned());
}

#[test]
fn change_styles_mut() {
    let mut str: Str = "hello".into();

    assert_eq!(str.styles(), Style::default());

    str.styles_mut().fg(Color::Red);

    assert_eq!(str.styles(), Style::new().fg(Color::Red));
}

#[test]
fn create_text_with_2part_of_str_should_not_allocate() {
    let str: Str = "hello".into();
    let str2 = str.clone();

    let text = Text::new([str, str2]);

    assert!(!text.spilled());
}

#[test]
fn create_text_with_more_than_2part_of_str_should_not_allocate() {
    let str: Str = "hello".into();
    let str2 = str.clone();
    let str3 = str.clone();

    let text = Text::new([str, str2, str3]);

    assert!(text.spilled());
}
