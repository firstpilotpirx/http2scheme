//! `LaunchServicesSchemeLinkOpenerGateway` with a recording program, and once with the real `open`.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::path::PathBuf;

use linkserver::adapters::secondary::linkopening::launchservices::launch_services_scheme_link_opener_gateway::LaunchServicesSchemeLinkOpenerGateway;
use linkserver::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use linkserver::core::domain::scheme_link_value_object::SchemeLinkValueObject;
use support::RecordingProgram;
use tempfile::TempDir;

#[tokio::test]
async fn passes_the_exact_link_as_one_argument() {
    let dir = TempDir::new().unwrap();
    let open = RecordingProgram::new(dir.path(), "open", 0);
    let text = "idea://open?file=/a%20b/\"q\";$(touch%20x)'.java&line=17";
    let gateway = LaunchServicesSchemeLinkOpenerGateway::new(open.program.clone());
    gateway
        .open(&SchemeLinkValueObject::parse(text).unwrap())
        .await
        .unwrap();
    assert_eq!(open.calls(), [[text.to_owned()]]);
    assert!(!dir.path().join("x").exists());
}

#[tokio::test]
async fn a_failing_program_is_an_error() {
    let dir = TempDir::new().unwrap();
    let open = RecordingProgram::new(dir.path(), "open", 1);
    let gateway = LaunchServicesSchemeLinkOpenerGateway::new(open.program.clone());
    let error = gateway
        .open(&SchemeLinkValueObject::parse("idea://x").unwrap())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("recorded failure"), "{error}");
}

#[tokio::test]
async fn a_missing_program_is_an_error() {
    let gateway = LaunchServicesSchemeLinkOpenerGateway::new(PathBuf::from("/nonexistent/open"));
    assert!(
        gateway
            .open(&SchemeLinkValueObject::parse("idea://x").unwrap())
            .await
            .is_err()
    );
}

#[tokio::test]
#[ignore = "runs the real /usr/bin/open on macOS"]
async fn the_real_open_refuses_an_unregistered_scheme() {
    let gateway = LaunchServicesSchemeLinkOpenerGateway::new(PathBuf::from("/usr/bin/open"));
    let link = SchemeLinkValueObject::parse("http2scheme-unregistered-test://x").unwrap();
    assert!(gateway.open(&link).await.is_err());
}
