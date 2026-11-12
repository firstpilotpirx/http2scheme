//! Builds the component's use cases, commands and routers from what the app hands in.

use std::sync::Arc;

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
use crate::adapters::primary::linkreplay::inprocess::replay_link_handler::ReplayLinkHandler;
use crate::adapters::primary::linksharing::cli::prepare_link_command::PrepareLinkCommand;
use crate::adapters::primary::serving::cli::serve_command::{ServeCommand, ServerFactory};
use crate::core::application::use_cases::check_agent_status_use_case::CheckAgentStatusUseCase;
use crate::core::application::use_cases::install_agent_use_case::InstallAgentUseCase;
use crate::core::application::use_cases::open_link_use_case::OpenLinkUseCase;
use crate::core::application::use_cases::prepare_link_use_case::PrepareLinkUseCase;
use crate::core::application::use_cases::prepare_settings_use_case::PrepareSettingsUseCase;
use crate::core::application::use_cases::replay_link_use_case::ReplayLinkUseCase;
use crate::core::application::use_cases::start_agent_use_case::StartAgentUseCase;
use crate::core::application::use_cases::stop_agent_use_case::StopAgentUseCase;
use crate::core::application::use_cases::uninstall_agent_use_case::UninstallAgentUseCase;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;
use crate::linkserver_ports::LinkserverPorts;
use crate::linkserver_setup::LinkserverSetup;
use crate::open_link_ports::OpenLinkPorts;

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

    /// The ▶ of the menu bar app's Link History window.
    #[must_use]
    pub fn replay_handler(&self) -> ReplayLinkHandler {
        ReplayLinkHandler::new(Arc::new(ReplayLinkUseCase::new(
            self.prepare_settings(),
            self.ports.opener.clone(),
            self.ports.attempts.clone(),
        )))
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
