//! What the last command printed.

/// The output, the errors and the exit status of the last command.
#[derive(Debug, Default)]
pub struct Printed {
    /// The exit status of the last command.
    pub code: Option<i32>,
    /// What the last command printed.
    pub out: String,
    /// What the last command printed as errors.
    pub err: String,
}
