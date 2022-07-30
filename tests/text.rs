use yatui::text::{graphemes::Grapheme, Color, Style, Text};

use pretty_assertions::assert_eq;

#[test]
fn create_borrowed_and_owned_strings() {
    let str: Text = "hello".into();
    let string: Text = "hello".to_owned().into();

    assert!(str.is_borrowed());
    assert!(string.is_owned());
}

// #[test]
// fn change_styles_mut() {
//     let mut str: Text = "hello".into();
//
//     assert_eq!(str.styles(), Style::default());
//
//     str.styles_mut().fg(Color::Red);
//
//     assert_eq!(str.styles(), Style::new().fg(Color::Red));
// }
//
// #[test]
// fn create_text_with_2part_of_str_should_not_allocate() {
//     let str: Str = "hello".into();
//     let str2 = str.clone();
//
//     let text = Text::new([str, str2]);
//
//     assert!(!text.spilled());
// }
//
// #[test]
// fn create_text_with_more_than_2part_of_str_should_not_allocate() {
//     let str: Str = "hello".into();
//     let str2 = str.clone();
//     let str3 = str.clone();
//
//     let text = Text::new([str, str2, str3]);
//
//     assert!(text.spilled());
// }
//
// #[test]
// fn length_ascii_string() {
//     let str: Str = "hello".into();
//
//     assert_eq!(str.len(), 5);
// }
//
// #[test]
// fn update_content_should_update_len() {
//     let mut str: Str = "hello".into();
//
//     str.update_content("hello world");
//
//     assert_eq!(str.len(), 11);
// }

#[test]
fn length_not_ascii_string() {
    let str: Text = "привет".into();

    assert_eq!(str.len(), 6);

    let str: Text = "Löwe 老虎 Léopard".into();

    assert_eq!(str.len(), 15);

    let str: Text = "❤️🧡💛💚💙💜".into();

    assert_eq!(str.len(), 6);
}
