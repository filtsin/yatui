use yatui::text::{Color, Modifier, Style, Text};

#[test]
fn create_borrowed_and_owned_strings() {
    let str: Text = "hello".into();
    let string: Text = "hello".to_owned().into();

    assert!(str.is_borrowed());
    assert!(string.is_owned());
}

#[test]
fn lines_columns_length() {
    let texts_len = [
        "hello",
        "\n\t\r\n",
        "привет",
        "Löwe 老虎 Léopard",
        "❤️🧡💛💚💙💜",
        "y\u{0306}", // 2 code points, not (0, 'y̆')
        "y̆",
        "𝄞",
        "text\nnew\r\nit is very big line\nnew line",
        "",
    ]
    .map(|v| {
        let text = Text::from(v);
        (text.lines(), text.columns(), text.len())
    });

    let len = [
        (1, 5, 5),
        (2, 0, 3),
        (1, 6, 6),
        (1, 17, 15),
        (1, 11, 6),
        (1, 1, 1),
        (1, 1, 1),
        (1, 1, 1),
        (4, 19, 37),
        (0, 0, 0),
    ];

    assert_eq!(texts_len, len);
}

#[test]
fn clear() {
    let mut str: Text = "Hello".into();
    str.styles_mut().add(0..=5, Style::new().fg(Color::Red));

    str.clear();

    assert_eq!((str.lines(), str.columns()), (0, 0));
    assert_eq!(str.styles().iter().count(), 1);
}

#[test]
fn clear_all() {
    let mut str: Text = "Hello".into();
    str.styles_mut().add(0..=5, Style::new().fg(Color::Red));

    str.clear_all();

    assert_eq!((str.lines(), str.columns()), (0, 0));
    assert_eq!(str.styles().iter().count(), 0);
}

#[test]
fn push_str() {
    let mut str: Text = "Hello".into();
    str.push_str(" world");

    assert_eq!(str.as_str(), "Hello world");
    assert_eq!(str.columns(), 11);
}

#[test]
fn remove() {
    let mut str: Text = "y\u{0306}es".into(); // y̆
    str.remove(0);

    assert_eq!(str.as_str(), "es");
    assert_eq!(str.columns(), 2);

    let mut str: Text = "Löwe 老虎".into();
    str.remove(1);

    assert_eq!(str.as_str(), "Lwe 老虎");
    assert_eq!(str.columns(), 8);

    let mut str: Text = "Hello".into();
    str.styles_mut().add(.., Style::new());
    str.remove(4);

    assert_eq!(str.as_str(), "Hell");
    assert_eq!(str.columns(), 4);
    assert_eq!(str.styles().iter().count(), 1);
}

#[test]
#[should_panic]
fn remove_string_out_of_bound() {
    let mut str: Text = "Text".into();
    str.remove(100);
}

#[test]
fn replace_range() {
    let mut str: Text = "Heööllo".into();
    str.replace_range(.., "New content");

    assert_eq!(str.as_str(), "New content");
    assert_eq!(str.columns(), 11);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(1..=5, "AAAAAAA");

    assert_eq!(str.as_str(), "LAAAAAAA虎");
    assert_eq!(str.columns(), 10);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(0..1, "");

    assert_eq!(str.as_str(), "öwe 老虎");
    assert_eq!(str.columns(), 8);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(..3, "T");

    assert_eq!(str.as_str(), "Te 老虎");
    assert_eq!(str.columns(), 7);

    let mut str: Text = "Löwe 老虎".into();
    str.replace_range(..=2, "T");

    assert_eq!(str.as_str(), "Te 老虎");
    assert_eq!(str.columns(), 7);

    let mut str: Text = "老Löwe 虎".into();
    str.styles_mut().add(.., Style::new());
    str.replace_range(1.., "T");

    assert_eq!(str.as_str(), "老T");
    assert_eq!(str.columns(), 3);
    assert_eq!(str.styles().iter().count(), 1);
}

#[test]
#[should_panic]
fn replace_range_out_of_bound() {
    let mut str: Text = "Hello".into();
    str.replace_range(0..100, "New content");
}

