//! Port: closing the browser tabs the open links left.

use async_trait::async_trait;

use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::domain::port_value_object::PortValueObject;

/// Closes the running browsers' tabs that show the server's open address.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BrowserTabCloserService: Send + Sync {
    /// Starts closing the open-address tabs of the server on this port.
    async fn close_open_tabs(&self, port: PortValueObject) -> Result<(), OpenLinkError>;
}

#[cfg(test)]
#[path = "browser_tab_closer_service_test.rs"]
mod browser_tab_closer_service_test;
