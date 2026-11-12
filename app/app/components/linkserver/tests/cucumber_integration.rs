//! Slice and component scenarios: the world and the runner.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod fakes;
mod steps;

use std::sync::Arc;

use cucumber::World as _;
use fakes::{FakeCloser, FakeHealth, FakeLog, FakeOpener, FakeSupervisor};
use steps::home::SharedBuffer;

/// The state one scenario builds up.
#[derive(Debug, cucumber::World)]
#[world(init = Self::new)]
pub struct LinkserverWorld {
    /// The schemes the settings allow.
    pub allowed: Vec<String>,
    /// Whether the settings close the tab.
    pub close_tab: bool,
    /// The link opener.
    pub opener: Arc<FakeOpener>,
    /// The tab closer.
    pub closer: Arc<FakeCloser>,
    /// The request log.
    pub log: Arc<FakeLog>,
    /// The port of the running server.
    pub port: Option<u16>,
    /// The scheme link the last request carried.
    pub link: Option<String>,
    /// The status of the last answer.
    pub status: u16,
    /// The body of the last answer.
    pub body: String,
    /// The temporary home of the settings, the key, the logs and the agent definitions.
    pub home: Option<tempfile::TempDir>,
    /// The agent supervisor.
    pub supervisor: Arc<FakeSupervisor>,
    /// The server health check.
    pub health: Arc<FakeHealth>,
    /// What the last command printed.
    pub out: SharedBuffer,
    /// What the last command printed as errors.
    pub err: SharedBuffer,
    /// The exit status of the last command.
    pub code: Option<u8>,
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
            opener: Arc::default(),
            closer: Arc::default(),
            log: Arc::default(),
            port: None,
            link: None,
            status: 0,
            body: String::new(),
            home: None,
            supervisor: Arc::default(),
            health: Arc::default(),
            out: SharedBuffer::default(),
            err: SharedBuffer::default(),
            code: None,
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
