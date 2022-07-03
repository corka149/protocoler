//! Hint and help.

use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, ListView, Panel, TextView};

/// Creates a help menu.
pub fn help_menu() -> Dialog {
    let help = ListView::new()
        .child("a", TextView::new("Add below"))
        .child("A", TextView::new("Add above"))
        .child("d", TextView::new("Delete an item"))
        .child("e", TextView::new("Edit current item"))
        .child("-", TextView::new("------------------"))
        .child("q", TextView::new("Quit"));

    Dialog::around(help)
        .button("Cancel", |s| {
            s.pop_layer();
        })
}

// Creates a hint bar with short information.
pub fn hint_bar() -> LinearLayout {
    LinearLayout::horizontal().child(
        Panel::new(
            TextView::new("x = menu")
        ).title("Hint").full_width()
    )
}

