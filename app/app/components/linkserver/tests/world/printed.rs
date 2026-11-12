//! What the last command printed.

use crate::steps::home::SharedBuffer;

/// The output, the errors and the exit status of the last command.
#[derive(Debug, Default)]
pub struct Printed {
    /// What the last command printed.
    pub out: SharedBuffer,
    /// What the last command printed as errors.
    pub err: SharedBuffer,
    /// The exit status of the last command.
    pub code: Option<u8>,
}
