//! Builds the component's use cases, commands and routers from what the app hands in.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;

use crate::adapters::primary::agentcontrol::cli::agent_status_command::AgentStatusCommand;
use crate::adapters::primary::agentcontrol::cli::install_agent_command::InstallAgentCommand;
use crate::adapters::primary::agentcontrol::cli::start_agent_command::StartAgentCommand;
use crate::adapters::primary::agentcontrol::cli::stop_agent_command::StopAgentCommand;
use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::adapters::primary::agentcontrol::cli::uninstall_agent_command::UninstallAgentCommand;
use crate::adapters::primary::linkopening::http::axum_config::AxumConfig;
use crate::adapters::primary::linkopening::http::open_link_handler::OpenLinkHandler;
use crate::adapters::primary::linkopening::http::open_link_response::OpenLinkResponse;
use crate::adapters::primary::linksharing::cli::prepare_link_command::PrepareLinkCommand;
use crate::adapters::primary::serving::cli::serve_command::{ServeCommand, ServerFactory};
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
use crate::core::application::use_cases::check_agent_status_use_case::CheckAgentStatusUseCase;
use crate::core::application::use_cases::install_agent_use_case::InstallAgentUseCase;
use crate::core::application::use_cases::open_link_use_case::OpenLinkUseCase;
use crate::core::application::use_cases::prepare_link_use_case::PrepareLinkUseCase;
use crate::core::application::use_cases::prepare_settings_use_case::PrepareSettingsUseCase;
use crate::core::application::use_cases::start_agent_use_case::StartAgentUseCase;
use crate::core::application::use_cases::stop_agent_use_case::StopAgentUseCase;
use crate::core::application::use_cases::uninstall_agent_use_case::UninstallAgentUseCase;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

const SETTINGS_FILE: &str = "settings.toml";
const REQUEST_LOG: &str = "requests.log";
const TAB_CLOSE_DELAY: Duration = Duration::from_millis(300);
const HEALTH_TIMEOUT: Duration = Duration::from_secs(1);

/// The folders, names and programs the app hands in; the component holds no machine literal.
#[derive(Debug, Clone)]
pub struct LinkserverSetup {
    /// The folder of `settings.toml` and `key`.
    pub settings_dir: PathBuf,
    /// The folder of the request log and the agent's output.
    pub log_dir: PathBuf,
    /// The folder of the per-user agent definitions.
    pub agents_dir: PathBuf,
    /// The program the agent runs.
    pub program: PathBuf,
    /// The agent label.
    pub label: String,
    /// The address the server binds and the printed links name.
    pub host: String,
    /// The host names the server serves and whose tabs it closes.
    pub hosts: Vec<String>,
    /// The program version.
    pub version: String,
    /// The user id of the agent's launchd domain.
    pub uid: u32,
    /// The macOS link opener.
    pub open_program: PathBuf,
    /// The launchd control program.
    pub launchctl_program: PathBuf,
    /// The `AppleScript` runner.
    pub osascript_program: PathBuf,
}

impl LinkserverSetup {
    fn settings_file(&self) -> String {
        self.settings_dir.join(SETTINGS_FILE).display().to_string()
    }

    fn request_log(&self) -> PathBuf {
        self.log_dir.join(REQUEST_LOG)
    }

    fn definition(&self) -> AgentDefinitionValueObject {
        AgentDefinitionValueObject::new(
            self.label.clone(),
            self.program.clone(),
            self.log_dir.clone(),
        )
    }

    fn pages(&self) -> OpenLinkResponse {
        OpenLinkResponse::new(
            self.settings_file(),
            self.request_log().display().to_string(),
        )
    }
}

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

/// The secondary ports the open-link use case drives.
pub struct OpenLinkPorts {
    /// Opens scheme links.
    pub opener: Arc<dyn SchemeLinkOpenerService>,
    /// Closes the browser tabs.
    pub closer: Arc<dyn BrowserTabCloserService>,
    /// Records open attempts.
    pub attempts: Arc<dyn OpenAttemptRepository>,
}

/// The component's composition root.
pub struct LinkserverConfig {
    setup: LinkserverSetup,
    ports: LinkserverPorts,
    style: TerminalStyleConfig,
}

impl LinkserverConfig {
    /// A root over the setup and the ports, printing with `style`.
    #[must_use]
    pub const fn new(
        setup: LinkserverSetup,
        ports: LinkserverPorts,
        style: TerminalStyleConfig,
    ) -> Self {
        Self {
            setup,
            ports,
            style,
        }
    }

