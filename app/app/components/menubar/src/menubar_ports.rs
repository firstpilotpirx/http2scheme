//! The secondary ports of the component and their macOS adapters.

use std::sync::Arc;

use crate::adapters::secondary::firstlaunch::filesystem::filesystem_first_launch_repository::FilesystemFirstLaunchRepository;
use crate::adapters::secondary::linkhistory::filesystem::filesystem_link_history_repository::FilesystemLinkHistoryRepository;
use crate::adapters::secondary::linkreplay::inprocess::in_process_link_replay_gateway::InProcessLinkReplayGateway;
use crate::adapters::secondary::loginitem::smappservice::sm_app_service_login_item_gateway::SmAppServiceLoginItemGateway;
use crate::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use crate::core::application::repositories::first_launch_repository::FirstLaunchRepository;
use crate::core::application::repositories::link_history_repository::LinkHistoryRepository;
use crate::core::application::services::link_replay_service::LinkReplayService;
use crate::core::application::services::login_item_service::LoginItemService;
use crate::core::application::services::server_state_service::ServerStateService;
use crate::core::application::services::server_switch_service::ServerSwitchService;
use crate::menubar_hooks::LinkReplayer;
use crate::menubar_setup::MenubarSetup;

/// The secondary ports the menu's use cases drive.
#[derive(Clone)]
pub struct MenubarPorts {
    /// The server's state.
    pub server: Arc<dyn ServerStateService>,
    /// Turns the server off and on.
    pub switch: Arc<dyn ServerSwitchService>,
    /// The system's login item.
    pub login_item: Arc<dyn LoginItemService>,
    /// The first-launch record.
    pub first_launch: Arc<dyn FirstLaunchRepository>,
    /// The request log, read back.
    pub history: Arc<dyn LinkHistoryRepository>,
    /// Opens a link again.
    pub replay: Arc<dyn LinkReplayService>,
}

impl MenubarPorts {
    /// The macOS adapters for the setup, reading and switching the server through `server_output`
    /// and replaying links through `replay`.
    #[must_use]
    pub fn macos(
        setup: &MenubarSetup,
        server_output: &ServeOutputServerStateGateway,
        replay: LinkReplayer,
    ) -> Self {
        Self {
            server: Arc::new(server_output.clone()),
            switch: Arc::new(server_output.clone()),
            login_item: Arc::new(SmAppServiceLoginItemGateway::new(&setup.program)),
            first_launch: Arc::new(FilesystemFirstLaunchRepository::new(&setup.settings_dir)),
            history: Arc::new(FilesystemLinkHistoryRepository::new(&setup.log_dir)),
            replay: Arc::new(InProcessLinkReplayGateway::new(replay)),
        }
    }
}

#[cfg(test)]
#[path = "menubar_ports_test.rs"]
mod menubar_ports_test;
