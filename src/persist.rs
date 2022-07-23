use std::path::PathBuf;
use std::str::FromStr;

use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, DummyView, EditView, NamedView, Panel, TextView, ViewRef};

use crate::{Cursive, error, LinearLayout, ProtocolTable, report, table_name};
use crate::persist::SaveStatus::{Saved, Unsaved};

#[derive(Debug, PartialEq)]
enum SaveStatus {
    Unsaved,
    Saved { path: PathBuf },
}

impl SaveStatus {
    pub const UNSAVED: &'static str = "*unsaved*";

    pub fn from_str(as_text: &str) -> SaveStatus {
        match as_text {
            path if !path.starts_with('*') && !path.ends_with('*') => {
                match PathBuf::from_str(path) {
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

impl From<StyledString> for SaveStatus {
    fn from(styled_string: StyledString) -> Self {
        SaveStatus::from_str(styled_string.source())
    }
}

impl From<String> for SaveStatus {
    fn from(string: String) -> Self {
        SaveStatus::from_str(&string)
    }
}

impl From<SaveStatus> for String {
    fn from(status: SaveStatus) -> Self {
        match status {
            Unsaved => SaveStatus::UNSAVED.to_string(),
            Saved { path } => match path.to_str() {
                None => SaveStatus::UNSAVED.to_string(),
                Some(path) => path.to_string()
            }
        }
    }
}

// ===== ===== module ===== =====

const TARGET_FILE_BOX_NAME: &str = "target_file_box";
const TARGET_FILE_INPUT_NAME: &str = "target_file_input";

/// Dialog for entering the save path and triggering the save process.
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

    app.call_on_name(error::ERROR_OUTPUT, |err: &mut TextView| {
        err.set_content("-")
    });

    let err_output = app.find_name::<TextView>(error::ERROR_OUTPUT);
    let success = app.call_on_name(TARGET_FILE_INPUT_NAME, |target_input: &mut EditView| {
        let target_path = target_input.get_content();
        let target_path = target_path.as_str();

        if let Some(mut text_box) = text_box {
            text_box.set_content(target_path);
        }

        let target_path = PathBuf::from_str(target_path);

        if let (Some(mut table), Ok(target_path)) = (table, target_path) {
            let entries = table.borrow_items();

            let save_result = report::save(entries, &target_path);

            if let (Err(err), Some(mut err_output)) = (save_result, err_output) {
                err_output.set_content(err.to_string());
                return false;
            }
        }

        true
    });

    if success.unwrap_or(true) {
        app.pop_layer();
    }
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