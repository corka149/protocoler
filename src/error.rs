use cursive::traits::*;
use cursive::views::{NamedView, TextView};

pub const ERROR_OUTPUT: &str = "error_output";

/// Returns a view in which errors can be shown to the user.
pub fn error_output() -> NamedView<TextView> {
    TextView::new("-").with_name(ERROR_OUTPUT)
}
