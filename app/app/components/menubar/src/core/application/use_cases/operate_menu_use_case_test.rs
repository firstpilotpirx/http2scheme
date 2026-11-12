use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::OperateMenuUseCase;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::folder_opener_service::MockFolderOpenerService;
use crate::core::application::services::login_item_service::MockLoginItemService;
use crate::core::application::services::server_state_service::MockServerStateService;
use crate::core::application::use_cases::operate_menu::OperateMenu;
use crate::core::domain::app_folder_enum::AppFolderEnum;
use crate::core::domain::app_folders_value_object::AppFoldersValueObject;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::menu_command_enum::MenuCommandEnum;

fn use_case(
    login_item: MockLoginItemService,
    opener: MockFolderOpenerService,
) -> OperateMenuUseCase {
    let mut server = MockServerStateService::new();
    server.expect_current().return_const(
        crate::core::domain::server_state_enum::ServerStateEnum::Running(
            "http://127.0.0.1:7777".to_owned(),
        ),
    );
    OperateMenuUseCase::new(
        "0.1.0".to_owned(),
        AppFoldersValueObject::new(PathBuf::from("/s"), PathBuf::from("/l")),
        Arc::new(server),
        Arc::new(login_item),
        Arc::new(opener),
    )
}

fn login_item(state: LoginItemStateEnum) -> MockLoginItemService {
    let mut mock = MockLoginItemService::new();
    mock.expect_status().return_const(state);
    mock
}

#[tokio::test]
async fn show_answers_the_current_view_and_changes_nothing() {
    let view = use_case(
        login_item(LoginItemStateEnum::On),
        MockFolderOpenerService::new(),
    )
    .execute(MenuCommandEnum::Show)
    .await
    .unwrap();
    assert_eq!(view.title(), "http2scheme 0.1.0");
    assert_eq!(view.status_line(), "Running on http://127.0.0.1:7777");
    assert!(view.login_checked());
}

#[tokio::test]
async fn toggle_registers_a_login_item_that_is_off() {
    let mut mock = login_item(LoginItemStateEnum::Off);
    mock.expect_register().times(1).returning(|| Ok(()));
    let result = use_case(mock, MockFolderOpenerService::new())
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_unregisters_a_login_item_that_is_on() {
    let mut mock = login_item(LoginItemStateEnum::On);
    mock.expect_unregister().times(1).returning(|| Ok(()));
    let result = use_case(mock, MockFolderOpenerService::new())
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_opens_system_settings_for_a_login_item_waiting_for_approval() {
    let mut mock = login_item(LoginItemStateEnum::NeedsApproval);
    mock.expect_open_system_settings()
        .times(1)
        .returning(|| Ok(()));
    let result = use_case(mock, MockFolderOpenerService::new())
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_does_nothing_outside_a_bundle() {
    let mock = login_item(LoginItemStateEnum::Unavailable);
    let view = use_case(mock, MockFolderOpenerService::new())
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await
        .unwrap();
    assert!(!view.login_enabled());
}

#[tokio::test]
async fn a_refused_toggle_is_an_error() {
    let mut mock = login_item(LoginItemStateEnum::Off);
    mock.expect_register()
        .returning(|| Err(MenuActionError::LoginItem("denied".to_owned())));
    let result = use_case(mock, MockFolderOpenerService::new())
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert_eq!(result, Err(MenuActionError::LoginItem("denied".to_owned())));
}

#[tokio::test]
async fn open_folder_opens_the_folder_it_names() {
    for (folder, path) in [(AppFolderEnum::Settings, "/s"), (AppFolderEnum::Logs, "/l")] {
        let mut opener = MockFolderOpenerService::new();
        opener
            .expect_open()
            .withf(move |opened| opened == Path::new(path))
            .times(1)
            .returning(|_| Ok(()));
        let result = use_case(login_item(LoginItemStateEnum::Off), opener)
            .execute(MenuCommandEnum::OpenFolder(folder))
            .await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn a_failed_open_is_an_error() {
    let mut opener = MockFolderOpenerService::new();
    opener.expect_open().returning(|_| {
        Err(MenuActionError::Folder {
            path: "/l".to_owned(),
            reason: "exit status 1".to_owned(),
        })
    });
    let result = use_case(login_item(LoginItemStateEnum::Off), opener)
        .execute(MenuCommandEnum::OpenFolder(AppFolderEnum::Logs))
        .await;
    assert!(matches!(result, Err(MenuActionError::Folder { .. })));
}
