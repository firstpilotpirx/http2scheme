//! The app's skill hooks over real folders, without the server component.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use menubar::core::domain::skill_place_value_object::SkillPlaceValueObject;
use menubar::core::domain::skill_state_enum::SkillStateEnum;
use menubar::menubar_hooks::{SkillAction, SkillText};

/// The text the fake app's skill has.
pub const SKILL_TEXT: &str = "---\nname: http2scheme\n---\nthe app's skill text\n";

/// Reads and writes `<folder>/skills/http2scheme/SKILL.md` as the server side does.
pub struct FakeSkill;

fn file(folder: &Path) -> PathBuf {
    folder.join("skills/http2scheme/SKILL.md")
}

fn real_file(folder: &Path) -> Option<PathBuf> {
    let skills = folder.join("skills");
    let linked = fs::symlink_metadata(&skills).is_ok_and(|meta| meta.file_type().is_symlink());
    linked.then(|| {
        fs::canonicalize(&skills)
            .unwrap()
            .join("http2scheme/SKILL.md")
    })
}

fn inspect(folder: &Path) -> SkillPlaceValueObject {
    let state = match fs::read_to_string(file(folder)) {
        Ok(text) if text == SKILL_TEXT => SkillStateEnum::Installed,
        Ok(_) => SkillStateEnum::Outdated,
        Err(_) => SkillStateEnum::NotInstalled,
    };
    SkillPlaceValueObject::new(file(folder), real_file(folder), state)
}

fn install(folder: &Path) -> Result<SkillPlaceValueObject, String> {
    let path = file(folder);
    fs::create_dir_all(path.parent().unwrap()).map_err(|error| error.to_string())?;
    fs::write(&path, SKILL_TEXT).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(inspect(folder))
}

impl FakeSkill {
    /// The text hook.
    pub fn text() -> SkillText {
        Arc::new(|| SKILL_TEXT.to_owned())
    }

    /// The inspect hook.
    pub fn inspect() -> SkillAction {
        Arc::new(|folder| Ok(inspect(folder)))
    }

    /// The install hook.
    pub fn install() -> SkillAction {
        Arc::new(install)
    }
}
