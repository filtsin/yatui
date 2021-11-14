use crate::widget::{Style, Widget};

pub struct Example {
    style: Style,
}

impl Example {
    pub fn new() -> Self {
        Self { style: Style::default() }
    }
}

impl Widget for Example {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn is_show(&self) -> bool {
        true
    }

    fn draw(&self) -> String {
        String::from("❤️")
    }
}
