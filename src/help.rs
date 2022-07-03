use cursive::views::{Dialog, ListView, TextView};

pub fn new() -> Dialog {
    let help = ListView::new()
        .child("a", TextView::new("Add below"))
        .child("A", TextView::new("Add above"))
        .child("d", TextView::new("Delete an item"))
        .child("e", TextView::new("Edit current item"))
        .child("-", TextView::new( "------------------"))
        .child("q", TextView::new( "Quit"));

    Dialog::around(help)
        .button("Cancel", |s| {
            s.pop_layer();
        })
}
