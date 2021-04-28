use crate::widget::Style;
use crate::widget::Widget;

pub struct Example {
    style: Style,
}

impl Example {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
        }
    }
}

impl Widget for Example {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn draw(&self) -> String {
        String::from("Hello world!")
    }

    fn is_show(&self) -> bool {
        true
    }
}
