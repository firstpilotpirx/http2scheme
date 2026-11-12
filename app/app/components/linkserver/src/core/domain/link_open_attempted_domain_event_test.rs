use std::time::{Duration, UNIX_EPOCH};

use super::LinkOpenAttemptedDomainEvent;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

#[test]
fn keeps_time_link_text_and_outcome() {
    let at = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let event =
        LinkOpenAttemptedDomainEvent::new(at, "idea://x".to_owned(), OpenOutcomeEnum::Opened);
    assert_eq!(event.at(), at);
    assert_eq!(event.link_text(), "idea://x");
    assert_eq!(event.outcome(), &OpenOutcomeEnum::Opened);
}
