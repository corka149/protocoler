//! Hint and help.

use crate::{error, persist};
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, ListView, Panel, TextView};

/// Creates a help menu.
pub fn help_menu() -> Dialog {
    let help = ListView::new()
        .child("a", TextView::new("Add new item"))
        .child("d", TextView::new("Delete an item"))
        .child("e", TextView::new("Edit current item"))
        .child("-", TextView::new("------------------"))
        .child("s", TextView::new("Save protocol"))
        .child("q", TextView::new("Quit"));

    Dialog::around(help).button("Cancel", |s| {
        s.pop_layer();
    })
}

// Creates a hint bar with short information.
pub fn hint_bar() -> LinearLayout {
    let content = LinearLayout::horizontal()
        .child(TextView::new("x = menu"))
        .child(DummyView::full_width(DummyView))
        .child(error::error_output())
        .child(DummyView::full_width(DummyView))
        .child(persist::target_fila_text());

    LinearLayout::horizontal().child(Panel::new(content).title("Hint").full_width())
}
