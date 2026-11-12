//! Port: asking the server whether it runs.

use async_trait::async_trait;

use crate::core::domain::port_value_object::PortValueObject;

/// Asks the health address on this port.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ServerHealthService: Send + Sync {
    /// Whether the server on this port answers.
    async fn is_answering(&self, port: PortValueObject) -> bool;
}
