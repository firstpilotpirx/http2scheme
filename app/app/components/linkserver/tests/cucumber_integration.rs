//! Slice and component scenarios: the world and the runner.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod fakes;
mod steps;
mod world;

use cucumber::World as _;
use fakes::Fakes;
use world::answer::Answer;
use world::printed::Printed;

/// The state one scenario builds up.
#[derive(Debug, cucumber::World)]
#[world(init = Self::new)]
pub struct LinkserverWorld {
    /// The schemes the settings allow.
    pub allowed: Vec<String>,
    /// Whether the settings close the tab.
    pub close_tab: bool,
    /// Whether the scenario must see one tab close tried.
    pub close_must_be_tried: bool,
    /// The fake secondary adapters.
    pub fakes: Fakes,
    /// The port of the running server.
    pub port: Option<u16>,
    /// The scheme link the last request carried.
    pub link: Option<String>,
    /// The last answer of the server.
    pub answer: Answer,
    /// The temporary home of the settings, the key, the logs and the agent definitions.
    pub home: Option<tempfile::TempDir>,
    /// What the last command printed.
    pub printed: Printed,
    /// The keys seen so far.
    pub keys: Vec<String>,
    /// A socket that holds a port.
    pub held: Option<std::net::TcpListener>,
    /// Stops the server running in the background.
    pub shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}

impl LinkserverWorld {
    fn new() -> Self {
        Self {
            allowed: vec!["idea".to_owned(), "goland".to_owned()],
            close_tab: true,
            close_must_be_tried: false,
            fakes: Fakes::default(),
            port: None,
            link: None,
            answer: Answer::default(),
            home: None,
            printed: Printed::default(),
            keys: Vec::new(),
            held: None,
            shutdown: None,
        }
    }
}

#[tokio::main]
async fn main() {
    LinkserverWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/features"))
        .await;
}