    /// The HTTP router over the open-link use case built from the settings, the key and the ports.
    pub fn open_link_router(
        settings: ServerSettingsValueObject,
        key: AccessKeyValueObject,
        ports: OpenLinkPorts,
        http: &AxumConfig,
        pages: OpenLinkResponse,
    ) -> Router {
        let use_case =
            OpenLinkUseCase::new(settings, key, ports.opener, ports.closer, ports.attempts);
        http.router(OpenLinkHandler::new(Arc::new(use_case), pages))
    }

    fn prepare_settings(&self) -> Arc<PrepareSettingsUseCase> {
        Arc::new(PrepareSettingsUseCase::new(
            self.ports.settings.clone(),
            self.ports.secrets.clone(),
        ))
    }

    /// `link`.
    #[must_use]
    pub fn link_command(&self) -> PrepareLinkCommand {
        let use_case = Arc::new(PrepareLinkUseCase::new(self.prepare_settings()));
        PrepareLinkCommand::new(
            use_case,
            self.setup.host.clone(),
            self.setup.settings_file(),
            self.style,
        )
    }

    /// `serve`.
    #[must_use]
    pub fn serve_command(&self) -> ServeCommand {
        let (setup, ports) = (self.setup.clone(), self.ports.clone());
        let factory: ServerFactory = Arc::new(move |settings: ServerSettingsValueObject, key| {
            let http = AxumConfig::new(setup.host.clone(), setup.hosts.clone(), settings.port());
            let open_ports = OpenLinkPorts {
                opener: ports.opener.clone(),
                closer: ports.closer.clone(),
                attempts: ports.attempts.clone(),
            };
            let router = Self::open_link_router(settings, key, open_ports, &http, setup.pages());
            (http, router)
        });
        let log = self.setup.request_log().display().to_string();
        ServeCommand::new(
            self.prepare_settings(),
            factory,
            self.setup.version.clone(),
            log,
            self.style,
        )
    }

    async fn port(&self) -> PortValueObject {
        match self.ports.settings.read_settings().await {
            Ok(Some(settings)) => settings.port(),
            _ => PortValueObject::default(),
        }
    }

    /// `install`; the reported address uses the port in the settings.
    pub async fn install_command(&self) -> InstallAgentCommand {
        let definition = self.setup.definition();
        let file = self
            .setup
            .agents_dir
            .join(format!("{}.plist", self.setup.label))
            .display()
            .to_string();
        let address = format!("http://{}:{}", self.setup.host, self.port().await.value());
        let program = self.setup.program.display().to_string();
        let use_case = InstallAgentUseCase::new(
            definition,
            self.ports.agents.clone(),
            self.ports.supervisor.clone(),
        );
        InstallAgentCommand::new(Arc::new(use_case), file, program, address, self.style)
    }

    /// `uninstall`.
    #[must_use]
    pub fn uninstall_command(&self) -> UninstallAgentCommand {
        let use_case = UninstallAgentUseCase::new(
            self.setup.label.clone(),
            self.ports.agents.clone(),
            self.ports.supervisor.clone(),
        );
        UninstallAgentCommand::new(Arc::new(use_case), self.style)
    }

    /// `start`.
    #[must_use]
    pub fn start_command(&self) -> StartAgentCommand {
        let use_case = StartAgentUseCase::new(
            self.setup.definition(),
            self.ports.agents.clone(),
            self.ports.supervisor.clone(),
        );
        StartAgentCommand::new(Arc::new(use_case), self.style)
    }

    /// `stop`.
    #[must_use]
    pub fn stop_command(&self) -> StopAgentCommand {
        let use_case = StopAgentUseCase::new(
            self.setup.label.clone(),
            self.ports.agents.clone(),
            self.ports.supervisor.clone(),
        );
        StopAgentCommand::new(Arc::new(use_case), self.style)
    }

    /// `status`; it checks the port in the settings.
    pub async fn status_command(&self) -> AgentStatusCommand {
        let port = self.port().await;
        let use_case = CheckAgentStatusUseCase::new(
            self.setup.label.clone(),
            port,
            self.ports.agents.clone(),
            self.ports.supervisor.clone(),
            self.ports.health.clone(),
        );
        let address = format!("{}:{}", self.setup.host, port.value());
        let error_log = self.setup.definition().stderr_log().display().to_string();
        AgentStatusCommand::new(Arc::new(use_case), address, error_log, self.style)
    }
}

#[cfg(test)]
#[path = "linkserver_config_test.rs"]
mod linkserver_config_test;
