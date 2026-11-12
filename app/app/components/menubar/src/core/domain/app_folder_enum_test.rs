use super::AppFolderEnum;

#[test]
fn each_folder_names_its_menu_item() {
    assert_eq!(AppFolderEnum::Settings.title(), "Open Settings Folder");
    assert_eq!(AppFolderEnum::Logs.title(), "Open Logs Folder");
}
