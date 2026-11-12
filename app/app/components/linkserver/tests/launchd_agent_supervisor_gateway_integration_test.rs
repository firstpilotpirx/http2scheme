//! `LaunchdAgentSupervisorGateway` with a recording `launchctl`, and once with the real one.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use linkserver::adapters::secondary::agentcontrol::filesystem::filesystem_agent_definition_repository::FilesystemAgentDefinitionRepository;
use linkserver::adapters::secondary::agentcontrol::launchd::launchd_agent_supervisor_gateway::LaunchdAgentSupervisorGateway;
use linkserver::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use linkserver::core::application::services::agent_supervisor_service::AgentSupervisorService;
use linkserver::core::domain::agent_definition_value_object::AgentDefinitionValueObject;
use support::RecordingProgram;
use tempfile::TempDir;

const LABEL: &str = "io.example.agent";

fn definition() -> AgentDefinitionValueObject {
    AgentDefinitionValueObject::new(
        LABEL.to_owned(),
        PathBuf::from("/bin/tool"),
        PathBuf::from("/logs"),
    )
}

#[tokio::test]
async fn load_bootstraps_the_definition_file_in_the_user_domain() {
    let dir = TempDir::new().unwrap();
    let launchctl = RecordingProgram::new(dir.path(), "launchctl", 0);
    let gateway = LaunchdAgentSupervisorGateway::new(
        launchctl.program.clone(),
        501,
        dir.path().join("agents"),
    );
    gateway.load(&definition()).await.unwrap();
    let plist = dir
        .path()
        .join("agents/io.example.agent.plist")
        .display()
        .to_string();
    assert_eq!(
        launchctl.calls(),
        [vec!["bootstrap".to_owned(), "gui/501".to_owned(), plist]]
    );
}

#[tokio::test]
async fn unload_boots_out_the_label() {
    let dir = TempDir::new().unwrap();
    let launchctl = RecordingProgram::new(dir.path(), "launchctl", 0);
    let gateway = LaunchdAgentSupervisorGateway::new(
        launchctl.program.clone(),
        501,
        dir.path().to_path_buf(),
    );
    gateway.unload(LABEL).await.unwrap();
    assert_eq!(launchctl.calls(), [["bootout", "gui/501/io.example.agent"]]);
}

#[tokio::test]
async fn is_loaded_reads_the_print_exit_code() {
    let dir = TempDir::new().unwrap();
    let loaded = RecordingProgram::new(dir.path(), "loaded", 0);
    let absent = RecordingProgram::new(dir.path(), "absent", 113);
    let yes =
        LaunchdAgentSupervisorGateway::new(loaded.program.clone(), 501, dir.path().to_path_buf());
    let no =
        LaunchdAgentSupervisorGateway::new(absent.program.clone(), 501, dir.path().to_path_buf());
    assert!(yes.is_loaded(LABEL).await.unwrap());
    assert!(!no.is_loaded(LABEL).await.unwrap());
    assert_eq!(loaded.calls(), [["print", "gui/501/io.example.agent"]]);
}

#[tokio::test]
async fn a_failing_command_is_an_error() {
    let dir = TempDir::new().unwrap();
    let launchctl = RecordingProgram::new(dir.path(), "launchctl", 5);
    let gateway = LaunchdAgentSupervisorGateway::new(
        launchctl.program.clone(),
        501,
        dir.path().to_path_buf(),
    );
    let error = gateway.unload(LABEL).await.unwrap_err();
    assert!(error.to_string().contains("recorded failure"), "{error}");
}

#[tokio::test]
#[ignore = "runs the real /bin/launchctl with a throw-away agent"]
async fn the_real_launchctl_loads_and_unloads_a_throw_away_agent() {
    let dir = TempDir::new().unwrap();
    let uid = fs::metadata(std::env::var("HOME").unwrap()).unwrap().uid();
    let label = format!(
        "io.github.firstpilotpirx.http2scheme.test{}",
        std::process::id()
    );
    let throw_away = AgentDefinitionValueObject::new(
        label.clone(),
        PathBuf::from("/usr/bin/true"),
        dir.path().to_path_buf(),
    );
    FilesystemAgentDefinitionRepository::new(dir.path().to_path_buf())
        .save(&throw_away)
        .await
        .unwrap();
    let gateway = LaunchdAgentSupervisorGateway::new(
        PathBuf::from("/bin/launchctl"),
        uid,
        dir.path().to_path_buf(),
    );
    gateway.load(&throw_away).await.unwrap();
    assert!(gateway.is_loaded(&label).await.unwrap());
    gateway.unload(&label).await.unwrap();
    assert!(!gateway.is_loaded(&label).await.unwrap());
}
