use yatui::text::{Color, Modifier, Style, TextStyle};

#[test]
fn change_styles_non_overlapping() {
    let mut styles = TextStyle::new();

    styles.add(..2, Style::new().fg(Color::Red));
    styles.add(2..5, Style::new().fg(Color::Blue));
    styles.add(5..=6, Style::new().fg(Color::Green));
    styles.add(7.., Style::new().fg(Color::Yellow));

    let result = vec![
        (0..=1, Style::new().fg(Color::Red)),
        (2..=4, Style::new().fg(Color::Blue)),
        (5..=6, Style::new().fg(Color::Green)),
        (7..=usize::MAX, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(styles.into_vec(), result);
}
//
// // ─────────────
// // x           y
// // ─────────────
// // x'          y'
// #[test]
// fn change_styles_overlapping_1() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=10, Style::new().fg(Color::Red));
//     styles.add(0..=10, Style::new().bg(Color::Blue));
//
//     let result = vec![(0..=10, Style::new().fg(Color::Red).bg(Color::Blue))];
//
//     assert_eq!(styles.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// // ───────────────
// // x'            y'
// #[test]
// fn change_styles_overlapping_2() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=10, Style::new().fg(Color::Red));
//     styles.add(0..=15, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (11..=15, Style::new().bg(Color::Blue)),
//     ];
//
//     assert_eq!(styles.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// //   ────────
// //   x'     y'
// #[test]
// fn change_styles_overlapping_3() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=10, Style::new().fg(Color::Red));
//     styles.add(3..=5, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red)),
//         (3..=5, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (6..=10, Style::new().bg(Color::Blue)),
//     ];
//
//     assert_eq!(styles.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// //    ──────────
// //    x'     y'
// #[test]
// fn change_styles_overlapping_4() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=10, Style::new().fg(Color::Red));
//     styles.add(3..=10, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red)),
//         (3..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//     ];
//
//     assert_eq!(styles.into_vec(), result);
// }
//
// // ─────────────
// // x           y
// //    ────────────
// //    x'         y'
// #[test]
// fn change_styles_overlapping_5() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=10, Style::new().fg(Color::Red));
//     styles.add(3..=15, Style::new().bg(Color::Blue));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red)),
//         (3..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (11..=15, Style::new().bg(Color::Blue)),
//     ];
//
//     assert_eq!(styles.into_vec(), result);
// }

//
//      ─────────────
//      x           y
// ──────
// x'   y'
#[test]
fn change_styles_overlapping_6() {
    let mut styles = TextStyle::new();

    styles.add(4..=10, Style::new().fg(Color::Red));
    styles.add(0..=4, Style::new().bg(Color::Blue));

    let result = vec![
        (0..=3, Style::new().bg(Color::Blue)),
        (4..=4, Style::new().bg(Color::Blue).fg(Color::Red)),
        (5..=10, Style::new().fg(Color::Red)),
    ];

    assert_eq!(styles.into_vec(), result);
}

//
//      ─────────────
//      x           y
// ──────────
// x'       y'
#[test]
fn change_styles_overlapping_7() {
    let mut styles = TextStyle::new();

    styles.add(4..=10, Style::new().fg(Color::Red));
    styles.add(0..=7, Style::new().bg(Color::Blue));

    let result = vec![
        (0..=3, Style::new().bg(Color::Blue)),
        (4..=7, Style::new().fg(Color::Red).bg(Color::Blue)),
        (8..=10, Style::new().fg(Color::Red)),
    ];

    assert_eq!(styles.into_vec(), result);
}

//
//      ─────────────
//      x           y
// ──────────────────
// x'               y'
#[test]
fn change_styles_overlapping_8() {
    let mut styles = TextStyle::new();

    styles.add(4..=10, Style::new().fg(Color::Red));
    styles.add(0..=10, Style::new().bg(Color::Blue));

    let result = vec![
        (0..=3, Style::new().bg(Color::Blue)),
        (4..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
    ];

    assert_eq!(styles.into_vec(), result);
}

//
//      ─────────────
//      x           y
// ──────────────────────
// x'                   y'
#[test]
fn change_styles_overlapping_9() {
    let mut styles = TextStyle::new();

    styles.add(4..=10, Style::new().fg(Color::Red));
    styles.add(0..=15, Style::new().bg(Color::Blue));

    let result = vec![
        (0..=3, Style::new().bg(Color::Blue)),
        (4..=10, Style::new().fg(Color::Red).bg(Color::Blue)),
        (11..=15, Style::new().bg(Color::Blue)),
    ];

    assert_eq!(styles.into_vec(), result);
}

//
//      ─────────────
//      x           y
//         ─────
//         x'  y'
#[test]
fn change_styles_overlapping_10() {
    let mut styles = TextStyle::new();

    styles.add(4..=10, Style::new().fg(Color::Red));
    styles.add(7..=9, Style::new().bg(Color::Blue));

    let result = vec![
        (4..=6, Style::new().fg(Color::Red)),
        (7..=9, Style::new().fg(Color::Red).bg(Color::Blue)),
        (10..=10, Style::new().fg(Color::Red)),
    ];

    assert_eq!(styles.into_vec(), result);
}

//  ───────     ─────
//  x''  y''    x'  y'
//  ───────────────────
//  x               y
#[test]
fn change_styles_overlapping_11() {
    let mut styles = TextStyle::new();

    styles.add(0..=3, Style::new().fg(Color::Red));
    styles.add(5..=10, Style::new().bg(Color::Red));
    styles.add(0..=12, Style::new().modifier(Modifier::BOLD));

    let result = vec![
        (0..=3, Style::new().fg(Color::Red).modifier(Modifier::BOLD)),
        (4..=4, Style::new().modifier(Modifier::BOLD)),
        (5..=10, Style::new().bg(Color::Red).modifier(Modifier::BOLD)),
        (11..=12, Style::new().modifier(Modifier::BOLD)),
    ];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn styles_for_exists_range_should_be_merged() {
    let mut styles = TextStyle::new();

    styles.add(0..=1, Style::new().fg(Color::Red).modifier(Modifier::ITALIC));
    styles.add(0..=1, Style::new().fg(Color::Blue).bg(Color::Yellow).modifier(Modifier::BOLD));

    let result = vec![(
        0..=1,
        Style::new().fg(Color::Blue).bg(Color::Yellow).modifier(Modifier::BOLD | Modifier::ITALIC),
    )];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn remove_full_range() {
    let mut styles = TextStyle::new();

    styles.add(0..=1, Style::new().fg(Color::Red));
    styles.add(2..=4, Style::new().fg(Color::Blue));
    styles.add(5..=6, Style::new().fg(Color::Green));

    styles.remove_range(2..=4);

    let result = vec![(0..=1, Style::new().fg(Color::Red)), (5..=6, Style::new().fg(Color::Green))];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn remove_styles() {
    let mut styles = TextStyle::new();

    styles.add(0..=4, Style::new().fg(Color::Red));
    styles.add(5..=7, Style::new().bg(Color::Blue));
    styles.add(8..=10, Style::new().modifier(Modifier::BOLD));

    styles.remove(1..=2);

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (3..=4, Style::new().fg(Color::Red)),
        (5..=7, Style::new().bg(Color::Blue)),
        (8..=10, Style::new().modifier(Modifier::BOLD)),
    ];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn clear() {
    let mut styles = TextStyle::new();

    styles.add(.., Style::new().fg(Color::Red));
    styles.clear();

    assert_eq!(styles.into_vec(), vec![]);
}

// #[test]
// fn ranges() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=1, Style::new().fg(Color::Red));
//     styles.add(2..=3, Style::new().fg(Color::Blue));
//
//     let result = vec![(0..=1, Style::new().fg(Color::Red)), (2..=3, Style::new().fg(Color::Blue))];
//     assert_eq!(styles.ranges().collect::<Vec<_>>(), result);
//
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=2, Style::new().fg(Color::Red));
//     styles.add(0..=3, Style::new().bg(Color::Blue));
//     styles.add(4..=5, Style::new().fg(Color::Green));
//
//     let result = vec![
//         (0..=2, Style::new().fg(Color::Red).bg(Color::Blue)),
//         (3..=3, Style::new().bg(Color::Blue)),
//         (4..=5, Style::new().fg(Color::Green)),
//     ];
//     assert_eq!(styles.ranges().collect::<Vec<_>>(), result);
//
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=2, Style::new().fg(Color::Red));
//     styles.add(1..=2, Style::new().bg(Color::Blue));
//     styles.add(4..=5, Style::new().fg(Color::Green));
//
//     let result = vec![
//         (0..=1, Style::new().fg(Color::Red)),
//         (2..=2, Style::new().bg(Color::Blue)),
//         (4..=5, Style::new().fg(Color::Green)),
//     ];
//     assert_eq!(styles.ranges().collect::<Vec<_>>(), result);
//
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=2, Style::new().fg(Color::Red));
//     styles.add(1..=3, Style::new().bg(Color::Blue));
//     styles.add(4..=5, Style::new().fg(Color::Green));
//
//     let result = vec![
//         (0..=1, Style::new().fg(Color::Red)),
//         (2..=3, Style::new().bg(Color::Blue)),
//         (4..=5, Style::new().fg(Color::Green)),
//     ];
//     assert_eq!(styles.ranges().collect::<Vec<_>>(), result);
// }
