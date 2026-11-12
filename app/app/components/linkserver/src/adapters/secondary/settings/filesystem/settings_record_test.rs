use config::{Config, File, FileFormat};

use super::SettingsRecord;

fn read(text: &str) -> SettingsRecord {
    Config::builder()
        .add_source(File::from_str(text, FileFormat::Toml))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}

#[test]
fn close_tab_is_on_when_missing() {
    let record = read("port = 7777\nallowed_schemes = [\"idea\"]\n");
    assert_eq!(
        record,
        SettingsRecord {
            port: 7777,
            allowed_schemes: vec!["idea".to_owned()],
            close_tab: true
        }
    );
}

#[test]
fn close_tab_can_be_off() {
    assert!(!read("port = 7777\nallowed_schemes = []\nclose_tab = false\n").close_tab);
}
