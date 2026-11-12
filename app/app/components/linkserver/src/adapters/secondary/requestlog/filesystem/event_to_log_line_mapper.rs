//! One request log line per open attempt.

use std::fmt::Write as _;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;

const SECONDS_PER_DAY: u64 = 86_400;

/// `<RFC 3339 UTC time>\t<result>\t<link>`, with control characters in the link escaped.
#[must_use]
pub fn to_line(event: &LinkOpenAttemptedDomainEvent) -> String {
    format!(
        "{}\t{}\t{}",
        rfc3339(event.at()),
        event.outcome().label(),
        escape(event.link_text())
    )
}

/// The time in RFC 3339 form, UTC, whole seconds.
#[must_use]
pub fn rfc3339(at: SystemTime) -> String {
    let seconds = at
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let (year, month, day) = civil_from_days(seconds / SECONDS_PER_DAY);
    let rest = seconds % SECONDS_PER_DAY;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}

/// Year, month and day of a count of days since 1970-01-01 (H. Hinnant's algorithm).
fn civil_from_days(days: u64) -> (u64, u64, u64) {
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
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    (year, month, day)
}

/// The text with backslashes and control characters escaped, so one event stays one line.
#[must_use]
pub fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other if other.is_control() => {
                let _ = write!(escaped, "\\u{{{:x}}}", u32::from(other));
            }
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
#[path = "event_to_log_line_mapper_test.rs"]
mod event_to_log_line_mapper_test;
