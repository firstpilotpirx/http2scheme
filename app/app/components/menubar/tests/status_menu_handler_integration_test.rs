//! `StatusMenuHandler` on the main thread over a fake menu use case (harness off: `AppKit` needs the main thread).
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use menubar::adapters::primary::statusmenu::appkit::app_kit_config::AppKitConfig;
use menubar::adapters::primary::statusmenu::appkit::status_menu_handler::StatusMenuHandler;
use menubar::core::application::exceptions::menu_action_error::MenuActionError;
use menubar::core::application::use_cases::operate_menu::OperateMenu;
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;
use menubar::core::domain::menu_command_enum::MenuCommandEnum;
use menubar::core::domain::menu_view_value_object::MenuViewValueObject;
use menubar::core::domain::server_state_enum::ServerStateEnum;

#[derive(Default)]
struct FakeMenu {
    commands: Mutex<Vec<MenuCommandEnum>>,
}

#[async_trait]
impl OperateMenu for FakeMenu {
    async fn execute(
        &self,
        command: MenuCommandEnum,
    ) -> Result<MenuViewValueObject, MenuActionError> {
        let mut commands = self.commands.lock().unwrap();
        commands.push(command);
        let toggled = commands
            .iter()
            .filter(|c| **c == MenuCommandEnum::ToggleLoginItem)
            .count();
        let login = if toggled % 2 == 1 {
            LoginItemStateEnum::On
        } else {
            LoginItemStateEnum::Off
        };
        let server = ServerStateEnum::Running("http://127.0.0.1:7777".to_owned());
        Ok(MenuViewValueObject::new("0.1.0", &server, login))
    }
}

fn row(title: &str, enabled: bool, checked: bool) -> (String, bool, bool) {
    (title.to_owned(), enabled, checked)
}

fn the_menu_reads_in_order() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let handler = StatusMenuHandler::install(mtm, Arc::new(FakeMenu::default()));
    assert_eq!(
        handler.items(),
        [
            row("http2scheme 0.1.0", false, false),
            row("Running on http://127.0.0.1:7777", false, false),
            row("-", false, false),
            row("Start at Login", true, false),
            row("Open Settings Folder", true, false),
            row("Open Logs Folder", true, false),
            row("-", false, false),
            row("Quit http2scheme", true, false),
        ]
    );
}

fn choosing_an_item_runs_its_command() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let fake = Arc::new(FakeMenu::default());
    let handler = StatusMenuHandler::install(mtm, fake.clone());
    handler.choose("Start at Login");
    handler.choose("Open Logs Folder");
    let commands = fake.commands.lock().unwrap().clone();
    assert!(
        commands.contains(&MenuCommandEnum::ToggleLoginItem),
        "{commands:?}"
    );
    assert!(
        commands.contains(&MenuCommandEnum::OpenFolder(
            menubar::core::domain::app_folder_enum::AppFolderEnum::Logs
        )),
        "{commands:?}"
    );
    assert!(
        handler.items()[3].2,
        "Start at Login is checked after one toggle"
    );
}

fn main() {
    the_menu_reads_in_order();
    choosing_an_item_runs_its_command();
    println!("test result: ok. 2 passed; 0 failed");
}
