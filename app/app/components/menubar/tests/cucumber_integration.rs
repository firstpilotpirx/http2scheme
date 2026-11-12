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
use fakes::FakeLoginItem;
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;
use menubar::menubar_config::MenubarConfig;
use support::RecordingProgram;

/// The state one scenario builds up.
#[derive(Debug, cucumber::World)]
#[world(init = Self::new)]
pub struct MenubarWorld {
    /// The temporary home of the settings and logs folders and the recording opener.
    pub home: tempfile::TempDir,
    /// The recording `open` program.
    pub open: RecordingProgram,
    /// What the server printed.
    pub output: menubar::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway,
    /// The fake system login item.
    pub login_item: Arc<FakeLoginItem>,
    /// The running program.
    pub program: String,
    /// The menu as it read last: `(title, enabled, checked)`.
    pub items: Vec<(String, bool, bool)>,
}

impl MenubarWorld {
    fn new() -> Self {
        let home = tempfile::tempdir().unwrap();
        let open = RecordingProgram::new(home.path(), "open", 0);
        Self {
            home,
            open,
            output: MenubarConfig::server_output(),
            login_item: Arc::new(FakeLoginItem::new(LoginItemStateEnum::Off)),
            program: "/Applications/http2scheme.app/Contents/MacOS/http2scheme".to_owned(),
            items: Vec::new(),
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
