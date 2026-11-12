use std::sync::Arc;

use super::StopAgentCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::use_cases::stop_agent::MockStopAgent;

async fn run(result: Result<(), AgentControlError>) -> (u8, String, String) {
    let mut use_case = MockStopAgent::new();
    use_case.expect_execute().return_once(move || result);
    let command = StopAgentCommand::new(Arc::new(use_case), TerminalStyleConfig::new(false));
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(&mut out, &mut err).await;
    (
        code,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

#[tokio::test]
async fn a_stop_says_until_when() {
    let (code, out, _) = run(Ok(())).await;
    assert_eq!(code, 0);
    assert!(
        out.contains("agent stopped until the next start or login"),
        "{out}"
    );
}

#[tokio::test]
async fn a_missing_agent_fails_with_the_hint() {
    let (code, _, err) = run(Err(AgentControlError::NotInstalled)).await;
    assert_eq!(code, 1);
    assert!(err.contains("http2scheme install"), "{err}");
}
