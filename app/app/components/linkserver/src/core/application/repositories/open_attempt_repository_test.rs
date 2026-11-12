use std::sync::Arc;

use std::time::SystemTime;

use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

use super::{MockOpenAttemptRepository, OpenAttemptRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockOpenAttemptRepository::new();
    mock.expect_record().times(1).returning(|_| Ok(()));
    let port: Arc<dyn OpenAttemptRepository> = Arc::new(mock);
    let event = LinkOpenAttemptedDomainEvent::new(
        SystemTime::UNIX_EPOCH,
        "idea://x".to_owned(),
        OpenOutcomeEnum::Opened,
    );
    assert!(port.record(&event).await.is_ok());
}
