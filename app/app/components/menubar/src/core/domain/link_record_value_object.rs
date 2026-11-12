//! One line of the request log, as the History tab lists it.

use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_PER_DAY: i64 = 86_400;

/// When a link was requested, what came of it and the link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkRecordValueObject {
    at: SystemTime,
    result: String,
    link: String,
}

impl LinkRecordValueObject {
    /// A record of the request at `at`.
    #[must_use]
    pub const fn new(at: SystemTime, result: String, link: String) -> Self {
        Self { at, result, link }
    }

    /// When the request came.
    #[must_use]
    pub const fn at(&self) -> SystemTime {
        self.at
    }

    /// The result word of the request log, such as `opened`.
    #[must_use]
    pub fn result(&self) -> &str {
        &self.result
    }

    /// The scheme link, or why it was unreadable.
    #[must_use]
    pub fn link(&self) -> &str {
        &self.link
    }

    /// `2026-11-12 19:42:05`: the time in the zone `utc_offset` seconds from UTC.
    #[must_use]
    pub fn local_time(&self, utc_offset: i64) -> String {
        let seconds = self
            .at
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| {
                i64::try_from(since.as_secs()).unwrap_or(i64::MAX)
            })
            .saturating_add(utc_offset)
            .max(0);
        let (year, month, day) = civil_from_days(seconds / SECONDS_PER_DAY);
        let rest = seconds % SECONDS_PER_DAY;
        format!(
            "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}",
            rest / 3600,
            rest % 3600 / 60,
            rest % 60
        )
    }
}

/// Year, month and day of a count of days since 1970-01-01 (H. Hinnant's algorithm).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted / 146_097;
    let day_of_era = shifted % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    (year_of_era + era * 400 + i64::from(month <= 2), month, day)
}

#[cfg(test)]
#[path = "link_record_value_object_test.rs"]
mod link_record_value_object_test;
