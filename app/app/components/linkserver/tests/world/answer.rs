//! The last answer of the server.

/// The status and the body of the last answer.
#[derive(Debug, Default)]
pub struct Answer {
    /// The status of the last answer.
    pub status: u16,
    /// The body of the last answer.
    pub body: String,
}
