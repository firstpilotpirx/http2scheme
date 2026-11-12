//! The `AppleScript` that closes the server's open-address tabs.

use std::fmt::Write as _;

use crate::core::domain::port_value_object::PortValueObject;

/// The browsers whose tabs `AppleScript` can close.
pub const BROWSERS: [&str; 5] = [
    "Safari",
    "Google Chrome",
    "Arc",
    "Brave Browser",
    "Microsoft Edge",
];

/// A script that closes, in every running browser, the tabs whose address starts with
/// `http://<host>:<port>/open?` for any of `hosts`.
///
/// Each browser runs in `run script` inside `try`, so a browser that is missing or refuses
/// stops neither the script nor the others.
#[must_use]
pub fn to_script(hosts: &[String], port: PortValueObject) -> String {
    let filter = hosts
        .iter()
        .map(|host| {
            format!(
                "URL starts with \\\"http://{host}:{}/open?\\\"",
                port.value()
            )
        })
        .collect::<Vec<_>>()
        .join(" or ");
    BROWSERS.iter().fold(String::new(), |mut script, browser| {
        let _ = writeln!(
            script,
            "try\nif application \"{browser}\" is running then run script \"tell application \\\"{browser}\\\" to close (every tab of every window whose {filter})\"\nend try"
        );
        script
    })
}

#[cfg(test)]
#[path = "port_to_apple_script_mapper_test.rs"]
mod port_to_apple_script_mapper_test;
