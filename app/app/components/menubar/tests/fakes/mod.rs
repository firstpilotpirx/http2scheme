//! A fake login item: the system's login items without touching them.

use std::sync::Mutex;

use async_trait::async_trait;
use menubar::core::application::exceptions::menu_action_error::MenuActionError;
use menubar::core::application::services::login_item_service::LoginItemService;
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;

/// Holds one login item state and the actions taken on it.
#[derive(Debug)]
pub struct FakeLoginItem {
    state: Mutex<LoginItemStateEnum>,
    actions: Mutex<Vec<&'static str>>,
}

impl FakeLoginItem {
    /// A login item in `state`.
    pub const fn new(state: LoginItemStateEnum) -> Self {
        Self {
            state: Mutex::new(state),
            actions: Mutex::new(Vec::new()),
        }
    }

    /// The current state.
    pub fn state(&self) -> LoginItemStateEnum {
        *self.state.lock().unwrap()
    }

    /// The actions taken, in order.
    pub fn actions(&self) -> Vec<&'static str> {
        self.actions.lock().unwrap().clone()
    }

    fn act(&self, action: &'static str, next: Option<LoginItemStateEnum>) {
        self.actions.lock().unwrap().push(action);
        if let Some(next) = next {
            *self.state.lock().unwrap() = next;
        }
    }
}

#[async_trait]
impl LoginItemService for FakeLoginItem {
    async fn status(&self) -> LoginItemStateEnum {
        self.state()
    }

    async fn register(&self) -> Result<(), MenuActionError> {
        self.act("register", Some(LoginItemStateEnum::On));
        Ok(())
    }

    async fn unregister(&self) -> Result<(), MenuActionError> {
        self.act("unregister", Some(LoginItemStateEnum::Off));
        Ok(())
    }

    async fn open_system_settings(&self) -> Result<(), MenuActionError> {
        self.act("open_system_settings", None);
        Ok(())
    }
}
