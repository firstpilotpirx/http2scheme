use std::time::{Duration, UNIX_EPOCH};

use super::{escape, rfc3339, to_line};
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

#[test]
fn formats_known_times() {
    assert_eq!(rfc3339(UNIX_EPOCH), "1970-01-01T00:00:00Z");
    assert_eq!(
        rfc3339(UNIX_EPOCH + Duration::from_secs(951_782_401)),
        "2000-02-29T00:00:01Z"
    );
    assert_eq!(
        rfc3339(UNIX_EPOCH + Duration::from_secs(1_763_000_000)),
        "2025-11-13T02:13:20Z"
    );
    assert_eq!(
        rfc3339(UNIX_EPOCH + Duration::from_secs(4_102_444_799)),
        "2099-12-31T23:59:59Z"
    );
}

#[test]
fn escapes_control_characters_and_backslashes() {
    assert_eq!(escape("a\nb\r\tc\\d\u{7}"), "a\\nb\\r\\tc\\\\d\\u{7}");
    assert_eq!(escape("idea://x?y=1"), "idea://x?y=1");
}

#[test]
fn a_line_has_time_result_and_link() {
    let event = LinkOpenAttemptedDomainEvent::new(
        UNIX_EPOCH,
        "slack://x".to_owned(),
        OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed {
            scheme: "slack".to_owned(),
        }),
    );
    assert_eq!(
        to_line(&event),
        "1970-01-01T00:00:00Z\tscheme not allowed\tslack://x"
    );
}

#[test]
fn an_unreadable_link_line_holds_its_reason() {
    let event = LinkOpenAttemptedDomainEvent::new(
        UNIX_EPOCH,
        LinkRejectedError::Missing.to_string(),
        OpenOutcomeEnum::Rejected(LinkRejectedError::Missing),
    );
    assert_eq!(
        to_line(&event),
        "1970-01-01T00:00:00Z\tmalformed link\tthe link is missing"
    );
}
