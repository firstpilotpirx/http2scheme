//! A custom URL scheme link, kept byte for byte.

use super::link_rejected_error::LinkRejectedError;
use super::scheme_name_value_object::SchemeNameValueObject;

const MAX_CHARACTERS: usize = 4096;

/// A well-formed scheme link: 1–4096 characters, no space or control
/// character, and a valid scheme before the first colon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeLinkValueObject {
    text: String,
    scheme: SchemeNameValueObject,
}

impl SchemeLinkValueObject {
    /// Parses the link without changing its text.
    ///
    /// # Errors
    ///
    /// The [`LinkRejectedError`] naming the first broken rule.
    pub fn parse(text: &str) -> Result<Self, LinkRejectedError> {
        if text.is_empty() {
            return Err(LinkRejectedError::Empty);
        }
        if text.chars().count() > MAX_CHARACTERS {
            return Err(LinkRejectedError::TooLong);
        }
        if text.chars().any(|c| c == ' ' || c.is_control()) {
            return Err(LinkRejectedError::ForbiddenCharacter);
        }
        let (scheme, _) = text.split_once(':').ok_or(LinkRejectedError::NoScheme)?;
        Ok(Self {
            text: text.to_owned(),
            scheme: SchemeNameValueObject::parse(scheme)?,
        })
    }

    /// The exact link text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The link's scheme, lower case.
    #[must_use]
    pub const fn scheme(&self) -> &SchemeNameValueObject {
        &self.scheme
    }
}

#[cfg(test)]
#[path = "scheme_link_value_object_test.rs"]
mod scheme_link_value_object_test;
