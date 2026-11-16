use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::path::PathBuf;

use super::{MenubarHooks, skill_place};
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[test]
fn the_hooks_call_what_the_app_hands_in() {
    let started = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&started);
    let hooks = MenubarHooks {
        serve: Arc::new(move |_, _, _| {
            counter.fetch_add(1, Ordering::SeqCst);
        }),
        replay: Arc::new(|link| format!("opened {link}")),
        skill_text: Arc::new(|| "skill".to_owned()),
        inspect_skill: Arc::new(|_| Err("inspected".to_owned())),
        install_skill: Arc::new(|_| Err("installed".to_owned())),
    };
    (hooks.serve)(
        Box::pin(std::future::ready(())),
        Box::new(std::io::sink()),
        Box::new(std::io::sink()),
    );
    assert_eq!(started.load(Ordering::SeqCst), 1);
    assert_eq!((hooks.replay)("idea://x"), "opened idea://x");
    assert_eq!((hooks.skill_text)(), "skill");
    let dir = std::path::Path::new("/c");
    assert_eq!((hooks.inspect_skill)(dir).unwrap_err(), "inspected");
    assert_eq!((hooks.install_skill)(dir).unwrap_err(), "installed");
}

#[test]
fn a_server_answer_becomes_a_skill_place() {
    let place = skill_place((
        PathBuf::from("/c/skills/http2scheme/SKILL.md"),
        Some(PathBuf::from("/real/SKILL.md")),
        "outdated",
    ))
    .unwrap();
    assert_eq!(place.state(), SkillStateEnum::Outdated);
    assert_eq!(
        place.real_file(),
        Some(std::path::Path::new("/real/SKILL.md"))
    );
}

#[test]
fn an_unknown_state_name_is_a_reason() {
    assert_eq!(
        skill_place((PathBuf::from("/c"), None, "gone")).unwrap_err(),
        "unknown skill state: gone"
    );
}
