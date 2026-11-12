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

/// The shape of a request time: `0` is a digit, every other byte stands for itself.
const TIME_SHAPE: &[u8; 20] = b"0000-00-00T00:00:00Z";

/// `2025-11-13T02:13:20Z` as an instant.
fn parse_time(text: &str) -> Option<SystemTime> {
    let fields = time_fields(text)?;
    in_range(fields).then(|| {
        let [year, month, day, hour, minute, second] = fields;
        let days = days_from_civil(year, month, day);
        UNIX_EPOCH
            + Duration::from_secs(days * SECONDS_PER_DAY + hour * 3600 + minute * 60 + second)
    })
}

/// Year, month, day, hour, minute and second of a text shaped like [`TIME_SHAPE`].
fn time_fields(text: &str) -> Option<[u64; 6]> {
    if !has_time_shape(text) {
        return None;
    }
    let number = |from: usize, to: usize| text.get(from..to)?.parse::<u64>().ok();
    Some([
        number(0, 4)?,
        number(5, 7)?,
        number(8, 10)?,
        number(11, 13)?,
        number(14, 16)?,
        number(17, 19)?,
    ])
}

/// Whether `text` has the digits and separators of [`TIME_SHAPE`].
fn has_time_shape(text: &str) -> bool {
    text.len() == TIME_SHAPE.len()
        && text.bytes().zip(TIME_SHAPE).all(|(byte, &shape)| {
            if shape == b'0' {
                byte.is_ascii_digit()
            } else {
                byte == shape
            }
        })
}

/// Whether year, month, day, hour, minute and second name a time from 1970 on.
fn in_range([year, month, day, hour, minute, second]: [u64; 6]) -> bool {
    (1..=12).contains(&month)
        && (1..=31).contains(&day)
        && hour < 24
        && minute < 60
        && second < 60
        && year >= 1970
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
        let code = characters.next()?;
        link.push(escaped(code, &mut characters)?);
    }
    Some(link)
}

/// The character an escape `\<code>` stands for; `\u{…}` reads its hex digits from `rest`.
fn escaped(code: char, rest: &mut std::str::Chars<'_>) -> Option<char> {
    match code {
        '\\' => Some('\\'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        'u' => {
            let digits: String = rest.take_while(|&c| c != '}').collect();
            char::from_u32(u32::from_str_radix(digits.strip_prefix('{')?, 16).ok()?)
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "log_line_to_record_mapper_test.rs"]
mod log_line_to_record_mapper_test;
