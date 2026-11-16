use super::SkillStateEnum;

#[test]
fn each_state_has_its_words() {
    assert_eq!(SkillStateEnum::NotInstalled.label(), "Not installed");
    assert_eq!(SkillStateEnum::Installed.label(), "Installed");
    assert_eq!(SkillStateEnum::Outdated.label(), "An older version is installed");
}

#[test]
fn only_an_older_skill_asks_for_an_update() {
    assert_eq!(SkillStateEnum::NotInstalled.button(), "Install");
    assert_eq!(SkillStateEnum::Installed.button(), "Install");
    assert_eq!(SkillStateEnum::Outdated.button(), "Update");
}
