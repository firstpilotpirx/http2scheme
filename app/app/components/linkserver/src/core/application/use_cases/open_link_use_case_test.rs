use std::sync::{Arc, Mutex};

use super::OpenLinkUseCase;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::repositories::open_attempt_repository::MockOpenAttemptRepository;
use crate::core::application::services::browser_tab_closer_service::MockBrowserTabCloserService;
use crate::core::application::services::scheme_link_opener_service::MockSchemeLinkOpenerService;
use crate::core::application::use_cases::open_link::OpenLink;
use crate::core::application::use_cases::open_link_command::OpenLinkCommand;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

struct Fixture {
    opener: MockSchemeLinkOpenerService,
    closer: MockBrowserTabCloserService,
    events: Arc<Mutex<Vec<LinkOpenAttemptedDomainEvent>>>,
    close_tab: bool,
}

impl Fixture {
    fn new() -> Self {
        Self { opener: MockSchemeLinkOpenerService::new(), closer: MockBrowserTabCloserService::new(), events: Arc::default(), close_tab: true }
    }

    fn build(self) -> (OpenLinkUseCase, Arc<Mutex<Vec<LinkOpenAttemptedDomainEvent>>>) {
        let mut attempts = MockOpenAttemptRepository::new();
        let sink = Arc::clone(&self.events);
        attempts.expect_record().returning(move |event| {
            sink.lock().unwrap().push(event.clone());
            Ok(())
        });
        let settings = ServerSettingsValueObject::new(PortValueObject::default(), SchemeAllowlistPolicy::defaults(), self.close_tab);
        let use_case = OpenLinkUseCase::new(
            settings,
            AccessKeyValueObject::from_bytes([7; 32]),
            Arc::new(self.opener),
            Arc::new(self.closer),
            Arc::new(attempts),
        );
        (use_case, self.events)
    }
}

fn right_key() -> Option<String> {
    Some("07".repeat(32))
}

fn command(key: Option<String>, link: &str) -> OpenLinkCommand {
    OpenLinkCommand::new(key, Some(link.to_owned()))
}

#[tokio::test]
async fn opens_an_allowed_link_closes_the_tab_and_records_it() {
    let mut fixture = Fixture::new();
    fixture.opener.expect_open().withf(|link| link.as_str() == "idea://open?line=17").times(1).returning(|_| Ok(()));
    fixture.closer.expect_close_open_tabs().withf(|port| port.value() == 7777).times(1).returning(|_| Ok(()));
    let (use_case, events) = fixture.build();
    let outcome = use_case.execute(command(right_key(), "idea://open?line=17")).await;
    assert_eq!(outcome, OpenOutcomeEnum::Opened);
    let recorded = events.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].link_text(), "idea://open?line=17");
    assert_eq!(recorded[0].outcome(), &OpenOutcomeEnum::Opened);
}

#[tokio::test]
async fn a_wrong_or_missing_key_opens_nothing() {
    for key in [Some("00".repeat(32)), None, Some(String::new())] {
        let (use_case, events) = Fixture::new().build();
        let outcome = use_case.execute(command(key, "idea://x")).await;
        assert_eq!(outcome, OpenOutcomeEnum::WrongKey);
        let recorded = events.lock().unwrap();
        assert_eq!(recorded[0].outcome(), &OpenOutcomeEnum::WrongKey);
        assert!(!format!("{recorded:?}").contains(&"07".repeat(32)));
    }
}

#[tokio::test]
async fn a_missing_link_is_rejected() {
    let (use_case, events) = Fixture::new().build();
    let outcome = use_case.execute(OpenLinkCommand::new(right_key(), None)).await;
    assert_eq!(outcome, OpenOutcomeEnum::Rejected(LinkRejectedError::Missing));
    assert_eq!(events.lock().unwrap()[0].link_text(), "");
}

#[tokio::test]
async fn a_malformed_link_is_rejected() {
    let (use_case, _) = Fixture::new().build();
    let outcome = use_case.execute(command(right_key(), "idea://a b")).await;
    assert_eq!(outcome, OpenOutcomeEnum::Rejected(LinkRejectedError::ForbiddenCharacter));
}

#[tokio::test]
async fn a_scheme_not_allowed_is_rejected() {
    let (use_case, events) = Fixture::new().build();
    let outcome = use_case.execute(command(right_key(), "slack://channel")).await;
    let expected = OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed { scheme: "slack".to_owned() });
    assert_eq!(outcome, expected);
    assert_eq!(events.lock().unwrap()[0].link_text(), "slack://channel");
}

#[tokio::test]
async fn an_opener_failure_is_open_failed_and_keeps_the_tab() {
    let mut fixture = Fixture::new();
    fixture.opener.expect_open().returning(|_| Err(OpenLinkError::OpenFailed { reason: "exit 1".to_owned() }));
    fixture.closer.expect_close_open_tabs().never();
    let (use_case, events) = fixture.build();
    assert_eq!(use_case.execute(command(right_key(), "idea://x")).await, OpenOutcomeEnum::OpenFailed);
    assert_eq!(events.lock().unwrap()[0].outcome(), &OpenOutcomeEnum::OpenFailed);
}

#[tokio::test]
async fn tab_closing_off_closes_no_tab() {
    let mut fixture = Fixture::new();
    fixture.close_tab = false;
    fixture.opener.expect_open().returning(|_| Ok(()));
    fixture.closer.expect_close_open_tabs().never();
    let (use_case, _) = fixture.build();
    assert_eq!(use_case.execute(command(right_key(), "idea://x")).await, OpenOutcomeEnum::Opened);
}

#[tokio::test]
async fn a_failed_tab_close_keeps_the_link_opened() {
    let mut fixture = Fixture::new();
    fixture.opener.expect_open().returning(|_| Ok(()));
    fixture.closer.expect_close_open_tabs().returning(|_| Err(OpenLinkError::TabCloseFailed { reason: "denied".to_owned() }));
    let (use_case, events) = fixture.build();
    assert_eq!(use_case.execute(command(right_key(), "idea://x")).await, OpenOutcomeEnum::Opened);
    assert_eq!(events.lock().unwrap()[0].outcome(), &OpenOutcomeEnum::Opened);
}

#[tokio::test]
async fn a_failed_record_keeps_the_outcome() {
    let mut opener = MockSchemeLinkOpenerService::new();
    opener.expect_open().returning(|_| Ok(()));
    let mut closer = MockBrowserTabCloserService::new();
    closer.expect_close_open_tabs().returning(|_| Ok(()));
    let mut attempts = MockOpenAttemptRepository::new();
    attempts.expect_record().returning(|_| Err(OpenLinkError::RecordFailed { reason: "full".to_owned() }));
    let use_case = OpenLinkUseCase::new(
        ServerSettingsValueObject::defaults(),
        AccessKeyValueObject::from_bytes([7; 32]),
        Arc::new(opener),
        Arc::new(closer),
        Arc::new(attempts),
    );
    assert_eq!(use_case.execute(command(right_key(), "idea://x")).await, OpenOutcomeEnum::Opened);
}
