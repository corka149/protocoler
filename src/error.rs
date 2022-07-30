use cursive::traits::*;
use cursive::views::{NamedView, TextView};
use std::fmt;

pub const ERROR_OUTPUT: &str = "error_output";

/// Returns a view in which errors can be shown to the user.
pub fn error_output() -> NamedView<TextView> {
    TextView::new("-").with_name(ERROR_OUTPUT)
}

#[derive(Debug, Clone)]
pub struct InvalidCsvError;

impl fmt::Display for InvalidCsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
