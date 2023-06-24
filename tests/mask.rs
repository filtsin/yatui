// use yatui::text::{Color, Mask, Modifier, Style};
//
// #[test]
// fn change_mask_non_overlapping() {
//     let mut mask = Mask::new();
//
//     mask.add(..2, Style::new().fg(Color::Red));
//     mask.add(2..5, Style::new().fg(Color::Blue));
//     mask.add(5..=6, Style::new().fg(Color::Green));
//     mask.add(7.., Style::new().fg(Color::Yellow));
//
//     let result = vec![
//         (0..=1, Style::new().fg(Color::Red)),
//         (2..=4, Style::new().fg(Color::Blue)),
//         (5..=6, Style::new().fg(Color::Green)),
//         (7..=usize::MAX, Style::new().fg(Color::Yellow)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// // ─────────────
// // x'          y'
// #[test]
// fn change_mask_overlapping_1() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=10, Style::new().fg(Color::Red));
//     mask.add(0..=10, Style::new().bg(Color::Blue));
//
//     let result = vec![(0..=10, Style::new().fg(Color::Red).bg(Color::Blue))];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// // ───────────────
// // x'            y'
// #[test]
// fn change_mask_overlapping_2() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=10, Style::new().fg(Color::Red));
//     mask.add(0..=15, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (11..=15, Style::new().bg(Color::Blue)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// //   ────────
// //   x'     y'
// #[test]
// fn change_mask_overlapping_3() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=10, Style::new().fg(Color::Red));
//     mask.add(3..=5, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red)),
//         (3..=5, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (6..=10, Style::new().fg(Color::Red)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// // // ─────────────
// // // x           y
// // //    ──────────
// // //    x'     y'
// #[test]
// fn change_mask_overlapping_4() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=10, Style::new().fg(Color::Red));
//     mask.add(3..=10, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red)),
//         (3..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// //    ────────────
// //    x'         y'
// #[test]
// fn change_mask_overlapping_5() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=10, Style::new().fg(Color::Red));
//     mask.add(3..=15, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red)),
//         (3..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (11..=15, Style::new().bg(Color::Blue)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //
// //      ─────────────
// //      x           y
// // ──────
// // x'   y'
// #[test]
// fn change_mask_overlapping_6() {
//     let mut mask = Mask::new();
//
//     mask.add(4..=10, Style::new().fg(Color::Red));
//     mask.add(0..=4, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=3, Style::new().bg(Color::Blue)),
//         (4..=4, Style::new().bg(Color::Blue).fg(Color::Red)),
//         (5..=10, Style::new().fg(Color::Red)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //
// //      ─────────────
// //      x           y
// // ──────────
// // x'       y'
// #[test]
// fn change_mask_overlapping_7() {
//     let mut mask = Mask::new();
//
//     mask.add(4..=10, Style::new().fg(Color::Red));
//     mask.add(0..=7, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=3, Style::new().bg(Color::Blue)),
//         (4..=7, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (8..=10, Style::new().fg(Color::Red)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //
// //      ─────────────
// //      x           y
// // ──────────────────
// // x'               y'
// #[test]
// fn change_mask_overlapping_8() {
//     let mut mask = Mask::new();
//
//     mask.add(4..=10, Style::new().fg(Color::Red));
//     mask.add(0..=10, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=3, Style::new().bg(Color::Blue)),
//         (4..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //
// //      ─────────────
// //      x           y
// // ──────────────────────
// // x'                   y'
// #[test]
// fn change_mask_overlapping_9() {
//     let mut mask = Mask::new();
//
//     mask.add(4..=10, Style::new().fg(Color::Red));
//     mask.add(0..=15, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=3, Style::new().bg(Color::Blue)),
//         (4..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (11..=15, Style::new().bg(Color::Blue)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //
// //      ─────────────
// //      x           y
// //         ─────
// //         x'  y'
// #[test]
// fn change_mask_overlapping_10() {
//     let mut mask = Mask::new();
//
//     mask.add(4..=10, Style::new().fg(Color::Red));
//     mask.add(7..=9, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (4..=6, Style::new().fg(Color::Red)),
//         (7..=9, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (10..=10, Style::new().fg(Color::Red)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //  ───────     ─────
// //  x''  y''    x'  y'
// //  ───────────────────
// //  x               y
// #[test]
// fn change_mask_overlapping_11() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=3, Style::new().fg(Color::Red));
//     mask.add(5..=10, Style::new().bg(Color::Red));
//     mask.add(0..=12, Style::new().modifier(Modifier::BOLD));
//
//     let result = vec![
//         (0..=3, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
//         (4..=4, Style::new().modifier(Modifier::BOLD)),
//         (5..=10, Style::new().bg(Color::Red).modifier(Modifier::BOLD)),
//         (11..=12, Style::new().modifier(Modifier::BOLD)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //  ───────     ─────
// //  x''  y''    x'  y'
// //  ──────────────
// //  x            y
// #[test]
// fn change_mask_overlapping_12() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=3, Style::new().fg(Color::Red));
//     mask.add(5..=10, Style::new().bg(Color::Red));
//     mask.add(0..=8, Style::new().modifier(Modifier::BOLD));
//
//     let result = vec![
//         (0..=3, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
//         (4..=4, Style::new().modifier(Modifier::BOLD)),
//         (5..=8, Style::new().bg(Color::Red).modifier(Modifier::BOLD)),
//         (9..=10, Style::new().bg(Color::Red)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// //  ───── ───── ─────
// //  ──────────────
// //  x            y
// #[test]
// fn change_mask_overlapping_13() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=3, Style::new().fg(Color::Red));
//     mask.add(4..=7, Style::new().fg(Color::Green));
//     mask.add(8..=10, Style::new().bg(Color::Red));
//     mask.add(0..=8, Style::new().modifier(Modifier::BOLD));
//
//     let result = vec![
//         (0..=3, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
//         (4..=7, Style::new().fg(Color::Green).modifier(Modifier::BOLD)),
//         (8..=8, Style::new().bg(Color::Red).modifier(Modifier::BOLD)),
//         (9..=10, Style::new().bg(Color::Red)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// #[test]
// fn mask_for_exists_range_should_be_merged() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=1, Style::new().fg(Color::Red).modifier(Modifier::ITALIC));
//     mask.add(0..=1, Style::new().fg(Color::Blue).bg(Color::Yellow).modifier(Modifier::BOLD));
//
//     let result = vec![(
//         0..=1,
//         Style::new().fg(Color::Blue).bg(Color::Yellow).modifier(Modifier::BOLD | Modifier::ITALIC),
//     )];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// #[test]
// fn remove_full_range() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=1, Style::new().fg(Color::Red));
//     mask.add(2..=4, Style::new().fg(Color::Blue));
//     mask.add(5..=6, Style::new().fg(Color::Green));
//
//     mask.remove_range(2..=4);
//
//     let result = vec![(0..=1, Style::new().fg(Color::Red)), (5..=6, Style::new().fg(Color::Green))];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// #[test]
// fn remove_mask() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=4, Style::new().fg(Color::Red));
//     mask.add(5..=7, Style::new().bg(Color::Blue));
//     mask.add(8..=10, Style::new().modifier(Modifier::BOLD));
//
//     mask.remove(1..=2);
//
//     let result = vec![
//         (0..=0, Style::new().fg(Color::Red)),
//         (3..=4, Style::new().fg(Color::Red)),
//         (5..=7, Style::new().bg(Color::Blue)),
//         (8..=10, Style::new().modifier(Modifier::BOLD)),
//     ];
//
//     assert_eq!(mask.into_vec(), result);
// }
//
// #[test]
// fn clear() {
//     let mut mask = Mask::new();
//
//     mask.add(.., Style::new().fg(Color::Red));
//     mask.clear();
//
//     assert_eq!(mask.into_vec(), vec![]);
// }
//
// #[test]
// fn range() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=1, Style::new().bg(Color::Red));
//     mask.add(2..=4, Style::new().bg(Color::Red));
//     mask.add(5..=5, Style::new().bg(Color::Green));
//     mask.add(6..=9, Style::new().bg(Color::Yellow));
//
//     let mask_for_range = mask.range(3..=7).collect::<Vec<_>>();
//     assert_eq!(
//         mask_for_range,
//         vec![
//             (3..=4, Style::new().bg(Color::Red)),
//             (5..=5, Style::new().bg(Color::Green)),
//             (6..=7, Style::new().bg(Color::Yellow))
//         ]
//     );
//
//     let mask_for_range = mask.range(3..=7).rev().collect::<Vec<_>>();
//     assert_eq!(
//         mask_for_range,
//         vec![
//             (6..=7, Style::new().bg(Color::Yellow)),
//             (5..=5, Style::new().bg(Color::Green)),
//             (3..=4, Style::new().bg(Color::Red))
//         ]
//     );
//
//     let mask_for_range = mask.range(12..).collect::<Vec<_>>();
//     assert_eq!(mask_for_range, vec![]);
// }
//
// #[test]
// fn shift_sub() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=3, Style::new().fg(Color::Red));
//     mask.add(4..=5, Style::new().modifier(Modifier::BOLD));
//     mask.add(10.., Style::new().bg(Color::Yellow));
//
//     mask.shift_sub(1.., 5);
//
//     assert_eq!(
//         mask.into_vec(),
//         vec![
//             (0..=0, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
//             (5..=usize::MAX - 5, Style::new().bg(Color::Yellow))
//         ]
//     );
// }
//
// #[test]
// fn shift_add() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=3, Style::new().fg(Color::Red));
//     mask.add(4..=7, Style::new().modifier(Modifier::BOLD));
//     mask.add(10.., Style::new().bg(Color::Yellow));
//
//     mask.shift_add(5.., 10);
//
//     assert_eq!(
//         mask.clone().into_vec(),
//         vec![
//             (0..=3, Style::new().fg(Color::Red)),
//             (4..=4, Style::new().modifier(Modifier::BOLD)),
//             (15..=17, Style::new().modifier(Modifier::BOLD)),
//             (20..=usize::MAX, Style::new().bg(Color::Yellow))
//         ]
//     );
// }
