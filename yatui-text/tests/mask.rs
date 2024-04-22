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
        7.. => Style::new().fg(Color::Yellow),
    ));

    let result = vec![
        (0..=1, Style::new().fg(Color::Red)),
        (2..=4, Style::new().fg(Color::Blue)),
        (5..=6, Style::new().fg(Color::Green)),
        (7..=usize::MAX, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(mask, result);
}

// 1) x' < x
//
// i. y' = x
//    ─────────────
//    x           y
// ────
// x' y'
//
#[test]
fn mask_add_intersection_1i() {
    let mask = mask_to_vec(mask!(
        1..=2 => Style::new().fg(Color::Red),
        0..=1 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().bg(Color::Green)),
        (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
        (2..=2, Style::new().fg(Color::Red)),
    ];

    assert_eq!(mask, result);
}

// 1)
//
// ii. y' > x && y' < y:
//    ───────────
//    x         y
// ────────
// x'     y'
#[test]
fn mask_add_intersection_1ii() {
    let mask = mask_to_vec(mask!(
        1..=3 => Style::new().fg(Color::Red),
        0..=2 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().bg(Color::Green)),
        (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
        (3..=3, Style::new().fg(Color::Red)),
    ];

    assert_eq!(mask, result);
}

// 1)
//
// iii. y' = y:
//    ───────────
//    x         y
// ──────────────
// x'           y'
#[test]
fn mask_add_intersection_1iii() {
    let mask = mask_to_vec(mask!(
        1..=3 => Style::new().fg(Color::Red),
        0..=3 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().bg(Color::Green)),
        (1..=3, Style::new().fg(Color::Red).bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

// 1)
//
// iv. y' > y:
//    ───────────
//    x         y
// ──────────────────
// x'               y'
#[test]
fn mask_add_intersection_1iv() {
    let mask = mask_to_vec(mask!(
        1..=3 => Style::new().fg(Color::Red),
        0..=4 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().bg(Color::Green)),
        (1..=3, Style::new().fg(Color::Red).bg(Color::Green)),
        (4..=4, Style::new().bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

// 2) x' = x
//
// i. y' = x
//
// ──────────
// x        y
// •
// x' = y'
#[test]
fn mask_add_intersection_2i() {
    let mask = mask_to_vec(mask!(
        0..=2 => Style::new().fg(Color::Red),
        0..=0 => Style::new().bg(Color::Green)
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red).bg(Color::Green)),
        (1..=2, Style::new().fg(Color::Red)),
    ];

    assert_eq!(mask, result);
}

// 2)
//
// ii. y' < y:
// ──────────
// x        y
// ───────
// x'    y'
#[test]
fn mask_add_intersection_2ii() {
    let mask = mask_to_vec(mask!(
        0..=2 => Style::new().fg(Color::Red),
        0..=1 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=1, Style::new().fg(Color::Red).bg(Color::Green)),
        (2..=2, Style::new().fg(Color::Red)),
    ];

    assert_eq!(mask, result);
}

// 2)
//
// iii. y' = y:
// ──────────
// x        y
// ──────────
// x'       y'
#[test]
fn mask_add_intersection_2iii() {
    let mask = mask_to_vec(mask!(
        0..=1 => Style::new().fg(Color::Red),
        0..=1 => Style::new().bg(Color::Green),
    ));

    let result = vec![(0..=1, Style::new().fg(Color::Red).bg(Color::Green))];

    assert_eq!(mask, result);
}

// 2)
//
// iv. y' > y:
// ──────────
// x        y
// ─────────────
// x'          y'
#[test]
fn mask_add_intersection_2iv() {
    let mask = mask_to_vec(mask!(
        0..=1 => Style::new().fg(Color::Red),
        0..=2 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=1, Style::new().fg(Color::Red).bg(Color::Green)),
        (2..=2, Style::new().bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

// 3) x' > x && x' < y
//
// i. y' = x'
// ──────────
// x        y
//     •
//     x' = y'
#[test]
fn mask_add_intersection_3i() {
    let mask = mask_to_vec(mask!(
        0..=2 => Style::new().fg(Color::Red),
        1..=1 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
        (2..=2, Style::new().fg(Color::Red)),
    ];

    assert_eq!(mask, result);
}

//
// ii. y' < y
// ──────────
// x        y
//    ────
//    x' y'
#[test]
fn mask_add_intersection_3ii() {
    let mask = mask_to_vec(mask!(
        0..=3 => Style::new().fg(Color::Red),
        1..=2 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
        (3..=3, Style::new().fg(Color::Red)),
    ];

    assert_eq!(mask, result);
}

// 3)
//
// iii. y' = y
// ──────────
// x        y
//    ───────
//    x'    y'
#[test]
fn mask_add_intersection_3iii() {
    let mask = mask_to_vec(mask!(
        0..=2 => Style::new().fg(Color::Red),
        1..=2 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

// 3)
//
// iv. y' > y
// ──────────
// x        y
//    ──────────
//    x'       y'
#[test]
fn mask_add_intersection_3iv() {
    let mask = mask_to_vec(mask!(
        0..=2 => Style::new().fg(Color::Red),
        1..=3 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (1..=2, Style::new().fg(Color::Red).bg(Color::Green)),
        (3..=3, Style::new().bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

// 4) x' = y
//
// i. y' = y
//
// ──────────
// x        y
//          •
//          x' = y'
#[test]
fn mask_add_intersection_4i() {
    let mask = mask_to_vec(mask!(
        0..=1 => Style::new().fg(Color::Red),
        1..=1 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

// 4)
//
// ii. y' > y
// ──────────
// x        y
//          ──────────
//          x'       y'
#[test]
fn mask_add_intersection_4ii() {
    let mask = mask_to_vec(mask!(
        0..=1 => Style::new().fg(Color::Red),
        1..=2 => Style::new().bg(Color::Green),
    ));

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (1..=1, Style::new().fg(Color::Red).bg(Color::Green)),
        (2..=2, Style::new().bg(Color::Green)),
    ];

    assert_eq!(mask, result);
}

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
