//! Input port: one request of the menu.

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;
use async_trait::async_trait;

/// Runs one menu request and answers what the menu shows next.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OperateMenu: Send + Sync {
    /// Runs the use case.
    ///
    /// # Errors
    /// The action failed; the menu keeps its words.
    async fn execute(
        &self,
        command: MenuCommandEnum,
    ) -> Result<MenuViewValueObject, MenuActionError>;
}

#[cfg(test)]
#[path = "operate_menu_test.rs"]
mod operate_menu_test;
