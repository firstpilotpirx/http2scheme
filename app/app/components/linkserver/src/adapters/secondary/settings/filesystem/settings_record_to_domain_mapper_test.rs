use super::{to_domain, to_text};
use crate::adapters::secondary::settings::filesystem::settings_record::SettingsRecord;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

fn record(port: u16, schemes: &[&str]) -> SettingsRecord {
    SettingsRecord {
        port,
        allowed_schemes: schemes.iter().map(|s| (*s).to_owned()).collect(),
        close_tab: true,
    }
}

#[test]
fn a_good_record_maps_to_the_settings() {
    let settings = to_domain(&record(7777, &["idea", "goland"]), "/s.toml").unwrap();
    assert_eq!(settings, ServerSettingsValueObject::defaults());
}

#[test]
fn a_low_port_names_the_port_field() {
    let error = to_domain(&record(80, &["idea"]), "/s.toml").unwrap_err();
    assert_eq!(error.field(), "port");
    assert!(error.to_string().starts_with("/s.toml: port"), "{error}");
}

#[test]
fn a_bad_scheme_names_the_schemes_field() {
    let error = to_domain(&record(7777, &["1bad"]), "/s.toml").unwrap_err();
    assert_eq!(error.field(), "allowed_schemes");
    assert!(error.to_string().contains("\"1bad\""), "{error}");
}

#[test]
fn the_text_holds_every_field() {
    let text = to_text(&ServerSettingsValueObject::defaults());
    assert!(text.contains("port = 7777\n"), "{text}");
    assert!(
        text.contains("allowed_schemes = [\"idea\", \"goland\"]\n"),
        "{text}"
    );
    assert!(text.contains("close_tab = true\n"), "{text}");
}
