use std::path::PathBuf;
use std::time::SystemTime;

use super::FilesystemOpenAttemptRepository;
use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

#[tokio::test]
async fn an_unwritable_file_is_a_record_failure() {
    let repository = FilesystemOpenAttemptRepository::new(PathBuf::from("/dev/null/requests.log"));
    let event = LinkOpenAttemptedDomainEvent::new(
        SystemTime::UNIX_EPOCH,
        "idea://x".to_owned(),
        OpenOutcomeEnum::Opened,
    );
    assert!(repository.record(&event).await.is_err());
}
