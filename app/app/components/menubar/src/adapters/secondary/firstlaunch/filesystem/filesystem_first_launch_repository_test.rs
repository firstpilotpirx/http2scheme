use super::FilesystemFirstLaunchRepository;
use crate::core::application::repositories::first_launch_repository::FirstLaunchRepository;

#[tokio::test]
async fn the_record_lives_in_the_settings_folder() {
    let home = tempfile::tempdir().unwrap();
    let repository = FilesystemFirstLaunchRepository::new(home.path());
    assert_eq!(repository.exists().await, Ok(false));
    repository.record().await.unwrap();
    assert!(home.path().join("login-item-offered").exists());
}

#[tokio::test]
async fn a_file_in_place_of_the_settings_folder_is_an_error() {
    let home = tempfile::tempdir().unwrap();
    let blocker = home.path().join("blocker");
    std::fs::write(&blocker, b"").unwrap();
    let repository = FilesystemFirstLaunchRepository::new(&blocker);
    assert!(repository.exists().await.is_err());
    assert!(repository.record().await.is_err());
}
