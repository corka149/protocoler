use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout};

mod style;

fn main() {
    let mut siv = cursive::default();

    let layout = LinearLayout::vertical();

    siv.add_fullscreen_layer(
        Dialog::around(layout)
            .title("Protocoler")
            .full_screen(),
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.update_theme(style::set_terminal_default);

    siv.run();
}
