use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::InstallSkillCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::use_cases::install_skill::MockInstallSkill;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

async fn run(use_case: MockInstallSkill, dir: Option<&Path>) -> (u8, String, String) {
    let command = InstallSkillCommand::new(
        Arc::new(use_case),
        PathBuf::from("/h/.claude"),
        TerminalStyleConfig::new(false),
    );
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(dir, &mut out, &mut err).await;
    (
        code,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

fn installs(real: Option<&'static str>) -> MockInstallSkill {
    let mut use_case = MockInstallSkill::new();
    use_case.expect_execute().returning(move |dir| {
        Ok(SkillPlaceValueObject::new(
            dir,
            real.map(PathBuf::from),
            SkillStateEnum::Installed,
        ))
    });
    use_case
}

#[tokio::test]
async fn without_a_folder_it_installs_into_the_default_one() {
    let (code, out, _) = run(installs(None), None).await;
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "[ok] skill installed: /h/.claude/skills/http2scheme/SKILL.md\n"
    );
}

#[tokio::test]
async fn a_named_folder_is_used() {
    let (_, out, _) = run(installs(None), Some(Path::new("/w/.claude"))).await;
    assert!(out.contains("/w/.claude/skills/http2scheme/SKILL.md"), "{out}");
}

#[tokio::test]
async fn a_linked_skills_folder_prints_the_real_file() {
    let (_, out, _) = run(installs(Some("/kit/skills/http2scheme/SKILL.md")), None).await;
    assert!(
        out.contains("  lands in /kit/skills/http2scheme/SKILL.md (skills is a link)"),
        "{out}"
    );
}

#[tokio::test]
async fn a_write_error_fails_with_the_reason() {
    let mut use_case = MockInstallSkill::new();
    use_case.expect_execute().returning(|_| {
        Err(InstallSkillError::Io {
            path: "/h/.claude/skills".to_owned(),
            reason: "permission denied".to_owned(),
        })
    });
    let (code, out, err) = run(use_case, None).await;
    assert_eq!(code, 1);
    assert!(out.is_empty());
    assert!(err.contains("/h/.claude/skills: permission denied"), "{err}");
}
