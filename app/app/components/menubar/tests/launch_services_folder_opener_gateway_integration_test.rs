//! `LaunchServicesFolderOpenerGateway` with a recording program.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::path::PathBuf;

use menubar::adapters::secondary::folders::launchservices::launch_services_folder_opener_gateway::LaunchServicesFolderOpenerGateway;
use menubar::core::application::services::folder_opener_service::FolderOpenerService;
use support::RecordingProgram;
use tempfile::TempDir;

#[tokio::test]
async fn creates_a_missing_folder_and_opens_it() {
    let dir = TempDir::new().unwrap();
    let open = RecordingProgram::new(dir.path(), "open", 0);
    let folder = dir.path().join("logs $(touch x)");
    let gateway = LaunchServicesFolderOpenerGateway::new(open.program.clone());
    gateway.open(&folder).await.unwrap();
    assert!(folder.is_dir());
    assert_eq!(open.calls(), [[folder.display().to_string()]]);
    assert!(!dir.path().join("x").exists());
}

#[tokio::test]
async fn a_failing_program_is_an_error() {
    let dir = TempDir::new().unwrap();
    let open = RecordingProgram::new(dir.path(), "open", 1);
    let gateway = LaunchServicesFolderOpenerGateway::new(open.program.clone());
    assert!(gateway.open(dir.path()).await.is_err());
}

#[tokio::test]
async fn a_missing_program_is_an_error() {
    let dir = TempDir::new().unwrap();
    let gateway = LaunchServicesFolderOpenerGateway::new(PathBuf::from("/nonexistent/open"));
    assert!(gateway.open(dir.path()).await.is_err());
}
