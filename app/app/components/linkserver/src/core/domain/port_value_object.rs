//! The server's TCP port.

const LOWEST: u16 = 1024;
const DEFAULT: u16 = 7777;

/// A port in 1024–65535.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortValueObject(u16);

impl PortValueObject {
    /// Accepts 1024–65535.
    #[must_use]
    pub const fn new(value: u16) -> Option<Self> {
        if value >= LOWEST {
            Some(Self(value))
        } else {
            None
        }
    }

    /// The port number.
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl Default for PortValueObject {
    fn default() -> Self {
        Self(DEFAULT)
    }
}

#[cfg(test)]
#[path = "port_value_object_test.rs"]
mod port_value_object_test;
