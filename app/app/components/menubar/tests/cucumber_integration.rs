//! Slice scenarios of the menu bar app: the world and the runner, on the main thread (`AppKit` needs it).
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod fakes;
mod steps;
mod support;

use std::sync::Arc;

use cucumber::World as _;
use fakes::{FakeLoginItem, FakeReplayer, FakeServer};
use menubar::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;

/// The state one scenario builds up.
#[derive(Debug, cucumber::World)]
#[world(init = Self::new)]
pub struct MenubarWorld {
    /// The temporary home of the settings and logs folders.
    pub home: tempfile::TempDir,
    /// What the server printed.
    pub output: ServeOutputServerStateGateway,
    /// The server the app's launcher starts.
    pub server: FakeServer,
    /// The app's replayer.
    pub replayer: FakeReplayer,
    /// The fake system login item.
    pub login_item: Arc<FakeLoginItem>,
    /// The running program.
    pub program: String,
    /// The History tab as it read last: `[time, result, link]`.
    pub rows: Vec<[String; 3]>,
    /// The line the History tab showed in place of rows.
    pub note: Option<String>,
    /// The menu as it read last: `(title, enabled, checked)`.
    pub items: Vec<(String, bool, bool)>,
    /// The Settings window's selected tab as it read last.
    pub tab: String,
    /// The Serve links and Start at login switches as they read last.
    pub switches: [bool; 2],
    /// The lines under the two switches as they read last.
    pub lines: [String; 2],
}

impl MenubarWorld {
    fn new() -> Self {
        let home = tempfile::tempdir().unwrap();
        let server = FakeServer::default();
        let replayer = FakeReplayer::new(home.path().join("logs/requests.log"));
        Self {
            output: ServeOutputServerStateGateway::with_launcher(server.launcher(7777)),
            server,
            replayer,
            rows: Vec::new(),
            note: None,
            home,
            login_item: Arc::new(FakeLoginItem::new(LoginItemStateEnum::Off)),
            program: "/Applications/http2scheme.app/Contents/MacOS/http2scheme".to_owned(),
            items: Vec::new(),
            tab: String::new(),
            switches: [false; 2],
            lines: [String::new(), String::new()],
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    MenubarWorld::cucumber()
        .max_concurrent_scenarios(1)
        .fail_on_skipped()
        .run_and_exit(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/features"))
        .await;
}
