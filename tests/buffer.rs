// use yatui::terminal::{buffer::Buffer, character::Character, cursor::Cursor, size::Size};
//
// use pretty_assertions::assert_eq;
//
// #[test]
// fn write_character() {
//     let mut buffer = Buffer::new(Size::new(3, 3));
//
//     buffer.write_character('0', Cursor::new(0, 0));
//     buffer.write_character('1', Cursor::new(1, 0));
//     buffer.write_character('2', Cursor::new(2, 0));
//     buffer.write_character('3', Cursor::new(0, 1));
//     buffer.write_character('4', Cursor::new(1, 1));
//     buffer.write_character('5', Cursor::new(2, 1));
//     buffer.write_character('6', Cursor::new(0, 2));
//     buffer.write_character('7', Cursor::new(1, 2));
//     buffer.write_character('8', Cursor::new(2, 2));
//
//     #[rustfmt::skip]
//     let s = vec![
//         "012",
//         "345",
//         "678"
//     ];
//
//     assert_eq!(buffer, Buffer::from(s));
// }
//
// #[test]
// #[should_panic]
// fn write_character_overflow_should_panic() {
//     let mut buffer = Buffer::new(Size::new(1, 1));
//
//     buffer.write_character('0', Cursor::new(1, 1));
// }
//
// #[test]
// fn resize() {
//     let mut buffer = Buffer::new(Size::new(3, 3));
//     assert_eq!(buffer.size(), Size::new(3, 3));
//
//     buffer.resize(Size::new(5, 5));
//     assert_eq!(buffer.size(), Size::new(5, 5));
//
//     buffer.resize(Size::new(1, 1));
//     assert_eq!(buffer.size(), Size::new(1, 1));
// }
//
// #[test]
// fn get() {
//     let mut buffer = Buffer::new(Size::new(3, 3));
//
//     buffer.write_character('0', Cursor::new(1, 1));
//
//     let character = buffer.get(Cursor::new(1, 1));
//
//     assert_eq!(*character, Character::from('0'));
// }
//
// #[test]
// #[should_panic]
// fn get_overflow_should_panic() {
//     let buffer = Buffer::new(Size::new(3, 3));
//
//     buffer.get(Cursor::new(3, 3));
// }
//
// #[test]
// fn get_index() {
//     let buffer = Buffer::new(Size::new(3, 3));
//
//     let index = buffer.get_index(&Cursor::new(1, 2));
//
//     assert_eq!(index, 7);
// }
