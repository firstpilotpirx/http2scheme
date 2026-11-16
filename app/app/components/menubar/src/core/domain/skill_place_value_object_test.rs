use std::path::{Path, PathBuf};

use super::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[test]
fn keeps_what_was_reported() {
    let place = SkillPlaceValueObject::new(
        PathBuf::from("/h/.claude/skills/http2scheme/SKILL.md"),
        Some(PathBuf::from("/kit/http2scheme/SKILL.md")),
        SkillStateEnum::Outdated,
    );
    assert_eq!(place.file(), Path::new("/h/.claude/skills/http2scheme/SKILL.md"));
    assert_eq!(place.real_file(), Some(Path::new("/kit/http2scheme/SKILL.md")));
    assert_eq!(place.state(), SkillStateEnum::Outdated);
}
