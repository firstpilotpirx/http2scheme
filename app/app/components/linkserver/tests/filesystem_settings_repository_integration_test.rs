//! `FilesystemSettingsRepository` against a real temporary folder.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::fs;
use std::os::unix::fs::PermissionsExt;

use linkserver::adapters::secondary::settings::filesystem::filesystem_settings_repository::FilesystemSettingsRepository;
use linkserver::core::application::exceptions::settings_error::SettingsError;
use linkserver::core::application::repositories::settings_repository::SettingsRepository;
use linkserver::core::domain::access_key_value_object::AccessKeyValueObject;
use linkserver::core::domain::port_value_object::PortValueObject;
use linkserver::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use linkserver::core::domain::server_settings_value_object::ServerSettingsValueObject;
use tempfile::TempDir;

fn repository(home: &TempDir) -> FilesystemSettingsRepository {
    FilesystemSettingsRepository::new(home.path().join("http2scheme"))
}

fn invalid_field(error: SettingsError) -> String {
    match error {
        SettingsError::Invalid(invalid) => {
            assert!(
                invalid.to_string().contains("settings.toml")
                    || invalid.to_string().contains("key"),
                "{invalid}"
            );
            invalid.field().to_owned()
        }
        SettingsError::Io { .. } => panic!("expected an invalid field, got {error}"),
    }
}

#[tokio::test]
async fn a_missing_folder_reads_as_nothing() {
    let home = TempDir::new().unwrap();
    assert_eq!(repository(&home).read_settings().await.unwrap(), None);
    assert_eq!(repository(&home).read_key().await.unwrap(), None);
}

#[tokio::test]
async fn created_settings_read_back_the_same() {
    let home = TempDir::new().unwrap();
    let repository = repository(&home);
    repository
        .create_settings(&ServerSettingsValueObject::defaults())
        .await
        .unwrap();
    assert_eq!(
        repository.read_settings().await.unwrap(),
        Some(ServerSettingsValueObject::defaults())
    );
    let text = fs::read_to_string(home.path().join("http2scheme/settings.toml")).unwrap();
    assert!(text.contains("port = 7777"), "{text}");
    assert!(text.contains("close_tab = true"), "{text}");
}

#[tokio::test]
async fn create_keeps_an_existing_settings_file() {
    let home = TempDir::new().unwrap();
    let repository = repository(&home);
    fs::create_dir_all(home.path().join("http2scheme")).unwrap();
    fs::write(
        home.path().join("http2scheme/settings.toml"),
        "port = 9000\nallowed_schemes = [\"slack\"]\n",
    )
    .unwrap();
    repository
        .create_settings(&ServerSettingsValueObject::defaults())
        .await
        .unwrap();
    let settings = repository.read_settings().await.unwrap().unwrap();
    assert_eq!(settings.port(), PortValueObject::new(9000).unwrap());
    assert_eq!(settings.allowlist().schemes()[0].as_str(), "slack");
    assert!(settings.close_tab());
}

#[tokio::test]
async fn created_key_is_owner_only_and_reads_back() {
    let home = TempDir::new().unwrap();
    let repository = repository(&home);
    let key = AccessKeyValueObject::from_bytes([4; 32]);
    repository.create_key(&key).await.unwrap();
    let mode = fs::metadata(home.path().join("http2scheme/key"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600);
    assert_eq!(repository.read_key().await.unwrap(), Some(key));
}

#[tokio::test]
async fn create_key_never_overwrites() {
    let home = TempDir::new().unwrap();
    let repository = repository(&home);
    repository
        .create_key(&AccessKeyValueObject::from_bytes([1; 32]))
        .await
        .unwrap();
    repository
        .create_key(&AccessKeyValueObject::from_bytes([2; 32]))
        .await
        .unwrap();
    assert_eq!(
        repository.read_key().await.unwrap(),
        Some(AccessKeyValueObject::from_bytes([1; 32]))
    );
}

#[tokio::test]
async fn bad_settings_name_the_file_and_field() {
    let cases = [
        ("port = 80\nallowed_schemes = [\"idea\"]\n", "port"),
        (
            "port = 7777\nallowed_schemes = [\"1bad\"]\n",
            "allowed_schemes",
        ),
        (
            "port = 7777\nallowed_schemes = [\"idea\"]\ncolour = 1\n",
            "colour",
        ),
        ("port = \"x\"\nallowed_schemes = [\"idea\"]\n", "port"),
    ];
    for (text, field) in cases {
        let home = TempDir::new().unwrap();
        fs::create_dir_all(home.path().join("http2scheme")).unwrap();
        fs::write(home.path().join("http2scheme/settings.toml"), text).unwrap();
        let error = repository(&home).read_settings().await.unwrap_err();
        assert_eq!(invalid_field(error), field, "{text}");
    }
}

#[tokio::test]
async fn a_key_readable_by_others_is_refused() {
    let home = TempDir::new().unwrap();
    let repository = repository(&home);
    repository
        .create_key(&AccessKeyValueObject::from_bytes([1; 32]))
        .await
        .unwrap();
    let path = home.path().join("http2scheme/key");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    let error = repository.read_key().await.unwrap_err();
    assert!(
        error.to_string().contains("readable only by its owner"),
        "{error}"
    );
}

#[tokio::test]
async fn a_key_that_is_not_hex_is_refused() {
    let home = TempDir::new().unwrap();
    let path = home.path().join("http2scheme/key");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "not a key").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    let error = repository(&home).read_key().await.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("64 lower-case hexadecimal characters"),
        "{error}"
    );
}

#[tokio::test]
async fn settings_with_close_tab_off_read_back() {
    let home = TempDir::new().unwrap();
    let repository = repository(&home);
    let settings = ServerSettingsValueObject::new(
        PortValueObject::new(8000).unwrap(),
        SchemeAllowlistPolicy::defaults(),
        false,
    );
    repository.create_settings(&settings).await.unwrap();
    assert_eq!(repository.read_settings().await.unwrap(), Some(settings));
}

#[tokio::test]
async fn a_settings_file_that_cannot_be_read_names_the_file() {
    let home = TempDir::new().unwrap();
    fs::create_dir_all(home.path().join("http2scheme/settings.toml")).unwrap();
    let error = repository(&home).read_settings().await.unwrap_err();
    assert!(error.to_string().contains("settings.toml"), "{error}");
}

#[tokio::test]
async fn a_file_in_place_of_the_folder_is_an_io_error() {
    let home = TempDir::new().unwrap();
    fs::write(home.path().join("http2scheme"), b"").unwrap();
    let error = repository(&home)
        .create_settings(&ServerSettingsValueObject::defaults())
        .await
        .unwrap_err();
    assert!(matches!(error, SettingsError::Io { .. }), "{error}");
}

#[tokio::test]
async fn a_folder_without_write_permission_is_an_io_error() {
    let home = TempDir::new().unwrap();
    let folder = home.path().join("http2scheme");
    fs::create_dir(&folder).unwrap();
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o500)).unwrap();
    let result = repository(&home)
        .create_settings(&ServerSettingsValueObject::defaults())
        .await;
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).unwrap();
    let error = result.unwrap_err();
    assert!(matches!(error, SettingsError::Io { .. }), "{error}");
}
