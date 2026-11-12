use std::path::Path;

use super::FilesystemLinkHistoryRepository;
use crate::core::application::repositories::link_history_repository::LinkHistoryRepository;

#[tokio::test]
async fn a_missing_log_is_an_empty_history() {
    let repository = FilesystemLinkHistoryRepository::new(Path::new("/nonexistent/http2scheme"));
    assert_eq!(repository.find_latest(100).await, Ok(Vec::new()));
}
