use super::{BUTTONS, NAMES, skill_name};
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[test]
fn the_skill_name_ends_with_a_dot_only_before_a_state() {
    assert_eq!(skill_name(Some(SkillStateEnum::Installed)), "Skill ·");
    assert_eq!(skill_name(None), "Skill");
}

#[test]
fn each_row_has_a_name_and_a_button() {
    assert_eq!(NAMES, ["Claude Code folder", "Skill", "Skill text"]);
    assert_eq!(BUTTONS, ["Choose…", "Install", "Copy skill text"]);
}
