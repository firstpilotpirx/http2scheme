//! One open request, as the request log records it.

use std::time::SystemTime;

use super::open_outcome_enum::OpenOutcomeEnum;

/// Time, link text (or the reason it was unreadable) and outcome. Never the key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkOpenAttemptedDomainEvent {
    at: SystemTime,
    link_text: String,
    outcome: OpenOutcomeEnum,
}

impl LinkOpenAttemptedDomainEvent {
    /// The event's parts.
    #[must_use]
    pub const fn new(at: SystemTime, link_text: String, outcome: OpenOutcomeEnum) -> Self {
        Self {
            at,
            link_text,
            outcome,
        }
    }

    /// When the request came.
    #[must_use]
    pub const fn at(&self) -> SystemTime {
        self.at
    }

    /// The link text, or why it was unreadable.
    #[must_use]
    pub fn link_text(&self) -> &str {
        &self.link_text
    }

    /// The outcome.
    #[must_use]
    pub const fn outcome(&self) -> &OpenOutcomeEnum {
        &self.outcome
    }
}

#[cfg(test)]
#[path = "link_open_attempted_domain_event_test.rs"]
mod link_open_attempted_domain_event_test;
