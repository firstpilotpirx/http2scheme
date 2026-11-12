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