#[test]
fn replace_range_polite() {
    let mut str: Text = "Löwe 老虎 y\u{0306}!".into();
    str.styles_mut().add(1..2, Style::new().fg(Color::Red)); // ö
    str.styles_mut().add(5..7, Style::new().fg(Color::Blue)); // 老虎
    str.styles_mut().add(8..=8, Style::new().fg(Color::Yellow)); // y\u{0306}
    let mut str2 = str.clone();

    str.replace_range_polite(3..=5, " New text ");

    assert_eq!(str.as_str(), "Löw New text 虎 y\u{0306}!");
    assert_eq!(str.columns(), 18);

    let styles = vec![
        (1..=1, Style::new().fg(Color::Red)),
        (13..=13, Style::new().fg(Color::Blue)),
        (15..=15, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(str.styles().clone().into_vec(), styles);

    str2.replace_range_polite(3..=5, "1");
    assert_eq!(str2.as_str(), "Löw1虎 y\u{0306}!");
    assert_eq!(str2.columns(), 9);

    let styles = vec![
        (1..=1, Style::new().fg(Color::Red)),
        (4..=4, Style::new().fg(Color::Blue)),
        (6..=6, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(str2.styles().clone().into_vec(), styles);
}

#[test]
fn pop() {
    let mut str: Text = "He\u{0306}y".into();
    str.pop();
    assert_eq!(str.as_str(), "He\u{0306}");
    assert_eq!(str.columns(), 2);
    str.pop();
    assert_eq!(str.as_str(), "H");
    assert_eq!(str.columns(), 1);
    str.pop();
    assert_eq!(str.as_str(), "");
    assert_eq!(str.columns(), 0);
    str.pop();
    assert_eq!(str.as_str(), "");
    assert_eq!(str.columns(), 0);
}

#[test]
fn push() {
    let mut str: Text = "fo".into();
    str.push('o');
    str.push('老');
    str.push('🖉');

    assert_eq!(str.as_str(), "foo老🖉");
    assert_eq!(str.columns(), 6);
}

#[test]
fn modify() {
    let mut str: Text = "hello".into();

    str.modify(|string| {
        string.make_ascii_uppercase();
    });

    assert_eq!(str.as_str(), "HELLO");
    assert_eq!(str.columns(), 5);
    assert_eq!(str.lines(), 1);

    str.modify(|string| {
        *string = string.replace('E', "\n2345\n");
    });

    assert_eq!(str.as_str(), "H\n2345\nLLO");
    assert_eq!(str.columns(), 4);
    assert_eq!(str.lines(), 3);
}

#[test]
fn insert_str() {
    let mut str: Text = "老hello".into();

    str.insert_str(0, "foo");
    assert_eq!(str.as_str(), "foo老hello");

    str.insert_str(4, " ");
    assert_eq!(str.as_str(), "foo老 hello");

    str.insert_str(str.len(), "\nnew content");
    assert_eq!(str.as_str(), "foo老 hello\nnew content");
    assert_eq!(str.columns(), 11);
    assert_eq!(str.lines(), 2);

    let mut str: Text = "".into();
    str.insert_str(0, "hello");
    assert_eq!(str.as_str(), "hello");
}

#[test]
#[should_panic]
fn insert_str_out_of_bounds() {
    let mut str: Text = "hello".into();
    str.insert_str(100, "foo");
}

#[test]
fn retain() {
    let mut str: Text = "老y\u{0306}foл".into();

    str.retain(|_| true);
    assert_eq!(str.as_str(), "老y\u{0306}foл");

    str.retain(|l| l != "y\u{0306}");
    assert_eq!(str.as_str(), "老foл");

    str.retain(|_| false);
    assert_eq!(str.as_str(), "");
    assert_eq!(str.columns(), 0);
    assert_eq!(str.lines(), 0);

    let mut str: Text = "yy\u{0306}".into();
    str.retain(|c| c != "y");
    assert_eq!(str.as_str(), "y\u{0306}");

    let mut str: Text = "y\u{0306}y\u{0306}❤️🧡🧡🧡🧡🧡🧡🧡🧡".into();
    str.retain(|c| c != "❤️");
    assert_eq!(str.as_str(), "y\u{0306}y\u{0306}🧡🧡🧡🧡🧡🧡🧡🧡");

    str.retain(|c| c == "🧡");
    assert_eq!(str.as_str(), "🧡🧡🧡🧡🧡🧡🧡🧡");
}

#[test]
fn truncate() {
    let mut str: Text = "y\u{0306}hello\nfoo老".into();

    str.truncate(7);

    assert_eq!(str.as_str(), "y\u{0306}hello\n");

    str.truncate(100);
    assert_eq!(str.as_str(), "y\u{0306}hello\n");

    str.truncate(0);
    assert_eq!(str.as_str(), "");
}

#[test]
fn split_off_polite() {
    let mut str: Text = "hey\u{0306}it\n is me".into();
    str.styles_mut().add(0..2, Style::new().fg(Color::Red));
    str.styles_mut().add(2..=3, Style::new().fg(Color::Blue));
    str.styles_mut().add(4..=6, Style::new().fg(Color::Green));
    str.styles_mut().add(7.., Style::new().fg(Color::Yellow));

    let str2 = str.split_off_polite(3);

    assert_eq!(str.as_str(), "hey\u{0306}");
    assert_eq!(str2.as_str(), "it\n is me");

    assert_eq!(str.columns(), 3);
    assert_eq!(str.lines(), 1);
    assert_eq!(str2.columns(), 6);
    assert_eq!(str2.lines(), 2);

    let styles_left = vec![
        (0..=1, Style::new().fg(Color::Red)),
        (2..=3, Style::new().fg(Color::Blue)),
        (4..=6, Style::new().fg(Color::Green)),
        (7..=std::usize::MAX, Style::new().fg(Color::Yellow)),
    ];

    let styles_right = vec![
        (0..=0, Style::new().fg(Color::Blue)),
        (1..=3, Style::new().fg(Color::Green)),
        (4..=std::usize::MAX - 3, Style::new().fg(Color::Yellow)),
    ];

    assert_eq!(str.styles().clone().into_vec(), styles_left);
    assert_eq!(str2.styles().clone().into_vec(), styles_right);
}

#[test]
#[should_panic]
fn split_off_polite_out_of_bounds() {
    let mut str: Text = "hey".into();
    str.split_off_polite(100);
}

#[test]
fn truncate_lines() {
    let mut str: Text = "hello\nnew\r\nworld\n".into();
    assert_eq!(str.lines(), 3);

    str.truncate_lines(4);
    assert_eq!(str.as_str(), "hello\nnew\r\nworld\n");
    assert_eq!(str.lines(), 3);

    str.truncate_lines(3);
    assert_eq!(str.as_str(), "hello\nnew\r\nworld");
    assert_eq!(str.lines(), 3);

    str.truncate_lines(2);
    assert_eq!(str.as_str(), "hello\nnew");
    assert_eq!(str.lines(), 2);

    let mut str: Text = "hello\nnew".into();

    str.truncate_lines(1);
    assert_eq!(str.as_str(), "hello");
    assert_eq!(str.lines(), 1);

    str.truncate_lines(0);
    assert_eq!(str.as_str(), "");
    assert_eq!(str.lines(), 0);
}

#[test]
fn truncate_columns() {
    let mut str: Text = "333\n4444\r\n4444\r\n55555\r\n1".into();
    assert_eq!(str.columns(), 5);

    str.truncate_columns(6);
    assert_eq!(str.as_str(), "333\n4444\r\n4444\r\n55555\r\n1");
    assert_eq!(str.columns(), 5);

    str.truncate_columns(5);
    assert_eq!(str.as_str(), "333\n4444\r\n4444\r\n55555\r\n1");
    assert_eq!(str.columns(), 5);

    str.truncate_columns(4);
    assert_eq!(str.as_str(), "333\n4444\r\n4444\r\n5555\r\n1");
    assert_eq!(str.columns(), 4);

    str.truncate_columns(3);
    assert_eq!(str.as_str(), "333\n444\r\n444\r\n555\r\n1");
    assert_eq!(str.columns(), 3);

    str.truncate_columns(2);
    assert_eq!(str.as_str(), "33\n44\r\n44\r\n55\r\n1");
    assert_eq!(str.columns(), 2);

    str.truncate_columns(1);
    assert_eq!(str.as_str(), "3\n4\r\n4\r\n5\r\n1");
    assert_eq!(str.columns(), 1);
    assert_eq!(str.lines(), 5);

    str.truncate_columns(0);
    assert_eq!(str.as_str(), "\n\r\n\r\n\r\n");
    assert_eq!(str.columns(), 0);
    assert_eq!(str.lines(), 4);
}

#[test]
fn index() {
    let str: Text = "老y\u{0306}hп\ntext\r\nf".into();

    assert_eq!(&str[0..1], "老");
    assert_eq!(&str[0..5], "老y\u{0306}hп\n");
    assert_eq!(&str[9..10], "\r\n");
    assert_eq!(&str[0..=10], str.as_str());
}

#[test]
#[should_panic]
fn index_out_of_bounds() {
    let str: Text = "hello".into();
    let _ = &str[100..200];
}
