//! What the Link History window shows at one moment.

use super::link_record_value_object::LinkRecordValueObject;

/// The line the window shows when the request log holds no request.
pub const NOTHING_OPENED_NOTE: &str = "No link has been opened yet.";

/// The rows, newest first, or the line said in place of an empty table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryViewValueObject {
    rows: Vec<LinkRecordValueObject>,
}

impl HistoryViewValueObject {
    /// The view of `rows`, newest first.
    #[must_use]
    pub const fn new(rows: Vec<LinkRecordValueObject>) -> Self {
        Self { rows }
    }

    /// The rows, newest first.
    #[must_use]
    pub fn rows(&self) -> &[LinkRecordValueObject] {
        &self.rows
    }

    /// The line shown in place of the table, when there is no row.
    #[must_use]
    pub fn note(&self) -> Option<&'static str> {
        self.rows.is_empty().then_some(NOTHING_OPENED_NOTE)
    }
}

#[cfg(test)]
#[path = "history_view_value_object_test.rs"]
mod history_view_value_object_test;
