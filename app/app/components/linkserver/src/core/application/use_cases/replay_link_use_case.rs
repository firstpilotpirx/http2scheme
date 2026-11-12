//! Opens a scheme link from the link history again: no key, no browser tab.

use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;

use super::prepare_settings::PrepareSettings;
use super::replay_link::ReplayLink;
use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

/// Checks the link against the allowed schemes, opens it and logs the attempt.
pub struct ReplayLinkUseCase {
    settings: Arc<dyn PrepareSettings>,
    opener: Arc<dyn SchemeLinkOpenerService>,
    attempts: Arc<dyn OpenAttemptRepository>,
}

impl ReplayLinkUseCase {
    /// A use case reading the settings through `settings`.
    #[must_use]
    pub fn new(
        settings: Arc<dyn PrepareSettings>,
        opener: Arc<dyn SchemeLinkOpenerService>,
        attempts: Arc<dyn OpenAttemptRepository>,
    ) -> Self {
        Self {
            settings,
            opener,
            attempts,
        }
    }

    async fn outcome(&self, text: &str) -> OpenOutcomeEnum {
        let Ok((settings, _)) = self.settings.execute().await else {
            return OpenOutcomeEnum::OpenFailed;
        };
        let checked = SchemeLinkValueObject::parse(text)
            .and_then(|link| settings.allowlist().check(&link).map(|()| link));
        match checked {
            Ok(link) if self.opener.open(&link).await.is_ok() => OpenOutcomeEnum::Opened,
            Ok(_) => OpenOutcomeEnum::OpenFailed,
            Err(reason) => OpenOutcomeEnum::Rejected(reason),
        }
    }
}

#[async_trait]
impl ReplayLink for ReplayLinkUseCase {
    async fn execute(&self, link: &str) -> OpenOutcomeEnum {
        let outcome = self.outcome(link).await;
        let event =
            LinkOpenAttemptedDomainEvent::new(SystemTime::now(), link.to_owned(), outcome.clone());
        let _ = self.attempts.record(&event).await;
        outcome
    }
}

#[cfg(test)]
#[path = "replay_link_use_case_test.rs"]
mod replay_link_use_case_test;
