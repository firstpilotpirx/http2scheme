use std::time::{Duration, UNIX_EPOCH};

use super::shown_rows;
use crate::core::domain::history_view_value_object::HistoryViewValueObject;
use crate::core::domain::link_record_value_object::LinkRecordValueObject;

#[test]
fn a_row_shows_the_result_and_the_link_as_logged() {
    let at = UNIX_EPOCH + Duration::from_secs(1_763_000_000);
    let record = LinkRecordValueObject::new(at, "wrong key".to_owned(), "idea://a".to_owned());
    let rows = shown_rows(&HistoryViewValueObject::new(vec![record]));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0][1], "wrong key");
    assert_eq!(rows[0][2], "idea://a");
    assert_eq!(rows[0][0].len(), "12.11.2025 02:13".len(), "{}", rows[0][0]);
}
