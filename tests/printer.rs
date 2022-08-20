use yatui::{
    backend::Raw,
    terminal::{Cursor, Printer, Region},
    text::{Color, Modifier, Style, Text},
};

#[test]
fn map() {
    let mut backend = Raw::new(5, 5);

    let mut printer = Printer::new(&mut backend);
    assert_eq!(printer.mapped_region(), Region::new(Cursor::new(0, 0), Cursor::new(4, 4)));

    let mut printer2 = printer.try_map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3))).unwrap();
    assert_eq!(printer2.mapped_region(), Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));

    let mut printer3 = printer2.try_map(Region::new(Cursor::new(1, 1), Cursor::new(2, 2))).unwrap();
    assert_eq!(printer3.mapped_region(), Region::new(Cursor::new(2, 2), Cursor::new(3, 3)));

    let printer4 = printer3.try_map(Region::new(Cursor::new(0, 0), Cursor::new(1, 0))).unwrap();
    assert_eq!(printer4.mapped_region(), Region::new(Cursor::new(2, 2), Cursor::new(3, 2)));

    let printer = printer.try_map(Region::new(Cursor::new(3, 3), Cursor::new(5, 5)));
    assert!(printer.is_none());
}

#[test]
fn map_line() {
    let mut backend = Raw::new(5, 5);
    let mut printer = Printer::new(&mut backend);

    let line0 = printer.map_line(0);
    assert_eq!(line0.mapped_region(), Region::new(Cursor::new(0, 0), Cursor::new(4, 0)));

    let line1 = printer.map_line(1);
    assert_eq!(line1.mapped_region(), Region::new(Cursor::new(0, 1), Cursor::new(4, 1)));

    let line2 = printer.map_line(2);
    assert_eq!(line2.mapped_region(), Region::new(Cursor::new(0, 2), Cursor::new(4, 2)));

    let line3 = printer.map_line(3);
    assert_eq!(line3.mapped_region(), Region::new(Cursor::new(0, 3), Cursor::new(4, 3)));

    let line4 = printer.map_line(4);
    assert_eq!(line4.mapped_region(), Region::new(Cursor::new(0, 4), Cursor::new(4, 4)));
}

#[test]
fn map_first_line() {
    let mut backend = Raw::new(5, 4);
    let mut printer = Printer::new(&mut backend);

    let region1 = printer.map_line(0).mapped_region();
    let region2 = printer.map_first_line().mapped_region();

    assert_eq!(region1, region2);
}

#[test]
fn map_last_line() {
    let mut backend = Raw::new(5, 4);
    let mut printer = Printer::new(&mut backend);

    let region1 = printer.map_line(3).mapped_region();
    let region2 = printer.map_last_line().mapped_region();

    assert_eq!(region1, region2);
}

#[test]
fn map_first_and_last_line_with_only_one_line() {
    let mut backend = Raw::new(5, 5);
    let mut printer = Printer::new(&mut backend);

    let mut printer = printer.map_first_line();

    let region1 = printer.map_first_line().mapped_region();
    let region2 = printer.map_last_line().mapped_region();

    assert_eq!(region1, region2);
}

#[test]
#[should_panic]
fn map_line_out_of_bounds() {
    let mut backend = Raw::new(5, 5);
    let mut printer = Printer::new(&mut backend);
    printer.map_line(100);
}

#[test]
fn map_column() {
    let mut backend = Raw::new(5, 5);
    let mut printer = Printer::new(&mut backend);

    let line0 = printer.map_column(0);
    assert_eq!(line0.mapped_region(), Region::new(Cursor::new(0, 0), Cursor::new(0, 4)));

    let line1 = printer.map_column(1);
    assert_eq!(line1.mapped_region(), Region::new(Cursor::new(1, 0), Cursor::new(1, 4)));

    let line2 = printer.map_column(2);
    assert_eq!(line2.mapped_region(), Region::new(Cursor::new(2, 0), Cursor::new(2, 4)));

    let line3 = printer.map_column(3);
    assert_eq!(line3.mapped_region(), Region::new(Cursor::new(3, 0), Cursor::new(3, 4)));

    let line4 = printer.map_column(4);
    assert_eq!(line4.mapped_region(), Region::new(Cursor::new(4, 0), Cursor::new(4, 4)));
}

#[test]
fn map_first_column() {
    let mut backend = Raw::new(5, 4);
    let mut printer = Printer::new(&mut backend);

    let region1 = printer.map_column(0).mapped_region();
    let region2 = printer.map_first_column().mapped_region();

    assert_eq!(region1, region2);
}

#[test]
fn map_last_column() {
    let mut backend = Raw::new(5, 4);
    let mut printer = Printer::new(&mut backend);

    let region1 = printer.map_column(4).mapped_region();
    let region2 = printer.map_last_column().mapped_region();

    assert_eq!(region1, region2);
}

#[test]
fn map_first_and_last_column_with_only_one_line() {
    let mut backend = Raw::new(5, 5);
    let mut printer = Printer::new(&mut backend);

    let mut printer = printer.map_first_column();

    let region1 = printer.map_first_column().mapped_region();
    let region2 = printer.map_last_column().mapped_region();

    assert_eq!(region1, region2);
}

#[test]
#[should_panic]
fn map_column_out_of_bounds() {
    let mut backend = Raw::new(5, 5);
    let mut printer = Printer::new(&mut backend);
    printer.map_column(100);
}

// #[test]
// fn padding() {
//     let mut backend = Raw::new(5, 5);
//     let mut printer = Printer::new(&mut backend);
//
//     let mut printer2 = printer.padding(1);
//     assert_eq!(printer2.mapped_region(), Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
//
//     let region = printer2.padding(1).mapped_region();
//     assert_eq!(printer.padding(2).mapped_region(), region);
//
//     let region = printer.padding(0).mapped_region();
//     assert_eq!(printer.mapped_region(), region);
//
//     let mut backend = Raw::new(2, 2);
//     let mut printer = Printer::new(&mut backend);
//     printer.padding(1);
// }
//
// #[test]
// #[should_panic]
// fn padding_out_of_bounds() {
//     let mut backend = Raw::new(5, 5);
//     let mut printer = Printer::new(&mut backend);
//     printer.padding(3);
// }

// #[test]
// fn write_text_without_styles() {
//     let mut backend = Raw::new(5, 4);
//     let mut printer = Printer::new(&mut backend);
//
//     let text: Text = "hello world".into();
//     printer.write((0, 0), &text);
//
//     #[rustfmt::skip]
//     let lines = vec![
//         "hello",
//         " worl",
//         "d    ",
//         "     "
//     ];
//
//     assert_eq!(backend.lines_to_vec(), lines);
// }
//
// #[test]
// fn write_text_with_styles() {
//     let mut backend = Raw::new(5, 4);
//     let mut printer = Printer::new(&mut backend);
//
//     let mut text: Text = "hello world".into();
//     text.styles_mut().add(0..=1, Style::new().fg(Color::Red));
//     text.styles_mut().add(0..=2, Style::new().bg(Color::Green));
//     text.styles_mut().add(0..=3, Style::new().modifier(Modifier::BOLD));
//     text.styles_mut().add(1..=2, Style::new().fg(Color::Yellow));
//     text.styles_mut().add(4..=6, Style::new().fg(Color::Black));
//     text.styles_mut().add(.., Style::new().modifier(Modifier::ITALIC));
//
//     printer.write((0, 0), &text);
//
//     #[rustfmt::skip]
//     let lines = vec![
//         "hello",
//         " worl",
//         "d    ",
//         "     "
//     ];
//
//     assert_eq!(backend.lines_to_vec(), lines);
// }
