use std::path::PathBuf;

use super::LaunchServicesFolderOpenerGateway;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::folder_opener_service::FolderOpenerService;

#[tokio::test]
async fn a_missing_program_is_an_error_naming_the_folder() {
    let home = tempfile::tempdir().unwrap();
    let folder = home.path().join("Logs");
    let gateway = LaunchServicesFolderOpenerGateway::new(PathBuf::from("/nonexistent/open"));
    let error = gateway.open(&folder).await.unwrap_err();
    assert!(
        matches!(error, MenuActionError::Folder { ref path, .. } if path == &folder.display().to_string())
    );
    assert!(folder.is_dir());
}
