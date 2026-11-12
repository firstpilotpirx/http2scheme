//! The app's login item through `SMAppService` (macOS 13 and later), as System Settings →
//! General → Login Items lists it.

#![expect(unsafe_code, reason = "ADR-001: ServiceManagement FFI")]

use std::path::Path;

use async_trait::async_trait;
use objc2_service_management::{SMAppService, SMAppServiceStatus};

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::login_item_service::LoginItemService;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;

const NOT_A_BUNDLE: &str = "http2scheme does not run as an installed app";

#[derive(Clone, Copy)]
enum Operation {
    Status,
    Register,
    Unregister,
    OpenSettings,
}

enum Answer {
    Status(SMAppServiceStatus),
    Done(Result<(), String>),
}

/// The login item of the bundle this process runs from; outside a bundle the system is never asked.
pub struct SmAppServiceLoginItemGateway {
    installed: bool,
}

impl SmAppServiceLoginItemGateway {
    /// The login item of the app whose program is `program`.
    #[must_use]
    pub fn new(program: &Path) -> Self {
        Self {
            installed: Self::is_bundle_program(program),
        }
    }

    /// Whether `program` is `<anything>.app/Contents/MacOS/<name>`.
    #[must_use]
    pub fn is_bundle_program(program: &Path) -> bool {
        let macos = program.parent();
        let contents = macos.and_then(Path::parent);
        let bundle = contents.and_then(Path::parent);
        macos
            .and_then(Path::file_name)
            .is_some_and(|name| name == "MacOS")
            && contents
                .and_then(Path::file_name)
                .is_some_and(|name| name == "Contents")
            && bundle
                .and_then(Path::extension)
                .is_some_and(|ext| ext == "app")
    }

    /// The login item state for the system's status: a never registered item can be registered.
    #[must_use]
    pub fn state_of(status: SMAppServiceStatus) -> LoginItemStateEnum {
        match status {
            SMAppServiceStatus::Enabled => LoginItemStateEnum::On,
            SMAppServiceStatus::RequiresApproval => LoginItemStateEnum::NeedsApproval,
            SMAppServiceStatus::NotRegistered | SMAppServiceStatus::NotFound => {
                LoginItemStateEnum::Off
            }
            _ => LoginItemStateEnum::Unavailable,
        }
    }

    fn call(&self, operation: Operation) -> Option<Answer> {
        if !self.installed {
            return None;
        }
        let describe = |error: objc2::rc::Retained<objc2_foundation::NSError>| {
            error.localizedDescription().to_string()
        };
        // SAFETY: each call takes only the service, alive for the call; any thread may call it.
        let answer = unsafe {
            let service = SMAppService::mainAppService();
            match operation {
                Operation::Status => Answer::Status(service.status()),
                Operation::Register => {
                    Answer::Done(service.registerAndReturnError().map_err(describe))
                }
                Operation::Unregister => {
                    Answer::Done(service.unregisterAndReturnError().map_err(describe))
                }
                Operation::OpenSettings => {
                    SMAppService::openSystemSettingsLoginItems();
                    Answer::Done(Ok(()))
                }
            }
        };
        Some(answer)
    }

    fn act(&self, operation: Operation) -> Result<(), MenuActionError> {
        match self.call(operation) {
            Some(Answer::Done(result)) => result.map_err(MenuActionError::LoginItem),
            Some(Answer::Status(_)) | None => {
                Err(MenuActionError::LoginItem(NOT_A_BUNDLE.to_owned()))
            }
        }
    }
}

#[async_trait]
impl LoginItemService for SmAppServiceLoginItemGateway {
    async fn status(&self) -> LoginItemStateEnum {
        match self.call(Operation::Status) {
            Some(Answer::Status(status)) => Self::state_of(status),
            _ => LoginItemStateEnum::Unavailable,
        }
    }

    async fn register(&self) -> Result<(), MenuActionError> {
        self.act(Operation::Register)
    }

    async fn unregister(&self) -> Result<(), MenuActionError> {
        self.act(Operation::Unregister)
    }

    async fn open_system_settings(&self) -> Result<(), MenuActionError> {
        self.act(Operation::OpenSettings)
    }
}

#[cfg(test)]
#[path = "sm_app_service_login_item_gateway_test.rs"]
mod sm_app_service_login_item_gateway_test;
