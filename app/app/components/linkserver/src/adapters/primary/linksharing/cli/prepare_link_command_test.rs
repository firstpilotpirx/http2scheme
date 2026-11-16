use std::sync::Arc;

use super::{PrepareLinkCommand, open_url};
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::prepare_link::MockPrepareLink;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::prepared_link_value_object::PreparedLinkValueObject;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

#[test]
fn encodes_every_reserved_character() {
    let prepared = PreparedLinkValueObject::new(
        PortValueObject::new(7777).unwrap(),
        AccessKeyValueObject::from_bytes([1; 32]),
        SchemeLinkValueObject::parse("idea://open?file=/a%20b/Файл.java&line=4~2").unwrap(),
    );
    let url = open_url("127.0.0.1", &prepared);
    let key = "01".repeat(32);
    assert_eq!(
        url,
        format!(
            "http://127.0.0.1:7777/open?key={key}&url=idea%3A%2F%2Fopen%3Ffile%3D%2Fa%2520b%2F%D0%A4%D0%B0%D0%B9%D0%BB.java%26line%3D4~2"
        )
    );
}

fn prepare(link: &str) -> PreparedLinkValueObject {
    PreparedLinkValueObject::new(
        PortValueObject::new(7777).unwrap(),
        AccessKeyValueObject::from_bytes([1; 32]),
        SchemeLinkValueObject::parse(link).unwrap(),
    )
}

async fn run(links: &[&str]) -> (u8, String, String) {
    let mut use_case = MockPrepareLink::new();
    use_case.expect_execute().returning(|link| {
        if link.starts_with("slack:") {
            Err(LinkRejectedError::SchemeNotAllowed {
                scheme: "slack".to_owned(),
            }
            .into())
        } else {
            Ok(prepare(link))
        }
    });
    let command = PrepareLinkCommand::new(
        Arc::new(use_case),
        "127.0.0.1".to_owned(),
        "/s/settings.toml".to_owned(),
        TerminalStyleConfig::new(false),
    );
    let links: Vec<String> = links.iter().map(|&link| link.to_owned()).collect();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(&links, &mut out, &mut err).await;
    (
        code,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

#[tokio::test]
async fn prints_one_line_per_link_in_order() {
    let links = ["idea://open?file=/a", "goland://open?file=/b", "idea://open?file=/c"];
    let (code, out, err) = run(&links).await;
    assert_eq!(code, 0);
    assert!(err.is_empty(), "{err}");
    let expected: Vec<String> = links
        .iter()
        .map(|link| open_url("127.0.0.1", &prepare(link)))
        .collect();
    assert_eq!(out.lines().collect::<Vec<_>>(), expected);
}

#[tokio::test]
async fn one_refused_link_prints_no_link_and_its_reason() {
    let (code, out, err) = run(&["idea://open?file=/a", "slack://channel?id=C1"]).await;
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(
        err,
        "[fail] scheme not allowed: slack\n  add \"slack\" to allowed_schemes in /s/settings.toml\n"
    );
}
