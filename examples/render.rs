use yatui::app::App;
use yatui::backend::Termion;

use yatui::component::{
    layout::{column, line},
    text,
};
use yatui::state::mut_state;

fn main() {
    let termion = Termion::new(std::io::stdout()).unwrap();
    let mut app = App::new(termion);

    let caption = mut_state("Hello");

    let entry = column([
        text("Example of layout"),
        text("Another column \n with multiline \n string"),
        line([text("There is a line layout "), text(caption.clone())]),
    ]);

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        // Update here will re-render entry
        caption.update(|v| v.push_str(" world"));
    });

    app.mount(entry);
    app.run();
}
