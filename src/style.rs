use cursive::theme::{BorderStyle, Color, Theme};

/// Sets most color keys to terminal default.
pub fn set_terminal_default(theme: &mut Theme) {
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
}