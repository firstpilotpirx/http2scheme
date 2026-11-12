//! A URI scheme name.

use std::fmt;

use super::link_rejected_error::LinkRejectedError;

/// A scheme name by RFC 3986, kept lower case.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemeNameValueObject(String);

impl SchemeNameValueObject {
    /// Parses `ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
    ///
    /// # Errors
    ///
    /// [`LinkRejectedError::NoScheme`] when the text is not a scheme name.
    pub fn parse(text: &str) -> Result<Self, LinkRejectedError> {
        let mut chars = text.chars();
        let first_is_letter = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
        let rest_valid = chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
        if first_is_letter && rest_valid {
            Ok(Self(text.to_ascii_lowercase()))
        } else {
            Err(LinkRejectedError::NoScheme)
        }
    }

    /// The lower-case name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SchemeNameValueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
#[path = "scheme_name_value_object_test.rs"]
mod scheme_name_value_object_test;
