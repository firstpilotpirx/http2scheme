use std::path::{Path, PathBuf};

use super::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[test]
fn the_file_lies_in_the_skills_folder() {
    let place = SkillPlaceValueObject::new(Path::new("/h/.claude"), None, SkillStateEnum::Installed);
    assert_eq!(place.file(), Path::new("/h/.claude/skills/http2scheme/SKILL.md"));
    assert_eq!(place.real_file(), None);
    assert_eq!(place.state(), SkillStateEnum::Installed);
}

#[test]
fn keeps_the_real_file_of_a_linked_folder() {
    let real = PathBuf::from("/kit/skills/http2scheme/SKILL.md");
    let place = SkillPlaceValueObject::new(
        Path::new("/h/.claude"),
        Some(real.clone()),
        SkillStateEnum::NotInstalled,
    );
    assert_eq!(place.real_file(), Some(real.as_path()));
}
