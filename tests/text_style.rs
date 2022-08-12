use yatui::text::{Color, Modifier, Style, TextStyle};

#[test]
fn change_styles_mut() {
    let mut styles = TextStyle::new();

    styles.add(..2, Style::new().fg(Color::Red));
    styles.add(.., Style::new().fg(Color::White));
    styles.add(2..3, Style::new().fg(Color::Blue));
    styles.add(3..=4, Style::new().fg(Color::Green));
    styles.add(5.., Style::new().fg(Color::Yellow));

    let result = vec![
        (0..=1, Style::new().fg(Color::Red)),
        (0..=usize::MAX, Style::new().fg(Color::White)),
        (2..=2, Style::new().fg(Color::Blue)),
        (3..=4, Style::new().fg(Color::Green)),
        (5..=usize::MAX, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn add_order() {
    let mut styles = TextStyle::new();

    // 5 is GREEN fg (and ITALIC and BOLD and BLUE bg)
    styles.add(4..=4, Style::new().fg(Color::Green));

    // All text BOLD
    styles.add(0..=8, Style::new().modifier(Modifier::BOLD));

    // 2345678 is ITALIC (and BOLD)
    styles.add(1..=7, Style::new().modifier(Modifier::ITALIC));

    // 456 is BLUE bg (and ITALIC and BOLD and RED fg)
    styles.add(2..=6, Style::new().bg(Color::Blue));

    // 34567 is RED fg (and ITALIC and BOLD)
    styles.add(3..=5, Style::new().fg(Color::Red));

    let result = vec![
        (0..=8, Style::new().modifier(Modifier::BOLD)),
        (1..=7, Style::new().modifier(Modifier::ITALIC)),
        (2..=6, Style::new().bg(Color::Blue)),
        (3..=5, Style::new().fg(Color::Red)),
        (4..=4, Style::new().fg(Color::Green)),
    ];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn styles_for_exists_range_should_be_replaced() {
    let mut styles = TextStyle::new();

    styles.add(0..=1, Style::new().fg(Color::Red));
    styles.add(0..=1, Style::new().fg(Color::Blue));

    let result = vec![(0..=1, Style::new().fg(Color::Blue))];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn remove_full_range() {
    let mut styles = TextStyle::new();

    styles.add(0..=1, Style::new().fg(Color::Red));
    styles.add(0..=4, Style::new().fg(Color::Blue));
    styles.add(2..=3, Style::new().fg(Color::Green));

    styles.remove_range(0..=4);

    let result = vec![(0..=1, Style::new().fg(Color::Red)), (2..=3, Style::new().fg(Color::Green))];

    assert_eq!(styles.into_vec(), result);
}

#[test]
fn remove_styles() {
    let mut styles = TextStyle::new();

    styles.add(0..=4, Style::new().fg(Color::Red));
    styles.add(1..=3, Style::new().bg(Color::Blue));
    styles.add(2..=2, Style::new().modifier(Modifier::BOLD));

    styles.remove(1..=2);

    let result = vec![
        (0..=0, Style::new().fg(Color::Red)),
        (3..=3, Style::new().bg(Color::Blue)),
        (3..=4, Style::new().fg(Color::Red)),
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
// fn positive_shift() {
//     let mut styles = TextStyle::new();
//
//     styles.add(..=4, Style::new().bg(Color::Red));
//     styles.add(3..=6, Style::new().bg(Color::Yellow));
//     styles.add(6..12, Style::new().bg(Color::Black));
//     styles.add(8..10, Style::new().bg(Color::Green));
//     styles.add(15.., Style::new().bg(Color::White));
//
//     styles.positive_shift(7, 10);
//
//     let result = vec![
//         (0..=4, Style::new().bg(Color::Red)),
//         (3..=6, Style::new().bg(Color::Yellow)),
//         (6..=11, Style::new().bg(Color::Black)),
//         (18..=19, Style::new().bg(Color::Green)),
//         (25..=usize::MAX, Style::new().bg(Color::White)),
//     ];
//
//     assert_eq!(styles.into_vec(), result);
// }
//
// #[test]
// fn negative_shift() {
//     let mut styles = TextStyle::new();
//
//     styles.add(0..=0, Style::new().bg(Color::Blue));
//     styles.add(..=4, Style::new().bg(Color::Red));
//     styles.add(3..=6, Style::new().bg(Color::Yellow));
//     styles.add(6..12, Style::new().bg(Color::Black));
//     styles.add(8..10, Style::new().bg(Color::Green));
//     styles.add(15.., Style::new().bg(Color::White));
//
//     styles.negative_shift(7, 10);
//
//     let result = vec![
//         (0..=0, Style::new().bg(Color::Green)),
//         (0..=4, Style::new().bg(Color::Red)),
//         (3..=6, Style::new().bg(Color::Yellow)),
//         (5..=usize::MAX - 10, Style::new().bg(Color::White)),
//         (6..=11, Style::new().bg(Color::Black)),
//     ];
//
//     assert_eq!(styles.into_vec(), result);
// }
