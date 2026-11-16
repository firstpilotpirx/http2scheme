use super::InstallSkillError;

#[test]
fn names_the_path_and_the_reason() {
    let error = InstallSkillError::Io {
        path: "/h/.claude/skills".to_owned(),
        reason: "permission denied".to_owned(),
    };
    assert_eq!(error.to_string(), "/h/.claude/skills: permission denied");
}
