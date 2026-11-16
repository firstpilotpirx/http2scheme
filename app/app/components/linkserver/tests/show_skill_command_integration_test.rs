//! `ShowSkillCommand` through the component's root with the macOS adapters and a temp home.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::io::{self, Write};

use support::root_in;

/// An output that refuses every write.
struct Closed;

impl Write for Closed {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "output closed"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn prints_the_skill_naming_the_program() {
    let home = tempfile::tempdir().unwrap();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = root_in(home.path())
        .skill_command()
        .run(&mut out, &mut err)
        .await;
    assert_eq!(code, 0);
    let text = String::from_utf8(out).unwrap();
    let program = home.path().join("http2scheme");
    assert!(text.starts_with("---\nname: http2scheme\n"), "{text}");
    assert!(
        text.contains(&format!("'{}' link", program.display())),
        "{text}"
    );
    assert_eq!(err, Vec::<u8>::new());
}

#[tokio::test]
async fn a_closed_output_fails_with_the_reason() {
    let home = tempfile::tempdir().unwrap();
    let mut err = Vec::new();
    let code = root_in(home.path())
        .skill_command()
        .run(&mut Closed, &mut err)
        .await;
    assert_eq!(code, 1);
    assert_eq!(String::from_utf8(err).unwrap(), "output closed\n");
}
