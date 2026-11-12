use std::sync::Arc;

use super::{ServeCommand, ServerFactory};
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::use_cases::prepare_settings::MockPrepareSettings;

#[tokio::test]
async fn a_settings_failure_stops_before_listening() {
    let mut use_case = MockPrepareSettings::new();
    use_case.expect_execute().return_once(|| {
        Err(SettingsError::Io {
            path: "/s/key".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let factory: ServerFactory = Arc::new(|_, _| unreachable!("no server is built"));
    let command = ServeCommand::new(
        Arc::new(use_case),
        factory,
        "0.1.0".to_owned(),
        "/logs/requests.log".to_owned(),
        TerminalStyleConfig::new(false),
    );
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command
        .run(std::future::ready(()), &mut out, &mut err)
        .await;
    assert_eq!((code, out.len()), (1, 0));
    let err = String::from_utf8(err).unwrap();
    assert!(err.contains("/s/key: denied"), "{err}");
}
