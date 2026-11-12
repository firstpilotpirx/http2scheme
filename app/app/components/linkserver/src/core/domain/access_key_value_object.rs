//! The secret key every open request must carry.

use std::fmt;
use std::fmt::Write as _;

use subtle::ConstantTimeEq;

const KEY_BYTES: usize = 32;
const HEX_LENGTH: usize = KEY_BYTES * 2;

/// 256 secret bits, written as 64 lower-case hex characters.
#[derive(Clone, PartialEq, Eq)]
pub struct AccessKeyValueObject(String);

impl AccessKeyValueObject {
    /// Builds the key from 32 random bytes.
    #[must_use]
    pub fn from_bytes(bytes: [u8; KEY_BYTES]) -> Self {
        let mut hex = String::with_capacity(HEX_LENGTH);
        for byte in bytes {
            let _ = write!(hex, "{byte:02x}");
        }
        Self(hex)
    }

    /// Reads a key file's text; one trailing line break is allowed.
    #[must_use]
    pub fn parse_hex(text: &str) -> Option<Self> {
        let hex = text.strip_suffix('\n').unwrap_or(text);
        let valid = hex.len() == HEX_LENGTH
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        valid.then(|| Self(hex.to_owned()))
    }

    /// The key as 64 hex characters.
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.0.clone()
    }

    /// Compares in constant time; a candidate of another length is compared
    /// against a dummy of the right length so the time does not depend on it.
    #[must_use]
    pub fn matches(&self, candidate: &str) -> bool {
        let expected = self.0.as_bytes();
        let same_length = candidate.len() == expected.len();
        let dummy = [0u8; HEX_LENGTH];
        let compared = if same_length {
            candidate.as_bytes()
        } else {
            &dummy[..]
        };
        bool::from(expected.ct_eq(compared)) && same_length
    }
}

impl fmt::Debug for AccessKeyValueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AccessKey(****)")
    }
}

#[cfg(test)]
#[path = "access_key_value_object_test.rs"]
mod access_key_value_object_test;
