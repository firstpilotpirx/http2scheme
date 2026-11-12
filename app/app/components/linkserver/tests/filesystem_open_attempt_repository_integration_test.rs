//! `FilesystemOpenAttemptRepository` against a real temporary folder.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::fs;
use std::time::{Duration, UNIX_EPOCH};

use linkserver::adapters::secondary::requestlog::filesystem::filesystem_open_attempt_repository::FilesystemOpenAttemptRepository;
use linkserver::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use linkserver::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use linkserver::core::domain::open_outcome_enum::OpenOutcomeEnum;
use tempfile::TempDir;

#[tokio::test]
async fn appends_one_line_per_event_and_creates_the_folder() {
    let home = TempDir::new().unwrap();
    let file = home.path().join("Logs/http2scheme/requests.log");
    let repository = FilesystemOpenAttemptRepository::new(file.clone());
    let at = UNIX_EPOCH + Duration::from_secs(1_763_000_000);
    repository
        .record(&LinkOpenAttemptedDomainEvent::new(
            at,
            "idea://x".to_owned(),
            OpenOutcomeEnum::Opened,
        ))
        .await
        .unwrap();
    repository
        .record(&LinkOpenAttemptedDomainEvent::new(
            at,
            "a\nb".to_owned(),
            OpenOutcomeEnum::WrongKey,
        ))
        .await
        .unwrap();
    let text = fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines,
        [
            "2025-11-13T02:13:20Z\topened\tidea://x",
            "2025-11-13T02:13:20Z\twrong key\ta\\nb"
        ]
    );
}
