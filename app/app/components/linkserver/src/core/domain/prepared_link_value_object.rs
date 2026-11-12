//! What an open link is made of.

use super::access_key_value_object::AccessKeyValueObject;
use super::port_value_object::PortValueObject;
use super::scheme_link_value_object::SchemeLinkValueObject;

/// The port, key and scheme link of one open link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedLinkValueObject {
    port: PortValueObject,
    key: AccessKeyValueObject,
    link: SchemeLinkValueObject,
}

impl PreparedLinkValueObject {
    /// The parts of an open link.
    #[must_use]
    pub const fn new(
        port: PortValueObject,
        key: AccessKeyValueObject,
        link: SchemeLinkValueObject,
    ) -> Self {
        Self { port, key, link }
    }

    /// The server port.
    #[must_use]
    pub const fn port(&self) -> PortValueObject {
        self.port
    }

    /// The secret key.
    #[must_use]
    pub const fn key(&self) -> &AccessKeyValueObject {
        &self.key
    }

    /// The scheme link.
    #[must_use]
    pub const fn link(&self) -> &SchemeLinkValueObject {
        &self.link
    }
}

#[cfg(test)]
#[path = "prepared_link_value_object_test.rs"]
mod prepared_link_value_object_test;
