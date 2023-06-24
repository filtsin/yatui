// use yatui::text::{
//     mask::StyleInfo,
//     part::{parts, Part},
//     Color, Style, StyledStr, Text,
// };
//
// #[test]
// fn part_iter_without_styles() {
//     let value = "he\tllo\nomg zero\r\nvalue";
//
//     let iter = parts(value, value.styles());
//
//     let result = vec![
//         Part::Str("he", 2, Style::default()),
//         Part::Str("llo", 3, Style::default()),
//         Part::NewLine,
//         Part::Str("omg zero", 8, Style::default()),
//         Part::NewLine,
//         Part::Str("value", 5, Style::default()),
//     ];
//
//     let v: Vec<_> = iter.collect();
//
//     assert_eq!(v, result);
// }
//
// #[test]
// fn part_iter_with_styles() {
//     let mut text: Text = "he\tllo\nomg zero\r\nvalue".into();
//     text.mask_mut().add(1..=4, Style::new().fg(Color::Red)); // e\tll
//     text.mask_mut().add(5..=6, Style::new().fg(Color::Blue)); // o\n
//     text.mask_mut().add(8..=9, Style::new().fg(Color::Green)); // mg
//     text.mask_mut().add(12..=18, Style::new().fg(Color::Yellow)); // ero\r\nval
//
//     let iter = parts(text.as_str(), text.mask().iter());
//
//     let result = vec![
//         Part::Str("h", 1, Style::default()),
//         Part::Str("e", 1, Style::new().fg(Color::Red)),
//         Part::Str("ll", 2, Style::new().fg(Color::Red)),
//         Part::Str("o", 1, Style::new().fg(Color::Blue)),
//         Part::NewLine,
//         Part::Str("o", 1, Style::default()),
//         Part::Str("mg", 2, Style::new().fg(Color::Green)),
//         Part::Str(" z", 2, Style::default()),
//         Part::Str("ero", 3, Style::new().fg(Color::Yellow)),
//         Part::NewLine,
//         Part::Str("val", 3, Style::new().fg(Color::Yellow)),
//         Part::Str("ue", 2, Style::default()),
//     ];
//
//     let v: Vec<_> = iter.collect();
//
//     assert_eq!(v, result);
// }
//
// #[test]
// fn part_iter_a_lot_zero_graphemes() {
//     let mut text: Text = "\t\0\r\r\n@\t\t\t\r\n\r\n\n老\t\t\t\r\n\t".into();
//     text.mask_mut().add(0..=0, Style::new().fg(Color::Red));
//     text.mask_mut().add(1..=1, Style::new().fg(Color::Blue));
//     text.mask_mut().add(2..=2, Style::new().fg(Color::Green));
//     text.mask_mut().add(3..=3, Style::new().fg(Color::Yellow));
//     text.mask_mut().add(4..=5, Style::new().fg(Color::Black));
//     text.mask_mut().add(6..=10, Style::new().fg(Color::Cyan));
//     text.mask_mut().add(11..=40, Style::new().fg(Color::White));
//
//     let iter = parts(text.as_str(), text.mask().iter());
//
//     let result = vec![
//         Part::NewLine,
//         Part::Str("@", 1, Style::new().fg(Color::Black)),
//         Part::NewLine,
//         Part::NewLine,
//         Part::NewLine,
//         Part::Str("老", 2, Style::new().fg(Color::White)),
//         Part::NewLine,
//     ];
//
//     let v: Vec<_> = iter.collect();
//
//     assert_eq!(v, result);
// }
//
// #[test]
// #[should_panic]
// fn not_increasing_style_ranges() {
//     let _ = parts(
//         "big text",
//         [StyleInfo::new(2..3, Style::default()), StyleInfo::new(0..4, Style::default())]
//             .into_iter(),
//     )
//     .collect::<Vec<_>>();
// }
