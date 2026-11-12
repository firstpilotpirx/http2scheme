use super::MenuCommandEnum;
use crate::core::domain::app_folder_enum::AppFolderEnum;

#[test]
fn an_open_folder_command_names_its_folder() {
    assert_ne!(
        MenuCommandEnum::OpenFolder(AppFolderEnum::Settings),
        MenuCommandEnum::OpenFolder(AppFolderEnum::Logs)
    );
    assert_ne!(MenuCommandEnum::Show, MenuCommandEnum::ToggleLoginItem);
}
