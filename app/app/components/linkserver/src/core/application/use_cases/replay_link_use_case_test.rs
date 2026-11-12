use std::sync::{Arc, Mutex};

use super::ReplayLinkUseCase;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::repositories::open_attempt_repository::MockOpenAttemptRepository;
use crate::core::application::services::scheme_link_opener_service::MockSchemeLinkOpenerService;
use crate::core::application::use_cases::prepare_settings::MockPrepareSettings;
use crate::core::application::use_cases::replay_link::ReplayLink;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

type Events = Arc<Mutex<Vec<LinkOpenAttemptedDomainEvent>>>;

fn settings() -> MockPrepareSettings {
    let mut settings = MockPrepareSettings::new();
    settings.expect_execute().returning(|| {
        let values = ServerSettingsValueObject::new(
            PortValueObject::default(),
            SchemeAllowlistPolicy::defaults(),
            true,
        );
        Ok((values, AccessKeyValueObject::from_bytes([7; 32])))
    });
    settings
}

fn use_case(
    settings: MockPrepareSettings,
    opener: MockSchemeLinkOpenerService,
) -> (ReplayLinkUseCase, Events) {
    let events: Events = Arc::default();
    let sink = Arc::clone(&events);
    let mut attempts = MockOpenAttemptRepository::new();
    attempts.expect_record().returning(move |event| {
        sink.lock().unwrap().push(event.clone());
        Ok(())
    });
    let use_case = ReplayLinkUseCase::new(Arc::new(settings), Arc::new(opener), Arc::new(attempts));
    (use_case, events)
}

#[tokio::test]
async fn an_allowed_link_opens_and_is_logged() {
    let mut opener = MockSchemeLinkOpenerService::new();
    opener
        .expect_open()
        .withf(|link| link.as_str() == "idea://open?line=17")
        .times(1)
        .returning(|_| Ok(()));
    let (use_case, events) = use_case(settings(), opener);
    let outcome = use_case.execute("idea://open?line=17").await;
    assert_eq!(outcome, OpenOutcomeEnum::Opened);
    let recorded = events.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].link_text(), "idea://open?line=17");
    assert_eq!(recorded[0].outcome(), &OpenOutcomeEnum::Opened);
}

#[tokio::test]
async fn a_scheme_not_allowed_opens_nothing_and_is_logged() {
    let mut opener = MockSchemeLinkOpenerService::new();
    opener.expect_open().never();
    let (use_case, events) = use_case(settings(), opener);
    let outcome = use_case.execute("slack://channel").await;
    let expected = OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed {
        scheme: "slack".to_owned(),
    });
    assert_eq!(outcome, expected);
    assert_eq!(events.lock().unwrap()[0].outcome(), &expected);
}

#[tokio::test]
async fn a_malformed_link_opens_nothing() {
    let mut opener = MockSchemeLinkOpenerService::new();
    opener.expect_open().never();
    let (use_case, events) = use_case(settings(), opener);
    let outcome = use_case.execute("idea://a b").await;
    assert_eq!(
        outcome,
        OpenOutcomeEnum::Rejected(LinkRejectedError::ForbiddenCharacter)
    );
    assert_eq!(events.lock().unwrap()[0].link_text(), "idea://a b");
}

#[tokio::test]
async fn a_failed_open_is_reported() {
    let mut opener = MockSchemeLinkOpenerService::new();
    opener.expect_open().returning(|_| {
        Err(OpenLinkError::OpenFailed {
            reason: "exit 1".to_owned(),
        })
    });
    let (use_case, events) = use_case(settings(), opener);
    assert_eq!(
        use_case.execute("idea://x").await,
        OpenOutcomeEnum::OpenFailed
    );
    assert_eq!(
        events.lock().unwrap()[0].outcome(),
        &OpenOutcomeEnum::OpenFailed
    );
}

#[tokio::test]
async fn unreadable_settings_open_nothing() {
    let mut broken = MockPrepareSettings::new();
    broken.expect_execute().returning(|| {
        Err(SettingsError::Io {
            path: "settings.toml".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let mut opener = MockSchemeLinkOpenerService::new();
    opener.expect_open().never();
    let (use_case, _) = use_case(broken, opener);
    assert_eq!(
        use_case.execute("idea://x").await,
        OpenOutcomeEnum::OpenFailed
    );
}
