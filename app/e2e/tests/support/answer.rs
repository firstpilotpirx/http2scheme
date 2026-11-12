//! The last answer of the server.

/// The status and the body of the last answer, or a refused connection.
#[derive(Debug, Default)]
pub struct Answer {
    /// The status of the last answer.
    pub status: u16,
    /// The body of the last answer.
    pub body: String,
    /// Whether the last connection was refused.
    pub refused: bool,
}
