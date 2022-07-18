use std::path::PathBuf;
use std::str::FromStr;

use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, DummyView, EditView, NamedView, Panel, TextView, ViewRef};

use crate::{Cursive, LinearLayout, ProtocolTable, report, table_name};
use crate::persist::SaveStatus::{Saved, Unsaved};

#[derive(Debug, PartialEq)]
enum SaveStatus {
    Unsaved,
    Saved { path: PathBuf },
}

impl SaveStatus {
    pub fn from_str(as_text: &str) -> SaveStatus {
        match as_text {
            path if !path.starts_with("*") && !path.ends_with("*") => {
                match PathBuf::from_str(&path) {
                    Ok(path) => Saved { path },
                    Err(_) => Unsaved
                }
            }
            _ => Unsaved
        }
    }
}

impl Default for SaveStatus {
    fn default() -> Self {
        Unsaved
    }
}

impl Into<StyledString> for SaveStatus {
    fn into(self) -> StyledString {
        let content = match self {
            Unsaved => "*unsaved*".to_string(),
            Saved { path } => match path.to_str() {
                None => "*unknown*".to_string(),
                Some(path_str) => path_str.to_string()
            }
        };

        StyledString::plain(content)
    }
}

// ===== ===== module ===== =====

const TARGET_FILE_BOX_NAME: &'static str = "target_file_box";
const TARGET_FILE_INPUT_NAME: &'static str = "target_file_input";

pub fn save_dialog(content: String) -> Dialog {
    let hint = TextView::new(
        "File extension determines format. (.csv, .md and plain (any other ext.))"
    );

    let target_input = Panel::new(
        EditView::default()
            .content(content)
            .with_name(TARGET_FILE_INPUT_NAME)
            .min_width(50)
    ).title("Target path");

    Dialog::default()
        .title("Save protocol")
        .content(
            LinearLayout::vertical()
                .child(DummyView)
                .child(target_input)
                .child(DummyView)
                .child(hint)
                .child(DummyView)
        )
        .button("Save", save)
        .button("Cancel", |app| {
            app.pop_layer();
        })
}

pub fn target_fila_text() -> NamedView<TextView> {
    TextView::new(Unsaved).with_name(TARGET_FILE_BOX_NAME)
}

pub fn get_target_path(app: &mut Cursive) -> Option<PathBuf> {
    if let Some(text_view) = app.find_name::<TextView>(TARGET_FILE_BOX_NAME) {
        let content = text_view.get_content();
        let src = content.source();
        match SaveStatus::from_str(src) {
            Unsaved => None,
            Saved { path } => Some(path)
        }
    } else {
        None
    }
}

fn save(app: &mut Cursive) {
    let text_box = app.find_name::<TextView>(TARGET_FILE_BOX_NAME);
    let table: Option<ViewRef<ProtocolTable>> = app.find_name(table_name());

    app.call_on_name(TARGET_FILE_INPUT_NAME, |target_input: &mut EditView| {
        let target_path = target_input.get_content();
        let target_path = target_path.as_str();

        if let Some(mut text_box) = text_box {
            text_box.set_content(target_path);
        }

        let target_path = PathBuf::from_str(target_path);

        if let (Some(mut table), Ok(target_path)) = (table, target_path) {
            let entries = table.borrow_items();

            let save_result = match target_path {
                path if path.ends_with(".md") =>
                    report::save_markdown(entries, &path),

                path if path.ends_with(".csv") =>
                    report::save_csv(entries, &path),

                path =>
                    report::save_raw(entries, &path)
            };
        }
    });
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_save_status_from_str() {
        assert_eq!(Unsaved, SaveStatus::from_str("*unsaved*"));
        assert_eq!(Unsaved, SaveStatus::from_str("*unknown*"));
    }
}