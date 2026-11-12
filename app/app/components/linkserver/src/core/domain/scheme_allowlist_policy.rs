//! Which schemes may open.

use super::link_rejected_error::LinkRejectedError;
use super::scheme_link_value_object::SchemeLinkValueObject;
use super::scheme_name_value_object::SchemeNameValueObject;

const NEVER_ALLOWED: [&str; 3] = ["http", "https", "file"];
const DEFAULT_SCHEMES: [&str; 2] = ["idea", "goland"];

/// The user's allowed schemes; `http`, `https` and `file` never pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeAllowlistPolicy(Vec<SchemeNameValueObject>);

impl SchemeAllowlistPolicy {
    /// A policy over the given schemes.
    #[must_use]
    pub const fn new(schemes: Vec<SchemeNameValueObject>) -> Self {
        Self(schemes)
    }

    /// `idea` and `goland`.
    #[must_use]
    pub fn defaults() -> Self {
        Self(DEFAULT_SCHEMES.iter().filter_map(|n| SchemeNameValueObject::parse(n).ok()).collect())
    }

    /// The listed schemes.
    #[must_use]
    pub fn schemes(&self) -> &[SchemeNameValueObject] {
        &self.0
    }

    /// Permits the link when its scheme is listed and not a web or file scheme.
    ///
    /// # Errors
    ///
    /// [`LinkRejectedError::SchemeNotAllowed`] naming the scheme.
    pub fn check(&self, link: &SchemeLinkValueObject) -> Result<(), LinkRejectedError> {
        let scheme = link.scheme();
        let never = NEVER_ALLOWED.contains(&scheme.as_str());
        if !never && self.0.contains(scheme) {
            Ok(())
        } else {
            Err(LinkRejectedError::SchemeNotAllowed { scheme: scheme.to_string() })
        }
    }
}

#[cfg(test)]
#[path = "scheme_allowlist_policy_test.rs"]
mod scheme_allowlist_policy_test;
