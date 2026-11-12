use std::sync::Arc;

use super::InstallAgentCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::use_cases::install_agent::MockInstallAgent;

async fn run(result: Result<(), AgentControlError>) -> (u8, String, String) {
    let mut use_case = MockInstallAgent::new();
    use_case.expect_execute().return_once(move || result);
    let command = InstallAgentCommand::new(
        Arc::new(use_case),
        "/a/agent.plist".to_owned(),
        "/bin/tool".to_owned(),
        "http://127.0.0.1:7777".to_owned(),
        TerminalStyleConfig::new(false),
    );
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = command.run(&mut out, &mut err).await;
    (
        code,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

#[tokio::test]
async fn an_install_prints_the_file_the_program_and_the_address() {
    let (code, out, _) = run(Ok(())).await;
    assert_eq!(code, 0);
    let lines: Vec<&str> = out.lines().collect();
    assert!(lines[0].ends_with("/a/agent.plist"), "{out}");
    assert!(lines[1].ends_with("/bin/tool serve"), "{out}");
    assert!(
        lines[2].ends_with("installed and running on http://127.0.0.1:7777"),
        "{out}"
    );
}

#[tokio::test]
async fn a_failed_install_prints_only_the_reason() {
    let error = AgentControlError::Supervisor {
        reason: "denied".to_owned(),
    };
    let (code, out, err) = run(Err(error)).await;
    assert_eq!((code, out.as_str()), (1, ""));
    assert!(err.contains("denied"), "{err}");
}
