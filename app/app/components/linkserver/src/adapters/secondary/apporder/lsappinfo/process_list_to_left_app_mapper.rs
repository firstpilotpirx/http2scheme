//! Reads the output of `lsappinfo`: the app behind the browser and an app's bundle id.

use crate::adapters::secondary::browsertabs::applescript::port_to_apple_script_mapper::BROWSERS;

/// The ASN of the app right behind the front app, when the front app is a supported browser.
///
/// `list` is the output of `lsappinfo visibleProcessList`, front to back:
/// `ASN:0x0-0x9ba9ba-"Claude": ASN:0x0-0xb40b4-"Safari": …`, a space in a name printed as `_`.
#[must_use]
pub fn left_app(list: &str) -> Option<String> {
    let mut entries = list.split("\": ").filter_map(entry);
    let (_, front) = entries.next()?;
    if !BROWSERS.contains(&front.replace('_', " ").as_str()) {
        return None;
    }
    entries.next().map(|(asn, _)| asn)
}

/// The bundle id in the output of `lsappinfo info -only bundleid <ASN>`: `bundleID="<id>"`.
#[must_use]
pub fn bundle_id(info: &str) -> Option<String> {
    let (_, rest) = info.split_once("bundleID=\"")?;
    let (id, _) = rest.split_once('"')?;
    (!id.is_empty()).then(|| id.to_owned())
}

/// One `ASN:<id>-"<name>` entry as `(ASN:<id>, name)`.
fn entry(text: &str) -> Option<(String, String)> {
    let text = text.trim();
    let (asn, name) = text.split_once("-\"")?;
    asn.starts_with("ASN:")
        .then(|| (asn.to_owned(), name.trim_end_matches(['"', ':']).to_owned()))
}

#[cfg(test)]
#[path = "process_list_to_left_app_mapper_test.rs"]
mod process_list_to_left_app_mapper_test;
