use cursive::Cursive;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};

fn main() {
    let mut siv = cursive::default();

    let layout = LinearLayout::vertical()
        .child(TextView::new("This is a dynamic theme example!"))
        .child(EditView::new().content("Woo! colors!"));

    siv.add_fullscreen_layer(
        Dialog::around(layout)
            .button("Quit", Cursive::quit)
            .full_screen(),
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.update_theme(|theme| {
        theme.shadow = false;
        theme.borders = BorderStyle::Simple;

        theme.palette.set_color("Background", Color::TerminalDefault);
        theme.palette.set_color("Shadow", Color::TerminalDefault);
        theme.palette.set_color("View", Color::TerminalDefault);
        theme.palette.set_color("Primary", Color::TerminalDefault);
        theme.palette.set_color("Secondary", Color::TerminalDefault);
        theme.palette.set_color("Tertiary", Color::TerminalDefault);
        theme.palette.set_color("TitlePrimary", Color::TerminalDefault);
        theme.palette.set_color("TitleSecondary", Color::TerminalDefault);
    });

    siv.run();
}