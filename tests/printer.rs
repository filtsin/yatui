use yatui::{
    backend::Raw,
    terminal::{Cursor, Printer, Region},
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
