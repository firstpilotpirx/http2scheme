use std::sync::Arc;

use super::AgentStatusCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::check_agent_status::MockCheckAgentStatus;
use crate::core::domain::agent_status_value_object::AgentStatusValueObject;

async fn run(status: AgentStatusValueObject) -> (u8, String) {
    let mut use_case = MockCheckAgentStatus::new();
    use_case.expect_execute().return_once(move || Ok(status));
    let command = AgentStatusCommand::new(
        Arc::new(use_case),
        "127.0.0.1:7777".to_owned(),
        "/logs/agent.err.log".to_owned(),
        TerminalStyleConfig::new(false),
    );
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(&mut out, &mut err).await;
    (code, String::from_utf8(out).unwrap())
}

#[tokio::test]
async fn a_healthy_agent_prints_three_good_lines() {
    let (code, out) = run(AgentStatusValueObject::new(true, true, true)).await;
    assert_eq!(code, 0);
    assert_eq!(out.lines().count(), 3, "{out}");
    assert!(!out.contains("see "), "{out}");
}

#[tokio::test]
async fn a_silent_server_points_at_the_error_log() {
    let (code, out) = run(AgentStatusValueObject::new(true, true, false)).await;
    assert_eq!(code, 1);
    assert!(out.contains("not answering on 127.0.0.1:7777"), "{out}");
    assert!(out.contains("see /logs/agent.err.log"), "{out}");
}
