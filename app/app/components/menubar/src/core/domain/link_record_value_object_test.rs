use std::time::{Duration, UNIX_EPOCH};

use super::LinkRecordValueObject;

fn record(seconds: u64) -> LinkRecordValueObject {
    LinkRecordValueObject::new(
        UNIX_EPOCH + Duration::from_secs(seconds),
        "opened".to_owned(),
        "idea://x".to_owned(),
    )
}

#[test]
fn holds_its_parts() {
    let record = record(0);
    assert_eq!(record.at(), UNIX_EPOCH);
    assert_eq!(record.result(), "opened");
    assert_eq!(record.link(), "idea://x");
}

#[test]
fn shows_the_time_east_of_utc() {
    assert_eq!(
        record(1_763_000_000).local_time(3 * 3600),
        "13.11.2025 05:13"
    );
}

#[test]
fn shows_the_time_west_of_utc_on_the_day_before() {
    assert_eq!(
        record(1_763_000_000).local_time(-5 * 3600),
        "12.11.2025 21:13"
    );
}

#[test]
fn shows_a_leap_day() {
    assert_eq!(record(951_782_401).local_time(0), "29.02.2000 00:00");
}
