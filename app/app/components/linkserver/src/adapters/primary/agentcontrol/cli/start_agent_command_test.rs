use std::sync::Arc;

use super::StartAgentCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::use_cases::start_agent::MockStartAgent;

async fn run(result: Result<(), AgentControlError>) -> (u8, String, String) {
    let mut use_case = MockStartAgent::new();
    use_case.expect_execute().return_once(move || result);
    let command = StartAgentCommand::new(Arc::new(use_case), TerminalStyleConfig::new(false));
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(&mut out, &mut err).await;
    (
        code,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

#[tokio::test]
async fn a_start_says_started() {
    let (code, out, _) = run(Ok(())).await;
    assert_eq!((code, out.trim()), (0, "[ok] agent started"));
}

#[tokio::test]
async fn a_failed_start_prints_the_reason() {
    let error = AgentControlError::Supervisor {
        reason: "busy".to_owned(),
    };
    let (code, _, err) = run(Err(error)).await;
    assert_eq!(code, 1);
    assert!(err.contains("launchctl: busy"), "{err}");
}
