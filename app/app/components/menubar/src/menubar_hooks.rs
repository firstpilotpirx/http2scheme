//! What the app hands the component to drive its server: the launcher and the replayer.

use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::sync::Arc;

/// Completes when the server must stop.
pub type ServeShutdown = Pin<Box<dyn Future<Output = ()> + Send>>;
/// Where the server prints: its standard output, then its error output.
pub type ServeOutput = Box<dyn Write + Send>;
/// Starts the server printing to the two outputs, until its shutdown completes; it returns at once.
pub type ServeLauncher = Arc<dyn Fn(ServeShutdown, ServeOutput, ServeOutput) + Send + Sync>;
/// Opens a link again and answers the result word; it returns when the attempt is logged.
pub type LinkReplayer = Arc<dyn Fn(&str) -> String + Send + Sync>;

/// The calls into the server the app runs; `menubar` knows nothing else of it.
#[derive(Clone)]
pub struct MenubarHooks {
    /// Starts the server.
    pub serve: ServeLauncher,
    /// Opens a link of the Link History window again.
    pub replay: LinkReplayer,
}

#[cfg(test)]
#[path = "menubar_hooks_test.rs"]
mod menubar_hooks_test;
