//! Builds the component's use cases and routers from what the app hands in.

use std::sync::Arc;

use axum::Router;

use crate::adapters::primary::linkopening::http::axum_config::AxumConfig;
use crate::adapters::primary::linkopening::http::open_link_handler::OpenLinkHandler;
use crate::adapters::primary::linkopening::http::open_link_response::OpenLinkResponse;
use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use crate::core::application::use_cases::open_link_use_case::OpenLinkUseCase;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

/// The secondary ports the open-link use case drives.
pub struct OpenLinkPorts {
    /// Opens scheme links.
    pub opener: Arc<dyn SchemeLinkOpenerService>,
    /// Closes the browser tabs.
    pub closer: Arc<dyn BrowserTabCloserService>,
    /// Records open attempts.
    pub attempts: Arc<dyn OpenAttemptRepository>,
}

/// The component's composition root.
#[derive(Debug, Clone, Copy, Default)]
pub struct LinkserverConfig;

impl LinkserverConfig {
    /// The HTTP router over the open-link use case built from the settings, the key and the ports.
    pub fn open_link_router(
        settings: ServerSettingsValueObject,
        key: AccessKeyValueObject,
        ports: OpenLinkPorts,
        http: &AxumConfig,
        pages: OpenLinkResponse,
    ) -> Router {
        let use_case =
            OpenLinkUseCase::new(settings, key, ports.opener, ports.closer, ports.attempts);
        http.router(OpenLinkHandler::new(Arc::new(use_case), pages))
    }
}
