//! The component's composition root: the menu, the first-launch offer and the application loop.

use std::sync::Arc;

use objc2::MainThreadMarker;

use crate::adapters::primary::statusmenu::appkit::app_kit_config::{AppKitConfig, ready};
use crate::adapters::primary::statusmenu::appkit::history_window_handler::HistoryWindowHandler;
use crate::adapters::primary::statusmenu::appkit::status_menu_handler::StatusMenuHandler;
use crate::adapters::secondary::loginitem::smappservice::sm_app_service_login_item_gateway::SmAppServiceLoginItemGateway;
use crate::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::use_cases::browse_history_use_case::BrowseHistoryUseCase;
use crate::core::application::use_cases::offer_login_item::OfferLoginItem as _;
use crate::core::application::use_cases::offer_login_item_use_case::OfferLoginItemUseCase;
use crate::core::application::use_cases::operate_menu_use_case::OperateMenuUseCase;
use crate::core::domain::app_folders_value_object::AppFoldersValueObject;
use crate::menubar_hooks::MenubarHooks;
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

    /// A root over the macOS adapters that starts the server and opens links through `hooks`.
    #[must_use]
    pub fn macos(setup: MenubarSetup, hooks: MenubarHooks) -> Self {
        let output = ServeOutputServerStateGateway::with_launcher(hooks.serve);
        let ports = MenubarPorts::macos(&setup, &output, hooks.replay);
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

    /// Starts the server through the app's launcher, as every launch does.
    pub fn start_server(&self) {
        ready(self.ports.switch.turn_on());
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

    /// Prepares the Link History window; it shows at [`HistoryWindowHandler::show`].
    #[must_use]
    pub fn history_window(&self, mtm: MainThreadMarker) -> HistoryWindowHandler {
        let use_case =
            BrowseHistoryUseCase::new(self.ports.history.clone(), self.ports.replay.clone());
        HistoryWindowHandler::install(mtm, Arc::new(use_case))
    }

    /// Puts the status item with its menu in the menu bar; Link History… calls `show_history`.
    #[must_use]
    pub fn status_menu(
        &self,
        mtm: MainThreadMarker,
        show_history: Box<dyn Fn()>,
    ) -> StatusMenuHandler {
        let folders =
            AppFoldersValueObject::new(self.setup.settings_dir.clone(), self.setup.log_dir.clone());
        let use_case = OperateMenuUseCase::new(
            self.setup.version.clone(),
            folders,
            self.ports.switch.clone(),
            self.ports.login_item.clone(),
            self.ports.opener.clone(),
        );
        StatusMenuHandler::install(mtm, Arc::new(use_case), show_history)
    }

    /// Shows the menu and runs the application loop until the app quits.
    pub fn run(&self, mtm: MainThreadMarker) {
        let history = std::rc::Rc::new(self.history_window(mtm));
        let shown = std::rc::Rc::clone(&history);
        let _menu = self.status_menu(mtm, Box::new(move || shown.show()));
        AppKitConfig::run(mtm);
    }
}

#[cfg(test)]
#[path = "menubar_config_test.rs"]
mod menubar_config_test;
