use yatui::terminal::{
    buffer::Buffer, character::Character, cursor::Cursor, region::Region, size::Size,
};

use pretty_assertions::assert_eq;

#[test]
fn creation() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));

    assert_eq!(mapped_buffer.region(), Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
}

#[test]
fn remap() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
    let mapped_buffer = mapped_buffer.map(Region::new(Cursor::new(0, 0), Cursor::new(1, 1)));

    assert_eq!(mapped_buffer.region(), Region::new(Cursor::new(1, 1), Cursor::new(2, 2)));
}

#[test]
#[should_panic]
fn remap_overflow_should_panic() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
    let _ = mapped_buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
}

#[test]
fn write_character() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));

    mapped_buffer.write_character('0', Cursor::new(0, 0));
    mapped_buffer.write_character('1', Cursor::new(1, 0));
    mapped_buffer.write_character('2', Cursor::new(2, 0));
    mapped_buffer.write_character('3', Cursor::new(0, 1));
    mapped_buffer.write_character('4', Cursor::new(1, 1));
    mapped_buffer.write_character('5', Cursor::new(2, 1));
    mapped_buffer.write_character('6', Cursor::new(0, 2));
    mapped_buffer.write_character('7', Cursor::new(1, 2));
    mapped_buffer.write_character('8', Cursor::new(2, 2));

    #[rustfmt::skip]
    let s = vec![
        "     ",
        " 012 ",
        " 345 ",
        " 678 ",
        "     "
    ];

    assert_eq!(buffer, Buffer::from(s));
}

#[test]
#[should_panic]
fn write_character_overflow_should_panic() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(1, 1)));

    mapped_buffer.write_character('0', Cursor::new(1, 1));
}

#[test]
fn try_write_character() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));

    mapped_buffer.try_write_character('0', Cursor::new(0, 0)).unwrap();
    mapped_buffer.try_write_character('1', Cursor::new(1, 0)).unwrap();
    mapped_buffer.try_write_character('2', Cursor::new(2, 0)).unwrap();
    mapped_buffer.try_write_character('3', Cursor::new(0, 1)).unwrap();
    mapped_buffer.try_write_character('4', Cursor::new(1, 1)).unwrap();
    mapped_buffer.try_write_character('5', Cursor::new(2, 1)).unwrap();
    mapped_buffer.try_write_character('6', Cursor::new(0, 2)).unwrap();
    mapped_buffer.try_write_character('7', Cursor::new(1, 2)).unwrap();
    mapped_buffer.try_write_character('8', Cursor::new(2, 2)).unwrap();

    #[rustfmt::skip]
    let s = vec![
        "     ",
        " 012 ",
        " 345 ",
        " 678 ",
        "     "
    ];

    assert_eq!(buffer, Buffer::from(s));
}

#[test]
fn try_write_character_overflow_should_return_err() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(1, 1)));

    let result = mapped_buffer.try_write_character('0', Cursor::new(1, 1));
    result.err().unwrap();
}

#[test]
fn fill() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
    mapped_buffer.fill('0');

    #[rustfmt::skip]
    let s = vec![
        "     ",
        " 000 ",
        " 000 ",
        " 000 ",
        "     "
    ];

    assert_eq!(buffer, Buffer::from(s));
}

#[test]
fn clear() {
    let mut buffer = Buffer::new(Size::new(5, 5));
    buffer.full_map().fill('0');

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
    mapped_buffer.clear();

    #[rustfmt::skip]
    let s = vec![
        "00000",
        "0   0",
        "0   0",
        "0   0",
        "00000"
    ];

    assert_eq!(buffer, Buffer::from(s));
}

#[test]
fn write_line() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
    mapped_buffer.write_line("01", 1);

    #[rustfmt::skip]
    let s = vec![
        "     ",
        "     ",
        " 01  ",
        "     ",
        "     "
    ];

    assert_eq!(buffer, Buffer::from(s));
}

#[test]
#[should_panic]
fn write_line_overflow_should_panic() {
    let mut buffer = Buffer::new(Size::new(5, 5));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(3, 3)));
    mapped_buffer.write_line("0123", 1);
}

#[test]
fn get() {
    let mut buffer = Buffer::new(Size::new(3, 3));

    let mut mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(1, 1)));
    mapped_buffer.write_character('0', Cursor::new(0, 0));

    let character = mapped_buffer.get(Cursor::new(0, 0));

    assert_eq!(*character, Character::from('0'));
}

#[test]
#[should_panic]
fn get_overflow_should_panic() {
    let mut buffer = Buffer::new(Size::new(3, 3));

    let mapped_buffer = buffer.map(Region::new(Cursor::new(1, 1), Cursor::new(1, 1)));

    mapped_buffer.get(Cursor::new(1, 1));
}
