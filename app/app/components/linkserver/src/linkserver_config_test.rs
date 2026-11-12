use std::path::PathBuf;
use std::sync::Arc;

use super::LinkserverConfig;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::repositories::agent_definition_repository::MockAgentDefinitionRepository;
use crate::core::application::repositories::settings_repository::MockSettingsRepository;
use crate::core::application::services::agent_supervisor_service::MockAgentSupervisorService;
use crate::core::application::services::server_health_service::MockServerHealthService;
use crate::linkserver_ports::LinkserverPorts;
use crate::linkserver_programs::LinkserverPrograms;
use crate::linkserver_setup::LinkserverSetup;

fn setup() -> LinkserverSetup {
    LinkserverSetup {
        settings_dir: PathBuf::from("/nonexistent/settings"),
        log_dir: PathBuf::from("/nonexistent/logs"),
        agents_dir: PathBuf::from("/nonexistent/agents"),
        program: PathBuf::from("/bin/tool"),
        label: "io.example.agent".to_owned(),
        host: "127.0.0.1".to_owned(),
        hosts: vec!["127.0.0.1".to_owned()],
        version: "0.1.0".to_owned(),
        uid: 501,
        programs: LinkserverPrograms {
            open: PathBuf::from("/nonexistent/open"),
            launchctl: PathBuf::from("/nonexistent/launchctl"),
            osascript: PathBuf::from("/nonexistent/osascript"),
            lsappinfo: PathBuf::from("/nonexistent/lsappinfo"),
        },
    }
}

#[tokio::test]
async fn status_checks_the_default_port_when_no_settings_exist() {
    let setup = setup();
    let mut ports = LinkserverPorts::macos(&setup);
    let mut settings = MockSettingsRepository::new();
    settings.expect_read_settings().returning(|| Ok(None));
    let mut agents = MockAgentDefinitionRepository::new();
    agents.expect_exists().returning(|_| Ok(true));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(true));
    let mut health = MockServerHealthService::new();
    health
        .expect_is_answering()
        .withf(|port| port.value() == 7777)
        .returning(|_| true);
    ports.settings = Arc::new(settings);
    ports.agents = Arc::new(agents);
    ports.supervisor = Arc::new(supervisor);
    ports.health = Arc::new(health);
    let root = LinkserverConfig::new(setup, ports, TerminalStyleConfig::new(false));
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = root.status_command().await.run(&mut out, &mut err).await;
    let out = String::from_utf8(out).unwrap();
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("answering on 127.0.0.1:7777"), "{out}");
}
