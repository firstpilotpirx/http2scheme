//! Port: the macOS link opener.

use async_trait::async_trait;

use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

/// Hands a scheme link, unchanged, to the app that registered its scheme.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SchemeLinkOpenerService: Send + Sync {
    /// Opens the link.
    async fn open(&self, link: &SchemeLinkValueObject) -> Result<(), OpenLinkError>;
}

#[cfg(test)]
#[path = "scheme_link_opener_service_test.rs"]
mod scheme_link_opener_service_test;
