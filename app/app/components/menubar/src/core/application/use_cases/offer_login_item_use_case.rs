//! Adds the app to the login items at its first launch from Applications (design D16).

use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

use super::offer_login_item::OfferLoginItem;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::first_launch_repository::FirstLaunchRepository;
use crate::core::application::services::login_item_service::LoginItemService;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;

/// The folder an installed app runs from.
const APPLICATIONS: &str = "/Applications/";

/// Registers once, when the program runs from Applications and no record exists.
pub struct OfferLoginItemUseCase {
    program: PathBuf,
    login_item: Arc<dyn LoginItemService>,
    record: Arc<dyn FirstLaunchRepository>,
}

impl OfferLoginItemUseCase {
    /// A use case for the running `program`.
    #[must_use]
    pub fn new(
        program: PathBuf,
        login_item: Arc<dyn LoginItemService>,
        record: Arc<dyn FirstLaunchRepository>,
    ) -> Self {
        Self {
            program,
            login_item,
            record,
        }
    }
}

#[async_trait]
impl OfferLoginItem for OfferLoginItemUseCase {
    async fn execute(&self) -> Result<(), MenuActionError> {
        if !self.program.starts_with(APPLICATIONS) || self.record.exists().await? {
            return Ok(());
        }
        if self.login_item.status().await == LoginItemStateEnum::Off {
            self.login_item.register().await?;
        }
        self.record.record().await
    }
}

#[cfg(test)]
#[path = "offer_login_item_use_case_test.rs"]
mod offer_login_item_use_case_test;
