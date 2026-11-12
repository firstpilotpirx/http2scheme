use std::time::UNIX_EPOCH;

use super::HistoryViewValueObject;
use crate::core::domain::link_record_value_object::LinkRecordValueObject;

#[test]
fn an_empty_history_says_nothing_was_opened() {
    let view = HistoryViewValueObject::new(Vec::new());
    assert_eq!(view.rows(), []);
    assert_eq!(view.note(), Some("No link has been opened yet."));
}

#[test]
fn a_history_with_rows_has_no_note() {
    let row = LinkRecordValueObject::new(UNIX_EPOCH, "opened".to_owned(), "idea://x".to_owned());
    let view = HistoryViewValueObject::new(vec![row.clone()]);
    assert_eq!(view.rows(), [row]);
    assert_eq!(view.note(), None);
}
