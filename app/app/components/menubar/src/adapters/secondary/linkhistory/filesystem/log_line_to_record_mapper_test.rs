use std::time::{Duration, UNIX_EPOCH};

use super::to_record;

#[test]
fn reads_a_well_formed_line() {
    let record = to_record("2025-11-13T02:13:20Z\topened\tidea://open?line=3").unwrap();
    assert_eq!(record.at(), UNIX_EPOCH + Duration::from_secs(1_763_000_000));
    assert_eq!(record.result(), "opened");
    assert_eq!(record.link(), "idea://open?line=3");
}

#[test]
fn reads_known_times() {
    let at = |line: &str| to_record(line).unwrap().at();
    assert_eq!(at("1970-01-01T00:00:00Z\topened\tx"), UNIX_EPOCH);
    assert_eq!(
        at("2000-02-29T00:00:01Z\topened\tx"),
        UNIX_EPOCH + Duration::from_secs(951_782_401)
    );
    assert_eq!(
        at("2099-12-31T23:59:59Z\topened\tx"),
        UNIX_EPOCH + Duration::from_secs(4_102_444_799)
    );
}

#[test]
fn undoes_the_escapes() {
    let record = to_record("1970-01-01T00:00:00Z\topened\ta\\nb\\r\\tc\\\\d\\u{7}").unwrap();
    assert_eq!(record.link(), "a\nb\r\tc\\d\u{7}");
}

#[test]
fn keeps_a_tab_free_reason_as_the_link() {
    let record = to_record("1970-01-01T00:00:00Z\tmalformed link\tthe link is missing").unwrap();
    assert_eq!(record.link(), "the link is missing");
}

#[test]
fn a_damaged_line_has_no_record() {
    for line in [
        "",
        "garbage",
        "2025-11-13T02:13:20Z\topened",
        "2025-13-13T02:13:20Z\topened\tx",
        "2025-11-13 02:13:20\topened\tx",
        "2025-11-13T02:13:20Z\t\tx",
        "2025-11-13T02:13:20Z\topened\tbad\\q",
        "2025-11-13T02:13:20Z\topened\tbad\\u{zz}",
    ] {
        assert_eq!(to_record(line), None, "{line:?}");
    }
}
