//! The History tab part of the world.

/// The History tab as it read last.
#[derive(Debug, Default)]
pub struct HistorySeen {
    /// The rows: `[time, result, link]`.
    pub rows: Vec<[String; 3]>,
    /// The line shown in place of rows.
    pub note: Option<String>,
}
