use super::AgentControlError;

#[test]
fn not_installed_gives_the_hint() {
    assert_eq!(
        AgentControlError::NotInstalled.to_string(),
        "the agent is not installed; run: http2scheme install"
    );
}

#[test]
fn other_errors_name_the_step() {
    assert_eq!(AgentControlError::Io { reason: "denied".to_owned() }.to_string(), "agent file: denied");
    assert_eq!(AgentControlError::Supervisor { reason: "exit 5".to_owned() }.to_string(), "launchctl: exit 5");
}
