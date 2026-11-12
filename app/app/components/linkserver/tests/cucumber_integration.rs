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
use fakes::{FakeCloser, FakeLog, FakeOpener};

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
