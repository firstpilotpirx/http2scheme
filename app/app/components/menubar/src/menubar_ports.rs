//! The secondary ports of the component and their macOS adapters.

use std::sync::Arc;

use crate::adapters::secondary::firstlaunch::filesystem::filesystem_first_launch_repository::FilesystemFirstLaunchRepository;
use crate::adapters::secondary::folders::launchservices::launch_services_folder_opener_gateway::LaunchServicesFolderOpenerGateway;
use crate::adapters::secondary::loginitem::smappservice::sm_app_service_login_item_gateway::SmAppServiceLoginItemGateway;
use crate::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use crate::core::application::repositories::first_launch_repository::FirstLaunchRepository;
use crate::core::application::services::folder_opener_service::FolderOpenerService;
use crate::core::application::services::login_item_service::LoginItemService;
use crate::core::application::services::server_state_service::ServerStateService;
use crate::menubar_setup::MenubarSetup;

/// The secondary ports the menu's use cases drive.
#[derive(Clone)]
pub struct MenubarPorts {
    /// The server's state.
    pub server: Arc<dyn ServerStateService>,
    /// The system's login item.
    pub login_item: Arc<dyn LoginItemService>,
    /// Shows folders.
    pub opener: Arc<dyn FolderOpenerService>,
    /// The first-launch record.
    pub first_launch: Arc<dyn FirstLaunchRepository>,
}

impl MenubarPorts {
    /// The macOS adapters for the setup, reading the server's state from `server_output`.
    #[must_use]
    pub fn macos(setup: &MenubarSetup, server_output: &ServeOutputServerStateGateway) -> Self {
        Self {
            server: Arc::new(server_output.clone()),
            login_item: Arc::new(SmAppServiceLoginItemGateway::new(&setup.program)),
            opener: Arc::new(LaunchServicesFolderOpenerGateway::new(
                setup.open_program.clone(),
            )),
            first_launch: Arc::new(FilesystemFirstLaunchRepository::new(&setup.settings_dir)),
        }
    }
}

#[cfg(test)]
#[path = "menubar_ports_test.rs"]
mod menubar_ports_test;
