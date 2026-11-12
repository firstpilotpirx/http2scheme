//! The server's settings.

use super::port_value_object::PortValueObject;
use super::scheme_allowlist_policy::SchemeAllowlistPolicy;

/// Port, allowed schemes and the tab closing switch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerSettingsValueObject {
    port: PortValueObject,
    allowlist: SchemeAllowlistPolicy,
    close_tab: bool,
}

impl ServerSettingsValueObject {
    /// Settings from their parts.
    #[must_use]
    pub const fn new(port: PortValueObject, allowlist: SchemeAllowlistPolicy, close_tab: bool) -> Self {
        Self { port, allowlist, close_tab }
    }

    /// Port 7777, `idea` and `goland`, tab closing on.
    #[must_use]
    pub fn defaults() -> Self {
        Self::new(PortValueObject::default(), SchemeAllowlistPolicy::defaults(), true)
    }

    /// The port.
    #[must_use]
    pub const fn port(&self) -> PortValueObject {
        self.port
    }

    /// The allowed schemes.
    #[must_use]
    pub const fn allowlist(&self) -> &SchemeAllowlistPolicy {
        &self.allowlist
    }

    /// Whether the browser tab of an opened link is closed.
    #[must_use]
    pub const fn close_tab(&self) -> bool {
        self.close_tab
    }
}

#[cfg(test)]
#[path = "server_settings_value_object_test.rs"]
mod server_settings_value_object_test;
