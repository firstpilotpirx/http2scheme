//! Port: bringing back the app the user left for the browser.

use async_trait::async_trait;

use crate::core::application::exceptions::open_link_error::OpenLinkError;

/// Changes which app the app switcher lists right behind the app a link opens.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AppOrderService: Send + Sync {
    /// Brings to the front the app that was in front before the browser, when a browser is in
    /// front; changes nothing otherwise.
    async fn bring_back_left_app(&self) -> Result<(), OpenLinkError>;
}

#[cfg(test)]
#[path = "app_order_service_test.rs"]
mod app_order_service_test;
