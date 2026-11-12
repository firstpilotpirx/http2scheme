//! E2E scenarios on the built binary: the world and the runner.
#![forbid(unsafe_code)]
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod steps;
mod support;

use cucumber::World as _;
use support::Home;
use support::answer::Answer;
use support::printed::Printed;
use tokio::process::Child;

/// The state one scenario builds up.
#[derive(Debug, cucumber::World)]
#[world(init = Self::new)]
pub struct E2eWorld {
    /// The home of the binary.
    pub home: Home,
    /// The schemes the settings allow.
    pub allowed: Vec<String>,
    /// Whether the settings close the tab.
    pub close_tab: bool,
    /// The port in the settings.
    pub port: Option<u16>,
    /// The server running in the background.
    pub server: Option<Child>,
    /// The scheme link of the last request or command.
    pub link: Option<String>,
    /// The last answer of the server.
    pub answer: Answer,
    /// What the last command printed.
    pub printed: Printed,
    /// The keys seen so far.
    pub keys: Vec<String>,
    /// A socket that holds a port.
    pub held: Option<std::net::TcpListener>,
}

impl E2eWorld {
    fn new() -> Self {
        Self {
            home: Home::new(),
            allowed: vec!["idea".to_owned(), "goland".to_owned()],
            close_tab: true,
            port: None,
            server: None,
            link: None,
            answer: Answer::default(),
            printed: Printed::default(),
            keys: Vec::new(),
            held: None,
        }
    }
}

#[tokio::main]
async fn main() {
    E2eWorld::cucumber()
        .max_concurrent_scenarios(4)
        .fail_on_skipped()
        .run_and_exit(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/features"))
        .await;
}
