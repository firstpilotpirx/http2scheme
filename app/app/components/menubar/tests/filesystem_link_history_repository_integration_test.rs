//! `FilesystemLinkHistoryRepository` over a request log in a temporary folder.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use std::fmt::Write as _;
use std::path::Path;

use menubar::adapters::secondary::linkhistory::filesystem::filesystem_link_history_repository::FilesystemLinkHistoryRepository;
use menubar::core::application::repositories::link_history_repository::LinkHistoryRepository;

fn write_log(dir: &Path, lines: &str) {
    std::fs::write(dir.join("requests.log"), lines).unwrap();
}

async fn links(dir: &Path, limit: usize) -> Vec<String> {
    FilesystemLinkHistoryRepository::new(dir)
        .find_latest(limit)
        .await
        .unwrap()
        .iter()
        .map(|record| record.link().to_owned())
        .collect()
}

#[tokio::test]
async fn the_latest_requests_come_newest_first() {
    let dir = tempfile::tempdir().unwrap();
    let mut log = String::new();
    for number in 0..105 {
        writeln!(
            log,
            "2026-11-12T10:00:00Z\topened\tidea://open?line={number}"
        )
        .unwrap();
    }
    write_log(dir.path(), &log);
    let latest = links(dir.path(), 100).await;
    assert_eq!(latest.len(), 100);
    assert_eq!(latest[0], "idea://open?line=104");
    assert_eq!(latest[99], "idea://open?line=5");
}

#[tokio::test]
async fn a_damaged_line_is_skipped() {
    let dir = tempfile::tempdir().unwrap();
    write_log(
        dir.path(),
        "2026-11-12T10:00:00Z\topened\tidea://a\ngarbage\n2026-11-12T10:01:00Z\twrong key\tgoland://b\n",
    );
    assert_eq!(links(dir.path(), 100).await, ["goland://b", "idea://a"]);
}

#[tokio::test]
async fn a_missing_log_has_no_request() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
        links(&dir.path().join("logs"), 100).await,
        Vec::<String>::new()
    );
}
