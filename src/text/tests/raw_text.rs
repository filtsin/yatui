// use crate::text::raw_text::RawText;
//
// #[test]
// pub fn create_text_with_static_str_without_allocation() {
//     let text: RawText = "234".into();
//
//     assert_eq!(text.capacity(), 0);
//     assert!(text.is_borrowed());
//     assert!(!text.is_owned());
//     assert_eq!(text.columns(), 3);
//     assert_eq!(text.lines(), 1);
//     assert_eq!(text.size(), 3);
// }
//
// #[test]
// pub fn modify_inner_string() {
//     let mut text: RawText = "234".into();
//     text.modify(|t| {
//         t.push_str("567");
//     });
// }
