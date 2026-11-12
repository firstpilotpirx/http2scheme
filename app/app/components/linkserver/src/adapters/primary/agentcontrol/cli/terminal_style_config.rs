//! Whether and how the commands color their output.

use owo_colors::OwoColorize as _;

/// The color decision and the styled pieces every command prints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalStyleConfig {
    color: bool,
}

impl TerminalStyleConfig {
    /// A style with color on or off.
    #[must_use]
    pub const fn new(color: bool) -> Self {
        Self { color }
    }

    /// Color unless `--no-color`, `NO_COLOR` or `TERM=dumb`, or the stream is not a terminal;
    /// `FORCE_COLOR=1` turns it on for a stream that is not a terminal.
    #[must_use]
    pub fn detect(
        no_color_flag: bool,
        is_terminal: bool,
        env: impl Fn(&str) -> Option<String>,
    ) -> Self {
        let set = |name: &str| env(name).is_some_and(|value| !value.is_empty());
        let color = if no_color_flag || set("NO_COLOR") {
            false
        } else if env("FORCE_COLOR").as_deref() == Some("1") {
            true
        } else {
            env("TERM").as_deref() != Some("dumb") && is_terminal
        };
        Self { color }
    }

    /// Whether the output is colored.
    #[must_use]
    pub const fn color(self) -> bool {
        self.color
    }

    /// `✓ text`, the mark in green.
    #[must_use]
    pub fn ok(self, text: &str) -> String {
        format!("{} {text}", self.mark(true))
    }

    /// `✓ text`, all in bold green: the final success line.
    #[must_use]
    pub fn done(self, text: &str) -> String {
        if self.color {
            format!("✓ {text}").green().bold().to_string()
        } else {
            format!("[ok] {text}")
        }
    }

    /// `✗ text`, all in bold red.
    #[must_use]
    pub fn fail(self, text: &str) -> String {
        if self.color {
            format!("✗ {text}").red().bold().to_string()
        } else {
            format!("[fail] {text}")
        }
    }

    /// `●` and the text in bold purple: a title line.
    #[must_use]
    pub fn title(self, text: &str) -> String {
        if self.color {
            format!("● {text}").magenta().bold().to_string()
        } else {
            format!("[..] {text}")
        }
    }

    /// A green `✓` or a red `✗`.
    #[must_use]
    pub fn mark(self, good: bool) -> String {
        match (self.color, good) {
            (true, true) => "✓".green().to_string(),
            (true, false) => "✗".red().to_string(),
            (false, true) => "[ok]".to_owned(),
            (false, false) => "[fail]".to_owned(),
        }
    }

    /// An address or a path, in cyan.
    #[must_use]
    pub fn path(self, text: &str) -> String {
        if self.color {
            text.cyan().to_string()
        } else {
            text.to_owned()
        }
    }

    /// A detail line, in gray.
    #[must_use]
    pub fn detail(self, text: &str) -> String {
        if self.color {
            text.bright_black().to_string()
        } else {
            text.to_owned()
        }
    }
}

#[cfg(test)]
#[path = "terminal_style_config_test.rs"]
mod terminal_style_config_test;
