use super::{BROWSERS, to_script};
use crate::core::domain::port_value_object::PortValueObject;

#[test]
fn one_guarded_block_per_browser_with_every_host() {
    let script = to_script(
        &["127.0.0.1".to_owned(), "localhost".to_owned()],
        PortValueObject::new(7801).unwrap(),
    );
    assert_eq!(
        script.lines().filter(|line| *line == "try").count(),
        BROWSERS.len()
    );
    assert_eq!(script.matches("end try\n").count(), BROWSERS.len());
    let safari = script
        .lines()
        .find(|line| line.contains("\"Safari\" is running"))
        .unwrap();
    assert_eq!(
        safari,
        "if application \"Safari\" is running then run script \"tell application \\\"Safari\\\" to close (every tab of every window whose URL starts with \\\"http://127.0.0.1:7801/open?\\\" or URL starts with \\\"http://localhost:7801/open?\\\")\""
    );
}
