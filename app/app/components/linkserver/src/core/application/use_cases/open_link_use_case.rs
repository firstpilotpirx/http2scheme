//! Opens an allowed scheme link from a keyed request.

use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;

use super::open_link::OpenLink;
use super::open_link_command::OpenLinkCommand;
use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::application::services::app_order_service::AppOrderService;
use crate::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

/// Checks the key and the link, brings back the app left for the browser, opens the link, closes
/// its tab and logs the request.
pub struct OpenLinkUseCase {
    settings: ServerSettingsValueObject,
    key: AccessKeyValueObject,
    opener: Arc<dyn SchemeLinkOpenerService>,
    closer: Arc<dyn BrowserTabCloserService>,
    order: Option<Arc<dyn AppOrderService>>,
    attempts: Arc<dyn OpenAttemptRepository>,
}

impl OpenLinkUseCase {
    /// A use case over the settings, the key and its ports.
    #[must_use]
    pub fn new(
        settings: ServerSettingsValueObject,
        key: AccessKeyValueObject,
        opener: Arc<dyn SchemeLinkOpenerService>,
        closer: Arc<dyn BrowserTabCloserService>,
        attempts: Arc<dyn OpenAttemptRepository>,
    ) -> Self {
        Self {
            settings,
            key,
            opener,
            closer,
            order: None,
            attempts,
        }
    }

    /// The same use case, bringing back the app left for the browser before each link opens.
    #[must_use]
    pub fn with_app_order(mut self, order: Arc<dyn AppOrderService>) -> Self {
        self.order = Some(order);
        self
    }

    fn admit(&self, command: &OpenLinkCommand) -> Result<SchemeLinkValueObject, OpenOutcomeEnum> {
        let key_ok = command
            .key()
            .is_some_and(|candidate| self.key.matches(candidate));
        if !key_ok {
            return Err(OpenOutcomeEnum::WrongKey);
        }
        let text = command
            .link()
            .ok_or(OpenOutcomeEnum::Rejected(LinkRejectedError::Missing))?;
        let link = SchemeLinkValueObject::parse(text).map_err(OpenOutcomeEnum::Rejected)?;
        self.settings
            .allowlist()
            .check(&link)
            .map_err(OpenOutcomeEnum::Rejected)?;
        Ok(link)
    }

    async fn open(&self, link: &SchemeLinkValueObject) -> OpenOutcomeEnum {
        if let Some(order) = &self.order {
            let _ = order.bring_back_left_app().await;
        }
        if self.opener.open(link).await.is_err() {
            return OpenOutcomeEnum::OpenFailed;
        }
        if self.settings.close_tab() {
            let _ = self.closer.close_open_tabs(self.settings.port()).await;
        }
        OpenOutcomeEnum::Opened
    }
}

#[async_trait]
impl OpenLink for OpenLinkUseCase {
    async fn execute(&self, command: OpenLinkCommand) -> OpenOutcomeEnum {
        let outcome = match self.admit(&command) {
            Ok(link) => self.open(&link).await,
            Err(refused) => refused,
        };
        let event = LinkOpenAttemptedDomainEvent::new(
            SystemTime::now(),
            logged_link(command.link()),
            outcome.clone(),
        );
        let _ = self.attempts.record(&event).await;
        outcome
    }
}

/// The link as the request log keeps it, or the reason there is none.
fn logged_link(link: Option<&str>) -> String {
    match link {
        Some(link) if !link.is_empty() => link.to_owned(),
        Some(_) => LinkRejectedError::Empty.to_string(),
        None => LinkRejectedError::Missing.to_string(),
    }
}

#[cfg(test)]
#[path = "open_link_use_case_test.rs"]
mod open_link_use_case_test;
