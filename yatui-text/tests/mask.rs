use std::ops::RangeInclusive;

use yatui_text::{mask, Color, IdxRange, Mask, Modifier, Style};

fn mask_to_vec(mask: Mask) -> Vec<(RangeInclusive<usize>, Style)> {
    mask.into_iter().map(|(range, style)| (range.start..=range.end, style)).collect()
}

#[test]
fn index_mask() {
    let mask: Mask = [(0..=1, Style::new().fg(Color::Red))].into();

    assert_eq!(mask[0], mask[1]);
    assert_eq!(mask[0], Style::new().fg(Color::Red));
    assert_eq!(mask[2], Style::default());
}

#[test]
fn add_styles_not_overlapping() {
    let mask = mask_to_vec(mask!(
        ..2 => Style::new().fg(Color::Red),
        2..5 => Style::new().fg(Color::Blue),
        5..=6 => Style::new().fg(Color::Green),
        7.. => Style::new().fg(Color::Yellow)
    ));

    let result = vec![
        (0..=1, Style::new().fg(Color::Red)),
        (2..=4, Style::new().fg(Color::Blue)),
        (5..=6, Style::new().fg(Color::Green)),
        (7..=usize::MAX, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(mask, result);
}

//
// // ─────────────
// // x           y
// // ─────────────
// // x'          y'
// #[test]
// fn change_mask_overlapping_full() {
//     let mut mask = Mask::new();
//
//     mask.add(0..=10, Style::new().fg(Color::Red));
//     mask.add(0..=10, Style::new().bg(Color::Blue));
//
//     let result = vec![(0..=10, Style::new().fg(Color::Red).bg(Color::Blue))];
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
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
//     let mask: Vec<_> = mask.into_iter().collect();
//
//     assert_eq!(mask, result);
// }
