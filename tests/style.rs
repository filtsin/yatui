use yatui::mask::{Color, Modifier, Style};

use pretty_assertions::assert_eq;

#[test]
fn create_empty_style() {
    let style = Style::new();

    assert_eq!(style.get_fg(), None);
    assert_eq!(style.get_bg(), None);
    assert_eq!(style.get_modifier(), Modifier::default());
}

#[test]
fn style_change_fg() {
    let style = Style::new().fg(Color::Black);

    assert_eq!(style.get_fg(), Some(Color::Black));
}

#[test]
fn style_change_bg() {
    let style = Style::new().bg(Color::Black);

    assert_eq!(style.get_bg(), Some(Color::Black));
}

#[test]
fn style_change_modifier() {
    let style = Style::new().modifier(Modifier::BOLD);

    assert_eq!(style.get_modifier(), Modifier::BOLD);
}

#[test]
fn style_clear_fg() {
    let style = Style::new().fg(Color::Black).clear_fg();

    assert_eq!(style.get_fg(), None);
}

#[test]
fn style_clear_bg() {
    let style = Style::new().bg(Color::Black).clear_bg();

    assert_eq!(style.get_bg(), None);
}

#[test]
fn style_clear_modifier() {
    let style = Style::new().modifier(Modifier::BOLD).clear_modifier();

    assert_eq!(style.get_modifier(), Modifier::default());
}

#[test]
fn style_merge() {
    let style1 = Style::new().fg(Color::Red).bg(Color::Green).modifier(Modifier::BOLD);
    let style2 = Style::new().fg(Color::Blue).bg(Color::Yellow).modifier(Modifier::ITALIC);

    assert_eq!(
        style1.merge(style2),
        Style::new().fg(Color::Blue).bg(Color::Yellow).modifier(Modifier::BOLD | Modifier::ITALIC)
    );
}
