//! What the Agents tab shows at one moment.

use std::path::{Path, PathBuf};

use super::skill_place_value_object::SkillPlaceValueObject;
use super::skill_state_enum::SkillStateEnum;

/// The line under Skill text before a copy.
pub const PASTE_NOTE: &str = "Paste it into any agent's skill folder.";
/// The line under Skill text after a copy.
pub const COPIED_NOTE: &str = "Copied to the clipboard.";

/// The shown folder, the skill's place and state, the last failure and whether the text was copied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentsViewValueObject {
    folder: PathBuf,
    home: PathBuf,
    place: Option<SkillPlaceValueObject>,
    error: Option<String>,
    clipboard: Option<String>,
}

impl AgentsViewValueObject {
    /// The view of `folder`, writing `home` as `~`; `place` is `None` when it could not be read.
    #[must_use]
    pub const fn new(
        folder: PathBuf,
        home: PathBuf,
        place: Option<SkillPlaceValueObject>,
        error: Option<String>,
    ) -> Self {
        Self {
            folder,
            home,
            place,
            error,
            clipboard: None,
        }
    }

    /// The same view after Copy skill text: `text` goes on the clipboard.
    #[must_use]
    pub fn copied(mut self, text: String) -> Self {
        self.clipboard = Some(text);
        self
    }

    /// The text to put on the clipboard, after Copy skill text.
    #[must_use]
    pub fn clipboard(&self) -> Option<&str> {
        self.clipboard.as_deref()
    }

    /// The shown Claude Code folder.
    #[must_use]
    pub fn folder(&self) -> &Path {
        &self.folder
    }

    fn short(&self, path: &Path) -> String {
        match path.strip_prefix(&self.home) {
            Ok(rest) if rest.as_os_str().is_empty() => "~".to_owned(),
            Ok(rest) => format!("~/{}", rest.display()),
            Err(_) => path.display().to_string(),
        }
    }

    /// The line under Claude Code folder.
    #[must_use]
    pub fn folder_line(&self) -> String {
        self.short(&self.folder)
    }

    /// The skill state, when it could be read.
    #[must_use]
    pub fn state(&self) -> Option<SkillStateEnum> {
        self.place.as_ref().map(SkillPlaceValueObject::state)
    }

    /// The name of the skill row: `Skill · <state>`.
    #[must_use]
    pub fn skill_title(&self) -> String {
        self.state().map_or_else(
            || "Skill".to_owned(),
            |state| format!("Skill · {}", state.label()),
        )
    }

    /// The line under the skill row: the failure, else where the file lands.
    #[must_use]
    pub fn skill_line(&self) -> String {
        if let Some(error) = &self.error {
            return error.clone();
        }
        match &self.place {
            Some(place) => match place.real_file() {
                Some(real) => format!("Lands in {} (skills is a link)", self.short(real)),
                None => self.short(place.file()),
            },
            None => String::new(),
        }
    }

    /// The failure of the last read or write.
    #[must_use]
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// The install button's title.
    #[must_use]
    pub fn button(&self) -> &'static str {
        self.state().map_or("Install", SkillStateEnum::button)
    }

    /// The line under Skill text.
    #[must_use]
    pub const fn copy_line(&self) -> &'static str {
        if self.is_copied() {
            COPIED_NOTE
        } else {
            PASTE_NOTE
        }
    }

    /// Whether the text was just copied.
    #[must_use]
    pub const fn is_copied(&self) -> bool {
        self.clipboard.is_some()
    }
}

#[cfg(test)]
#[path = "agents_view_value_object_test.rs"]
mod agents_view_value_object_test;
