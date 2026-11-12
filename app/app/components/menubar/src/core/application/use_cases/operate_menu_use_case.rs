//! Shows the menu's state and runs what the user chooses in it.

use async_trait::async_trait;
use std::sync::Arc;

use super::operate_menu::OperateMenu;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::login_item_service::LoginItemService;
use crate::core::application::services::server_switch_service::ServerSwitchService;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;

/// The menu's use case over the server's state and switch, and the login item.
pub struct OperateMenuUseCase {
    version: String,
    server: Arc<dyn ServerSwitchService>,
    login_item: Arc<dyn LoginItemService>,
}

impl OperateMenuUseCase {
    /// A use case for the app `version`.
    #[must_use]
    pub fn new(
        version: String,
        server: Arc<dyn ServerSwitchService>,
        login_item: Arc<dyn LoginItemService>,
    ) -> Self {
        Self {
            version,
            server,
            login_item,
        }
    }

    async fn toggle(&self) -> Result<(), MenuActionError> {
        match self.login_item.status().await {
            LoginItemStateEnum::On => self.login_item.unregister().await,
            LoginItemStateEnum::Off => self.login_item.register().await,
            LoginItemStateEnum::NeedsApproval => self.login_item.open_system_settings().await,
            LoginItemStateEnum::Unavailable => Ok(()),
        }
    }

    async fn toggle_server(&self) {
        if self.server.current().await.is_serving() {
            self.server.turn_off().await;
        } else {
            self.server.turn_on().await;
        }
    }
}

#[async_trait]
impl OperateMenu for OperateMenuUseCase {
    async fn execute(
        &self,
        command: MenuCommandEnum,
    ) -> Result<MenuViewValueObject, MenuActionError> {
        match command {
            MenuCommandEnum::Show => {}
            MenuCommandEnum::ToggleServer => self.toggle_server().await,
            MenuCommandEnum::ToggleLoginItem => self.toggle().await?,
        }
        let server = self.server.current().await;
        Ok(MenuViewValueObject::new(
            &self.version,
            &server,
            self.login_item.status().await,
        ))
    }
}

#[cfg(test)]
#[path = "operate_menu_use_case_test.rs"]
mod operate_menu_use_case_test;
