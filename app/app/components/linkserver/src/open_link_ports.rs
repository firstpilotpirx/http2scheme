//! The secondary ports the open-link use case drives.

use std::sync::Arc;

use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::application::services::app_order_service::AppOrderService;
use crate::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;

/// The secondary ports the open-link use case drives.
pub struct OpenLinkPorts {
    /// Opens scheme links.
    pub opener: Arc<dyn SchemeLinkOpenerService>,
    /// Closes the browser tabs.
    pub closer: Arc<dyn BrowserTabCloserService>,
    /// Brings back the app left for the browser.
    pub order: Arc<dyn AppOrderService>,
    /// Records open attempts.
    pub attempts: Arc<dyn OpenAttemptRepository>,
}

#[cfg(test)]
#[path = "open_link_ports_test.rs"]
mod open_link_ports_test;
