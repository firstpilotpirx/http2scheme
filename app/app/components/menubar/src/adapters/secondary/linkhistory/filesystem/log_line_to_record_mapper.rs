//! One request log line back to a record: `<RFC 3339 UTC time>\t<result>\t<escaped link>`.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::core::domain::link_record_value_object::LinkRecordValueObject;

const SECONDS_PER_DAY: u64 = 86_400;

/// The record of a well-formed line; none for a damaged one.
#[must_use]
pub fn to_record(line: &str) -> Option<LinkRecordValueObject> {
    let mut fields = line.splitn(3, '\t');
    let at = parse_time(fields.next()?)?;
    let outcome = fields.next().filter(|word| !word.is_empty())?;
    let scheme_link = unescape(fields.next()?)?;
    Some(LinkRecordValueObject::new(
        at,
        outcome.to_owned(),
        scheme_link,
    ))
}

/// `2025-11-13T02:13:20Z` as an instant.
fn parse_time(text: &str) -> Option<SystemTime> {
    let bytes = text.as_bytes();
    let shape_ok = bytes.len() == 20
        && [4, 7].iter().all(|&i| bytes[i] == b'-')
        && bytes[10] == b'T'
        && [13, 16].iter().all(|&i| bytes[i] == b':')
        && bytes[19] == b'Z';
    if !shape_ok {
        return None;
    }
    let number = |from: usize, to: usize| text.get(from..to)?.parse::<u64>().ok();
    let (year, month, day) = (number(0, 4)?, number(5, 7)?, number(8, 10)?);
    let (hour, minute, second) = (number(11, 13)?, number(14, 16)?, number(17, 19)?);
    let valid = (1..=12).contains(&month)
        && (1..=31).contains(&day)
        && hour < 24
        && minute < 60
        && second < 60
        && year >= 1970;
    if !valid {
        return None;
    }
    let seconds =
        days_from_civil(year, month, day) * SECONDS_PER_DAY + hour * 3600 + minute * 60 + second;
    Some(UNIX_EPOCH + Duration::from_secs(seconds))
}

/// Days since 1970-01-01 of a date (H. Hinnant's algorithm), for years from 1970.
const fn days_from_civil(year: u64, month: u64, day: u64) -> u64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year / 400;
    let year_of_era = year - era * 400;
    let month_index = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * month_index + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The link with the request log's escapes undone; none for a broken escape.
fn unescape(text: &str) -> Option<String> {
    let mut link = String::with_capacity(text.len());
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            link.push(character);
            continue;
        }
        match characters.next()? {
            '\\' => link.push('\\'),
            'n' => link.push('\n'),
            'r' => link.push('\r'),
            't' => link.push('\t'),
            'u' => {
                let rest: String = characters.by_ref().take_while(|&c| c != '}').collect();
                let hex = rest.strip_prefix('{')?;
                link.push(char::from_u32(u32::from_str_radix(hex, 16).ok()?)?);
            }
            _ => return None,
        }
    }
    Some(link)
}

#[cfg(test)]
#[path = "log_line_to_record_mapper_test.rs"]
mod log_line_to_record_mapper_test;
