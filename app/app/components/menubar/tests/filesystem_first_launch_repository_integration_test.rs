//! `FilesystemFirstLaunchRepository` over a temporary settings folder.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use menubar::adapters::secondary::firstlaunch::filesystem::filesystem_first_launch_repository::FilesystemFirstLaunchRepository;
use menubar::core::application::repositories::first_launch_repository::FirstLaunchRepository;
use tempfile::TempDir;

#[tokio::test]
async fn a_record_creates_the_folder_and_then_exists() {
    let home = TempDir::new().unwrap();
    let settings = home.path().join("settings");
    let repository = FilesystemFirstLaunchRepository::new(&settings);
    assert!(!repository.exists().await.unwrap());
    repository.record().await.unwrap();
    assert!(repository.exists().await.unwrap());
    assert!(settings.join("login-item-offered").is_file());
}

#[tokio::test]
async fn a_record_under_a_file_is_an_error() {
    let home = TempDir::new().unwrap();
    let blocker = home.path().join("file");
    std::fs::write(&blocker, "").unwrap();
    let repository = FilesystemFirstLaunchRepository::new(&blocker.join("settings"));
    assert!(repository.record().await.is_err());
}
