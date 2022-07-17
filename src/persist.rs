use std::path::PathBuf;
use std::str::FromStr;

use cursive::align::{HAlign, VAlign};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, DummyView, EditView, NamedView, Panel, TextView};

use crate::{Cursive, LinearLayout};
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

const TARGET_FILE_BOX_NAME: &'static str = "target_file";

pub fn save_dialog(content: String) -> Dialog {
    let hint = TextView::new(
      "File extension determines format. (.csv, .md and plain (any other ext.))"
    );

    let target_input = Panel::new(
        EditView::default().content(content).min_width(50)
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
        .button("Save", |app| unimplemented!())
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