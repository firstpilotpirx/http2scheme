//! The component's composition root: the menu, the first-launch offer and the application loop.

use std::sync::Arc;

use objc2::MainThreadMarker;

use crate::adapters::primary::statusmenu::appkit::app_kit_config::AppKitConfig;
use crate::adapters::primary::statusmenu::appkit::status_menu_handler::StatusMenuHandler;
use crate::adapters::secondary::loginitem::smappservice::sm_app_service_login_item_gateway::SmAppServiceLoginItemGateway;
use crate::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::use_cases::offer_login_item::OfferLoginItem as _;
use crate::core::application::use_cases::offer_login_item_use_case::OfferLoginItemUseCase;
use crate::core::application::use_cases::operate_menu_use_case::OperateMenuUseCase;
use crate::core::domain::app_folders_value_object::AppFoldersValueObject;
use crate::menubar_ports::MenubarPorts;
use crate::menubar_setup::MenubarSetup;

/// Builds the menu and its use cases from what the app hands in.
pub struct MenubarConfig {
    setup: MenubarSetup,
    ports: MenubarPorts,
    output: ServeOutputServerStateGateway,
}

impl MenubarConfig {
    /// A root over the setup and the ports.
    #[must_use]
    pub fn new(setup: MenubarSetup, ports: MenubarPorts) -> Self {
        Self {
            setup,
            ports,
            output: ServeOutputServerStateGateway::new(),
        }
    }

    /// A root over the macOS adapters, whose menu reads the server's state from [`Self::server_output`].
    #[must_use]
    pub fn macos(setup: MenubarSetup) -> Self {
        let output = ServeOutputServerStateGateway::new();
        let ports = MenubarPorts::macos(&setup, &output);
        Self {
            setup,
            ports,
            output,
        }
    }

    /// The writers the server prints to: `out()` for its output and `err()` for its errors.
    #[must_use]
    pub fn server_output(&self) -> ServeOutputServerStateGateway {
        self.output.clone()
    }

    /// Whether `program` runs from inside an app bundle.
    #[must_use]
    pub fn is_bundle_program(program: &std::path::Path) -> bool {
        SmAppServiceLoginItemGateway::is_bundle_program(program)
    }

    /// The main-thread marker, when called on the main thread.
    #[must_use]
    pub fn main_thread() -> Option<MainThreadMarker> {
        AppKitConfig::main_thread()
    }

    /// Adds the app to the login items at its first launch from Applications.
    ///
    /// # Errors
    ///
    /// When the login item or the first-launch record fails.
    pub async fn offer_login_item(&self) -> Result<(), MenuActionError> {
        OfferLoginItemUseCase::new(
            self.setup.program.clone(),
            self.ports.login_item.clone(),
            self.ports.first_launch.clone(),
        )
        .execute()
        .await
    }

    /// Puts the status item with its menu in the menu bar.
    #[must_use]
    pub fn status_menu(&self, mtm: MainThreadMarker) -> StatusMenuHandler {
        let folders =
            AppFoldersValueObject::new(self.setup.settings_dir.clone(), self.setup.log_dir.clone());
        let use_case = OperateMenuUseCase::new(
            self.setup.version.clone(),
            folders,
            self.ports.server.clone(),
            self.ports.login_item.clone(),
            self.ports.opener.clone(),
        );
        StatusMenuHandler::install(mtm, Arc::new(use_case))
    }

    /// Shows the menu and runs the application loop until the app quits.
    pub fn run(&self, mtm: MainThreadMarker) {
        let _menu = self.status_menu(mtm);
        AppKitConfig::run(mtm);
    }
}

#[cfg(test)]
#[path = "menubar_config_test.rs"]
mod menubar_config_test;
