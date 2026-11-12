//! Shows the menu's state and runs what the user chooses in it.

use async_trait::async_trait;
use std::sync::Arc;

use super::operate_menu::OperateMenu;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::folder_opener_service::FolderOpenerService;
use crate::core::application::services::login_item_service::LoginItemService;
use crate::core::application::services::server_state_service::ServerStateService;
use crate::core::domain::app_folder_enum::AppFolderEnum;
use crate::core::domain::app_folders_value_object::AppFoldersValueObject;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;

/// The menu's use case over the server state, the login item and the Finder.
pub struct OperateMenuUseCase {
    version: String,
    folders: AppFoldersValueObject,
    server: Arc<dyn ServerStateService>,
    login_item: Arc<dyn LoginItemService>,
    opener: Arc<dyn FolderOpenerService>,
}

impl OperateMenuUseCase {
    /// A use case for the app `version` with its `folders`.
    #[must_use]
    pub fn new(
        version: String,
        folders: AppFoldersValueObject,
        server: Arc<dyn ServerStateService>,
        login_item: Arc<dyn LoginItemService>,
        opener: Arc<dyn FolderOpenerService>,
    ) -> Self {
        Self {
            version,
            folders,
            server,
            login_item,
            opener,
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

    async fn open(&self, folder: AppFolderEnum) -> Result<(), MenuActionError> {
        self.opener.open(self.folders.path(folder)).await
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
            MenuCommandEnum::ToggleLoginItem => self.toggle().await?,
            MenuCommandEnum::OpenFolder(folder) => self.open(folder).await?,
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
