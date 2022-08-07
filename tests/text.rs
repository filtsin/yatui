use yatui::text::{Color, Grapheme, Modifier, Style, Text};

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

    str.styles_mut().add(0, 5, Style::new().fg(Color::Red));

    let mut styles = str.styles().iter();
    assert_eq!(styles.next().unwrap(), (0, 5, Style::new().fg(Color::Red)));
    assert_eq!(styles.next(), None)
}

#[test]
fn length() {
    let texts_len = [
        "hello",
        "\n\t\r\n",
        "привет",
        "Löwe 老虎 Léopard",
        "❤️🧡💛💚💙💜",
        "y\u{0306}", // 2 code points, not (0, 'y̆')
        "y̆",
        "text\nnew\r\nit is very big line\nnew line",
    ]
    .map(|v| {
        let text = Text::from(v);
        (text.lines(), text.columns())
    });

    let len = [(1, 5), (2, 0), (1, 6), (1, 17), (1, 11), (1, 1), (1, 1), (4, 19)];

    assert_eq!(texts_len, len);
}

#[test]
fn add_order() {
    let mut str: Text = "123456789".into();

    let styles = str.styles_mut();

    // 5 is GREEN fg (and ITALIC and BOLD and BLUE bg)
    styles.add(4, 4, Style::new().fg(Color::Green));

    // All text BOLD
    styles.add(0, 8, Style::new().modifier(Modifier::BOLD));

    // 2345678 is ITALIC (and BOLD)
    styles.add(1, 7, Style::new().modifier(Modifier::ITALIC));

    // 456 is BLUE bg (and ITALIC and BOLD and RED fg)
    styles.add(2, 6, Style::new().bg(Color::Blue));

    // 34567 is RED fg (and ITALIC and BOLD)
    styles.add(3, 5, Style::new().fg(Color::Red));

    let styles_result = vec![
        (0, 8, Style::new().modifier(Modifier::BOLD)),
        (1, 7, Style::new().modifier(Modifier::ITALIC)),
        (2, 6, Style::new().bg(Color::Blue)),
        (3, 5, Style::new().fg(Color::Red)),
        (4, 4, Style::new().fg(Color::Green)),
    ];

    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles, styles_result);
}

#[test]
fn styles_for_exists_range_should_be_replaced() {
    let mut str: Text = "123".into();

    let styles = str.styles_mut();
    styles.add(0, 1, Style::new().fg(Color::Red));
    styles.add(0, 1, Style::new().fg(Color::Blue));

    let styles_result = vec![(0, 1, Style::new().fg(Color::Blue))];
    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles, styles_result);
}

#[test]
fn add_with_grapheme() {
    let mut str: Text = "Löwe 老虎".into();

    let (graphemes, styles) = str.parts();
    let graphemes: Vec<_> = graphemes.collect();

    println!("{:?}", graphemes);

    // we is RED fg (and BOLD and ITALIC)
    let from = graphemes.get(2).unwrap();
    let to = graphemes.get(3).unwrap();
    styles.add_with_grapheme(from, to, Style::new().fg(Color::Red));

    // öwe 老 is ITALIC (and BOLD)
    let from = graphemes.iter().find(|v| v.data() == "ö").unwrap();
    let to = graphemes.iter().find(|v| v.data() == "老").unwrap();
    styles.add_with_grapheme(from, to, Style::new().modifier(Modifier::ITALIC));

    // All text BOLD
    let from = graphemes.first().unwrap();
    let to = graphemes.last().unwrap();
    styles.add_with_grapheme(from, to, Style::new().modifier(Modifier::BOLD));

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

    styles.add(0, 1, Style::new().fg(Color::Red));
    styles.add(0, 4, Style::new().fg(Color::Blue));
    styles.add(2, 3, Style::new().fg(Color::Green));

    styles.remove_range(0, 4);

    let styles_result =
        vec![(0, 1, Style::new().fg(Color::Red)), (2, 3, Style::new().fg(Color::Green))];

    let styles: Vec<_> = styles.iter().collect();

    assert_eq!(styles_result, styles);
}

#[test]
fn remove_styles() {
    let mut str: Text = "Hello".into();

    let styles = str.styles_mut();

    styles.add(0, 4, Style::new().fg(Color::Red));
    styles.add(1, 3, Style::new().bg(Color::Blue));
    styles.add(2, 2, Style::new().modifier(Modifier::BOLD));

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
    str.styles_mut().add(0, 5, Style::new().fg(Color::Red));

    str.clear();

    assert_eq!((str.lines(), str.columns()), (0, 0));
    assert_eq!(str.styles().iter().count(), 0);
}

#[test]
fn push_str() {
    let mut str: Text = "Hello".into();
    str.push_str(" world");

    assert_eq!(str.as_ref(), "Hello world");
    assert_eq!(str.columns(), 11);
}

#[test]
fn remove() {
    let mut str: Text = "y\u{0306}es".into(); // y̆
    str.remove(0);

    assert_eq!(str.as_ref(), "es");
    assert_eq!(str.columns(), 2);

    let mut str: Text = "Löwe 老虎".into();
    str.remove(1);

    assert_eq!(str.as_ref(), "Lwe 老虎");
    assert_eq!(str.columns(), 8);

    let mut str: Text = "Hello".into();
    str.remove(4);

    assert_eq!(str.as_ref(), "Hell");
    assert_eq!(str.columns(), 4);

    let mut str: Text = "He\u{0306}llo".into();
    str.styles_mut().add(0, 6, Style::default().bg(Color::Blue));
    str.styles_mut().add(4, 5, Style::default().bg(Color::Red));
    str.styles_mut().add(4, 4, Style::default().bg(Color::Green));
    str.styles_mut().add(1, 3, Style::default().bg(Color::White));
    str.remove(1);

    let styles_result = vec![
        (0, 0, Style::default().bg(Color::Blue)),
        (1, 1, Style::default().bg(Color::Green)),
        (1, 2, Style::default().bg(Color::Red)),
        (1, 3, Style::default().bg(Color::Blue)),
    ];

    let styles: Vec<_> = str.styles().iter().collect();

    assert_eq!(styles, styles_result);
}

#[test]
fn replace_range() {
    let mut str: Text = "Heööllo".into();
    str.replace_range(.., "New content");

    assert_eq!(str.as_ref(), "New content");
    assert_eq!(str.columns(), 11);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(1..=5, "AAAAAAA");

    assert_eq!(str.as_ref(), "LAAAAAAA虎");
    assert_eq!(str.columns(), 10);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(0..1, "");

    assert_eq!(str.as_ref(), "öwe 老虎");
    assert_eq!(str.columns(), 8);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(..3, "T");

    assert_eq!(str.as_ref(), "Te 老虎");
    assert_eq!(str.columns(), 7);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(..=2, "T");

    assert_eq!(str.as_ref(), "Te 老虎");
    assert_eq!(str.columns(), 7);
}

#[test]
#[should_panic]
fn replace_range_out_of_bound() {
    let mut str: Text = "Hello".into();
    str.replace_range(0..100, "New content");
}
