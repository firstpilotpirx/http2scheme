use std::sync::Arc;

use super::UninstallAgentCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::use_cases::uninstall_agent::MockUninstallAgent;

async fn run(result: Result<bool, AgentControlError>) -> (u8, String) {
    let mut use_case = MockUninstallAgent::new();
    use_case.expect_execute().return_once(move || result);
    let command = UninstallAgentCommand::new(Arc::new(use_case), TerminalStyleConfig::new(false));
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(&mut out, &mut err).await;
    out.extend(err);
    (code, String::from_utf8(out).unwrap())
}

#[tokio::test]
async fn a_removal_says_what_stays() {
    let (code, text) = run(Ok(true)).await;
    assert_eq!(code, 0);
    assert!(text.contains("settings, key and logs are kept"), "{text}");
}

#[tokio::test]
async fn nothing_to_remove_still_succeeds() {
    let (code, text) = run(Ok(false)).await;
    assert_eq!(code, 0);
    assert!(text.contains("nothing was installed"), "{text}");
}

#[tokio::test]
async fn a_failure_ends_with_status_1() {
    let error = AgentControlError::Io {
        reason: "denied".to_owned(),
    };
    assert_eq!(run(Err(error)).await.0, 1);
}
