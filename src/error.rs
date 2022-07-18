use cursive::traits::*;
use cursive::views::{NamedView, TextView};

pub const ERROR_OUTPUT: &str = "error_output";

pub fn error_output() -> NamedView<TextView> {
    TextView::new("-")
        .with_name(ERROR_OUTPUT)
}
