//! What the app hands the component to drive its server: the launcher, the replayer and the skill.

use std::future::Future;
use std::io::Write;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;

/// Completes when the server must stop.
pub type ServeShutdown = Pin<Box<dyn Future<Output = ()> + Send>>;
/// Where the server prints: its standard output, then its error output.
pub type ServeOutput = Box<dyn Write + Send>;
/// Starts the server printing to the two outputs, until its shutdown completes; it returns at once.
pub type ServeLauncher = Arc<dyn Fn(ServeShutdown, ServeOutput, ServeOutput) + Send + Sync>;
/// Opens a link again and answers the result word; it returns when the attempt is logged.
pub type LinkReplayer = Arc<dyn Fn(&str) -> String + Send + Sync>;
/// Answers the agent skill text, the same the skill command prints.
pub type SkillText = Arc<dyn Fn() -> String + Send + Sync>;
/// Inspects or installs the skill in a Claude Code folder; the reason when it fails.
pub type SkillAction = Arc<dyn Fn(&Path) -> Result<SkillPlaceValueObject, String> + Send + Sync>;

/// The calls into the server the app runs; `menubar` knows nothing else of it.
#[derive(Clone)]
pub struct MenubarHooks {
    /// Starts the server.
    pub serve: ServeLauncher,
    /// Opens a link of the History tab again.
    pub replay: LinkReplayer,
    /// The agent skill text.
    pub skill_text: SkillText,
    /// The skill's place and state in a folder.
    pub inspect_skill: SkillAction,
    /// Writes the skill into a folder.
    pub install_skill: SkillAction,
}

#[cfg(test)]
#[path = "menubar_hooks_test.rs"]
mod menubar_hooks_test;
