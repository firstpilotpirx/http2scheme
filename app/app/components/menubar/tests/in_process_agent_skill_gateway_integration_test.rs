//! `InProcessAgentSkillGateway` over the app's skill hooks.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use menubar::adapters::secondary::agentskill::inprocess::in_process_agent_skill_gateway::InProcessAgentSkillGateway;
use menubar::core::application::services::agent_skill_service::AgentSkillService;
use menubar::menubar_hooks::skill_place;

#[tokio::test]
async fn each_call_goes_to_its_hook_and_its_answer_comes_back() {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let (inspected, installed) = (Arc::clone(&seen), Arc::clone(&seen));
    let gateway = InProcessAgentSkillGateway::new(
        Arc::new(|| "the skill".to_owned()),
        Arc::new(move |dir: &Path| {
            inspected
                .lock()
                .unwrap()
                .push(format!("inspect {}", dir.display()));
            skill_place((dir.join("SKILL.md"), None, "outdated"))
        }),
        Arc::new(move |dir: &Path| {
            installed
                .lock()
                .unwrap()
                .push(format!("install {}", dir.display()));
            Err("denied".to_owned())
        }),
    );
    assert_eq!(gateway.text().await, "the skill");
    let place = gateway.inspect(Path::new("/c")).await.unwrap();
    assert_eq!(place.file(), PathBuf::from("/c/SKILL.md"));
    assert_eq!(
        gateway.install(Path::new("/w")).await.unwrap_err(),
        "denied"
    );
    assert_eq!(*seen.lock().unwrap(), ["inspect /c", "install /w"]);
}
