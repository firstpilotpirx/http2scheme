//! `FilesystemAgentDefinitionRepository` against a real temporary folder.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::fs;
use std::path::PathBuf;

use linkserver::adapters::secondary::agentcontrol::filesystem::filesystem_agent_definition_repository::FilesystemAgentDefinitionRepository;
use linkserver::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use linkserver::core::domain::agent_definition_value_object::AgentDefinitionValueObject;
use tempfile::TempDir;

const LABEL: &str = "io.example.agent";

fn definition() -> AgentDefinitionValueObject {
    AgentDefinitionValueObject::new(
        LABEL.to_owned(),
        PathBuf::from("/opt/x & y/tool"),
        PathBuf::from("/logs"),
    )
}

#[tokio::test]
async fn writes_a_property_list_that_runs_serve_at_login_and_keeps_it_alive() {
    let home = TempDir::new().unwrap();
    let agents = home.path().join("Library/LaunchAgents");
    let repository = FilesystemAgentDefinitionRepository::new(agents.clone());
    repository.save(&definition()).await.unwrap();
    let text = fs::read_to_string(agents.join("io.example.agent.plist")).unwrap();
    for expected in [
        "<key>Label</key>\n    <string>io.example.agent</string>",
        "<string>/opt/x &amp; y/tool</string>\n        <string>serve</string>",
        "<key>RunAtLoad</key>\n    <true/>",
        "<key>KeepAlive</key>\n    <true/>",
        "<string>/logs/agent.out.log</string>",
        "<string>/logs/agent.err.log</string>",
    ] {
        assert!(text.contains(expected), "missing {expected:?} in\n{text}");
    }
    let lint = std::process::Command::new("/usr/bin/plutil")
        .arg("-lint")
        .arg(agents.join("io.example.agent.plist"))
        .status()
        .unwrap();
    assert!(lint.success());
}

#[tokio::test]
async fn finds_and_deletes_the_definition() {
    let home = TempDir::new().unwrap();
    let repository = FilesystemAgentDefinitionRepository::new(home.path().to_path_buf());
    assert!(!repository.exists(LABEL).await.unwrap());
    repository.save(&definition()).await.unwrap();
    assert!(repository.exists(LABEL).await.unwrap());
    repository.save(&definition()).await.unwrap();
    repository.delete(LABEL).await.unwrap();
    assert!(!repository.exists(LABEL).await.unwrap());
}

#[tokio::test]
async fn deleting_a_missing_definition_is_an_error() {
    let home = TempDir::new().unwrap();
    let repository = FilesystemAgentDefinitionRepository::new(home.path().to_path_buf());
    assert!(repository.delete(LABEL).await.is_err());
}
