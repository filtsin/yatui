pub mod example;
/// Widget trait
pub mod style;

use style::Style;

pub trait Widget: Send {
    fn get_style(&self) -> &Style;
    fn is_show(&self) -> bool;
    fn draw(&self) -> String;
}
