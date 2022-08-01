use yatui::text::{graphemes::Grapheme, Color, Modifier, Style, Text};

use pretty_assertions::assert_eq;

#[test]
fn create_borrowed_and_owned_strings() {
    let str: Text = "hello".into();
    let string: Text = "hello".to_owned().into();

    assert!(str.is_borrowed());
    assert!(string.is_owned());
}

#[test]
fn change_styles_mut() {
    let mut str: Text = "hello".into();

    assert_eq!(str.styles().iter().count(), 0);

    str.styles_mut().add_style_raw(0, 5, Style::new().fg(Color::Red));

    let mut styles = str.styles().iter();
    assert_eq!(styles.next().unwrap(), (0, 5, Style::new().fg(Color::Red)));
    assert_eq!(styles.next(), None)
}

#[test]
fn length() {
    let texts_len = [
        "hello",
        "!!!",
        "@!0123456789",
        "   spaces   ",
        "\n\t\r\n",
        "привет",
        "Löwe 老虎 Léopard",
        "❤️🧡💛💚💙💜",
        "y\u{0306}", // 2 code points, not (0, 'y̆')
        "y̆",
    ]
    .map(|v| Text::from(v).len());

    let len = [5, 3, 12, 12, 0, 6, 17, 11, 1, 1];

    assert_eq!(texts_len, len);
}

#[test]
fn add_style_raw_order() {
    let mut str: Text = "123456789".into();
    let len = str.len();

    let styles = str.styles_mut();

    // 5 is GREEN fg (and ITALIC and BOLD and BLUE bg)
    styles.add_style_raw(4, 4, Style::new().fg(Color::Green));

    // All text BOLD
    styles.add_style_raw(0, len - 1, Style::new().modifier(Modifier::BOLD));

    // 2345678 is ITALIC (and BOLD)
    styles.add_style_raw(1, len - 2, Style::new().modifier(Modifier::ITALIC));

    // 456 is BLUE bg (and ITALIC and BOLD and RED fg)
    styles.add_style_raw(3, len - 4, Style::new().bg(Color::Blue));

    // 34567 is RED fg (and ITALIC and BOLD)
    styles.add_style_raw(2, len - 3, Style::new().fg(Color::Red));

    let styles_result = vec![
        (0, len - 1, Style::new().modifier(Modifier::BOLD)),
        (1, len - 2, Style::new().modifier(Modifier::ITALIC)),
        (2, len - 3, Style::new().fg(Color::Red)),
        (3, len - 4, Style::new().bg(Color::Blue)),
        (4, len - 5, Style::new().fg(Color::Green)),
    ];

    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles, styles_result);
}

#[test]
fn styles_for_exists_range_should_be_replaced() {
    let mut str: Text = "123".into();

    let styles = str.styles_mut();
    styles.add_style_raw(0, 1, Style::new().fg(Color::Red));
    styles.add_style_raw(0, 1, Style::new().fg(Color::Blue));

    let styles_result = vec![(0, 1, Style::new().fg(Color::Blue))];
    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles, styles_result);
}

#[test]
fn add_style_order() {
    let mut str: Text = "Löwe 老虎".into();

    let (graphemes, styles) = str.parts();
    let graphemes: Vec<_> = graphemes.collect();

    println!("{:?}", graphemes);

    // we is RED fg (and BOLD and ITALIC)
    let from = graphemes.get(2).unwrap();
    let to = graphemes.get(3).unwrap();
    styles.add_style(from, to, Style::new().fg(Color::Red));

    // öwe 老 is ITALIC (and BOLD)
    let from = graphemes.iter().find(|v| v.data() == "ö").unwrap();
    let to = graphemes.iter().find(|v| v.data() == "老").unwrap();
    styles.add_style(from, to, Style::new().modifier(Modifier::ITALIC));

    // All text BOLD
    let from = graphemes.first().unwrap();
    let to = graphemes.last().unwrap();
    styles.add_style(from, to, Style::new().modifier(Modifier::BOLD));

    let styles_result = vec![
        (0, 11, Style::new().modifier(Modifier::BOLD)),
        (1, 8, Style::new().modifier(Modifier::ITALIC)),
        (3, 4, Style::new().fg(Color::Red)),
    ];

    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles, styles_result);
}

#[test]
fn remove_full_range() {
    let mut str: Text = "Hello".into();

    let styles = str.styles_mut();

    styles.add_style_raw(0, 1, Style::new().fg(Color::Red));
    styles.add_style_raw(0, 4, Style::new().fg(Color::Blue));
    styles.add_style_raw(2, 3, Style::new().fg(Color::Green));

    styles.remove_full(0, 4);

    let styles_result =
        vec![(0, 1, Style::new().fg(Color::Red)), (2, 3, Style::new().fg(Color::Green))];

    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles_result, styles);
}

#[test]
fn remove_styles() {
    let mut str: Text = "Hello".into();

    let styles = str.styles_mut();

    styles.add_style_raw(0, 4, Style::new().fg(Color::Red));
    styles.add_style_raw(1, 3, Style::new().bg(Color::Blue));
    styles.add_style_raw(2, 2, Style::new().modifier(Modifier::BOLD));

    styles.remove(1, 2);

    let styles_result = vec![
        (0, 0, Style::new().fg(Color::Red)),
        (3, 3, Style::new().bg(Color::Blue)),
        (3, 4, Style::new().fg(Color::Red)),
    ];

    let styles: Vec<_> = styles.iter().collect();

    println!("{:?}", styles);

    assert_eq!(styles_result, styles);
}

#[test]
fn clear() {
    let mut str: Text = "Hello".into();
    str.styles_mut().add_style_raw(0, 5, Style::new().fg(Color::Red));

    str.clear();

    assert_eq!(str.len(), 0);
    assert_eq!(str.styles().iter().count(), 0);
}

#[test]
fn push_str() {
    let mut str: Text = "Hello".into();
    str.push_str(" world");

    assert_eq!(str.as_str(), "Hello world");
    assert_eq!(str.len(), 11);
}

#[test]
fn remove() {
    let mut str: Text = "y\u{0306}es".into(); // y̆
    str.remove(0);

    assert_eq!(str.as_str(), "es");
    assert_eq!(str.len(), 2);

    let mut str: Text = "Löwe 老虎".into();
    str.remove(1);

    assert_eq!(str.as_str(), "Lwe 老虎");
    assert_eq!(str.len(), 8);

    let mut str: Text = "Hello".into();
    str.remove(4);

    assert_eq!(str.as_str(), "Hell");
    assert_eq!(str.len(), 4);

    let mut str: Text = "1".into();
    str.remove(100);

    assert_eq!(str.as_str(), "1");
    assert_eq!(str.len(), 1);
}

#[test]
fn replace_range() {
    let mut str: Text = "Hello".into();
    str.replace_range(.., "New content");

    assert_eq!(str.as_str(), "New content");
    assert_eq!(str.len(), 11);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(1..=5, "AAAAAAA");

    assert_eq!(str.as_str(), "L虎");
    assert_eq!(str.len(), 3);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(0..1, "");

    assert_eq!(str.as_str(), "öwe 老虎");
    assert_eq!(str.len(), 8);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(..3, "T");

    assert_eq!(str.as_str(), "we 老虎");
    assert_eq!(str.len(), 7);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(..=2, "T");

    assert_eq!(str.as_str(), "we 老虎");
    assert_eq!(str.len(), 7);
}

#[test]
#[should_panic]
fn replace_range_out_of_bound() {
    let mut str: Text = "Hello".into();
    str.replace_range(0..100, "New content");
}
