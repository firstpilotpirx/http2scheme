//! The secondary ports of the component and their macOS adapters.

use std::sync::Arc;
use std::time::Duration;

use crate::adapters::secondary::agentcontrol::filesystem::filesystem_agent_definition_repository::FilesystemAgentDefinitionRepository;
use crate::adapters::secondary::agentcontrol::launchd::launchd_agent_supervisor_gateway::LaunchdAgentSupervisorGateway;
use crate::adapters::secondary::browsertabs::applescript::apple_script_browser_tab_closer_gateway::AppleScriptBrowserTabCloserGateway;
use crate::adapters::secondary::linkopening::launchservices::launch_services_scheme_link_opener_gateway::LaunchServicesSchemeLinkOpenerGateway;
use crate::adapters::secondary::requestlog::filesystem::filesystem_open_attempt_repository::FilesystemOpenAttemptRepository;
use crate::adapters::secondary::secret::osrandom::os_random_secret_generator_gateway::OsRandomSecretGeneratorGateway;
use crate::adapters::secondary::serverhealth::http::http_server_health_client::HttpServerHealthClient;
use crate::adapters::secondary::settings::filesystem::filesystem_settings_repository::FilesystemSettingsRepository;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::application::repositories::settings_repository::SettingsRepository;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;
use crate::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use crate::core::application::services::secret_generator_service::SecretGeneratorService;
use crate::core::application::services::server_health_service::ServerHealthService;
use crate::linkserver_setup::LinkserverSetup;

const TAB_CLOSE_DELAY: Duration = Duration::from_millis(300);
const HEALTH_TIMEOUT: Duration = Duration::from_secs(1);

/// The secondary ports every use case drives.
#[derive(Clone)]
pub struct LinkserverPorts {
    /// The settings and key files.
    pub settings: Arc<dyn SettingsRepository>,
    /// The key source.
    pub secrets: Arc<dyn SecretGeneratorService>,
    /// Opens scheme links.
    pub opener: Arc<dyn SchemeLinkOpenerService>,
    /// Closes the browser tabs.
    pub closer: Arc<dyn BrowserTabCloserService>,
    /// Records open attempts.
    pub attempts: Arc<dyn OpenAttemptRepository>,
    /// The agent definitions.
    pub agents: Arc<dyn AgentDefinitionRepository>,
    /// Loads and unloads the agent.
    pub supervisor: Arc<dyn AgentSupervisorService>,
    /// Asks the server whether it answers.
    pub health: Arc<dyn ServerHealthService>,
}

impl LinkserverPorts {
    /// The macOS adapters for the setup.
    #[must_use]
    pub fn macos(setup: &LinkserverSetup) -> Self {
        Self {
            settings: Arc::new(FilesystemSettingsRepository::new(
                setup.settings_dir.clone(),
            )),
            secrets: Arc::new(OsRandomSecretGeneratorGateway),
            opener: Arc::new(LaunchServicesSchemeLinkOpenerGateway::new(
                setup.open_program.clone(),
            )),
            closer: Arc::new(AppleScriptBrowserTabCloserGateway::new(
                setup.osascript_program.clone(),
                setup.hosts.clone(),
                TAB_CLOSE_DELAY,
            )),
            attempts: Arc::new(FilesystemOpenAttemptRepository::new(setup.request_log())),
            agents: Arc::new(FilesystemAgentDefinitionRepository::new(
                setup.agents_dir.clone(),
            )),
            supervisor: Arc::new(LaunchdAgentSupervisorGateway::new(
                setup.launchctl_program.clone(),
                setup.uid,
                setup.agents_dir.clone(),
            )),
            health: Arc::new(HttpServerHealthClient::new(
                setup.host.clone(),
                HEALTH_TIMEOUT,
            )),
        }
    }
}

#[cfg(test)]
#[path = "linkserver_ports_test.rs"]
mod linkserver_ports_test;
