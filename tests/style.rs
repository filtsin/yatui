use yatui::text::{Color, Modifier, Style};

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
