use std::path::{Path, PathBuf};

use super::AppFoldersValueObject;
use crate::core::domain::app_folder_enum::AppFolderEnum;

#[test]
fn each_folder_has_its_path() {
    let folders = AppFoldersValueObject::new(PathBuf::from("/s"), PathBuf::from("/l"));
    assert_eq!(folders.path(AppFolderEnum::Settings), Path::new("/s"));
    assert_eq!(folders.path(AppFolderEnum::Logs), Path::new("/l"));
}
