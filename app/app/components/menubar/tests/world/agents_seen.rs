//! The Agents tab part of the world.

/// The Agents tab and the general pasteboard as they read last.
#[derive(Debug, Default)]
pub struct AgentsSeen {
    /// The row names.
    pub names: [String; 3],
    /// The lines under the rows.
    pub lines: [String; 3],
    /// The install button's title.
    pub button: String,
    /// The general pasteboard's text.
    pub clipboard: Option<String>,
}
