//! A tiny binary for the CLI adapter integration tests: it mounts one real command over a fake use case.
//!
//! `linkserver-cli-fixture <command> <case> [argument] [--no-color]`

#![forbid(unsafe_code)]

use std::io::IsTerminal as _;
use std::process::ExitCode;
use std::sync::Arc;

use async_trait::async_trait;
use linkserver::adapters::primary::agentcontrol::cli::agent_status_command::AgentStatusCommand;
use linkserver::adapters::primary::agentcontrol::cli::install_agent_command::InstallAgentCommand;
use linkserver::adapters::primary::agentcontrol::cli::start_agent_command::StartAgentCommand;
use linkserver::adapters::primary::agentcontrol::cli::stop_agent_command::StopAgentCommand;
use linkserver::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use linkserver::adapters::primary::agentcontrol::cli::uninstall_agent_command::UninstallAgentCommand;
use linkserver::adapters::primary::linkopening::http::axum_config::AxumConfig;
use linkserver::adapters::primary::linksharing::cli::prepare_link_command::PrepareLinkCommand;
use linkserver::adapters::primary::serving::cli::serve_command::{ServeCommand, ServerFactory};
use linkserver::core::application::exceptions::agent_control_error::AgentControlError;
use linkserver::core::application::exceptions::prepare_link_error::PrepareLinkError;
use linkserver::core::application::exceptions::settings_error::SettingsError;
use linkserver::core::application::use_cases::check_agent_status::CheckAgentStatus;
use linkserver::core::application::use_cases::install_agent::InstallAgent;
use linkserver::core::application::use_cases::prepare_link::PrepareLink;
use linkserver::core::application::use_cases::prepare_settings::PrepareSettings;
use linkserver::core::application::use_cases::start_agent::StartAgent;
use linkserver::core::application::use_cases::stop_agent::StopAgent;
use linkserver::core::application::use_cases::uninstall_agent::UninstallAgent;
use linkserver::core::domain::access_key_value_object::AccessKeyValueObject;
use linkserver::core::domain::agent_status_value_object::AgentStatusValueObject;
use linkserver::core::domain::link_rejected_error::LinkRejectedError;
use linkserver::core::domain::port_value_object::PortValueObject;
use linkserver::core::domain::prepared_link_value_object::PreparedLinkValueObject;
use linkserver::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use linkserver::core::domain::scheme_link_value_object::SchemeLinkValueObject;
use linkserver::core::domain::server_settings_value_object::ServerSettingsValueObject;
use linkserver::core::domain::settings_invalid_error::SettingsInvalidError;

/// One fake that answers every use case by the case name.
struct Fake {
    case: String,
    argument: String,
}

impl Fake {
    fn agent_result(&self) -> Result<(), AgentControlError> {
        match self.case.as_str() {
            "not-installed" => Err(AgentControlError::NotInstalled),
            "fail" => Err(AgentControlError::Supervisor {
                reason: "exit 5".to_owned(),
            }),
            _ => Ok(()),
        }
    }

    fn port(&self) -> PortValueObject {
        self.argument
            .parse()
            .ok()
            .and_then(PortValueObject::new)
            .unwrap_or_default()
    }
}

#[async_trait]
impl PrepareLink for Fake {
    async fn execute(&self, link: &str) -> Result<PreparedLinkValueObject, PrepareLinkError> {
        if self.case == "settings" {
            return Err(SettingsError::Io {
                path: "/home/key".to_owned(),
                reason: "denied".to_owned(),
            }
            .into());
        }
        let link = SchemeLinkValueObject::parse(link)?;
        if self.case == "not-allowed" {
            return Err(LinkRejectedError::SchemeNotAllowed {
                scheme: link.scheme().to_string(),
            }
            .into());
        }
        Ok(PreparedLinkValueObject::new(
            PortValueObject::default(),
            AccessKeyValueObject::from_bytes([1; 32]),
            link,
        ))
    }
}

#[async_trait]
impl PrepareSettings for Fake {
    async fn execute(
        &self,
    ) -> Result<(ServerSettingsValueObject, AccessKeyValueObject), SettingsError> {
        if self.case == "broken" {
            return Err(SettingsInvalidError::new(
                "/home/settings.toml",
                "port",
                "must be between 1024 and 65535",
            )
            .into());
        }
        let settings =
            ServerSettingsValueObject::new(self.port(), SchemeAllowlistPolicy::defaults(), true);
        Ok((settings, AccessKeyValueObject::from_bytes([1; 32])))
    }
}

#[async_trait]
impl InstallAgent for Fake {
    async fn execute(&self) -> Result<(), AgentControlError> {
        self.agent_result()
    }
}

#[async_trait]
impl StartAgent for Fake {
    async fn execute(&self) -> Result<(), AgentControlError> {
        self.agent_result()
    }
}

#[async_trait]
impl StopAgent for Fake {
    async fn execute(&self) -> Result<(), AgentControlError> {
        self.agent_result()
    }
}

#[async_trait]
impl UninstallAgent for Fake {
    async fn execute(&self) -> Result<bool, AgentControlError> {
        self.agent_result().map(|()| self.case == "removed")
    }
}

#[async_trait]
impl CheckAgentStatus for Fake {
    async fn execute(&self) -> Result<AgentStatusValueObject, AgentControlError> {
        let healthy = self.case == "healthy";
        Ok(AgentStatusValueObject::new(true, healthy, healthy))
    }
}

fn factory() -> ServerFactory {
    Arc::new(|settings: ServerSettingsValueObject, _key| {
        let http = AxumConfig::new(
            "127.0.0.1".to_owned(),
            vec!["127.0.0.1".to_owned()],
            settings.port(),
        );
        let router = axum::Router::new().route("/health", axum::routing::get(|| async { "ok" }));
        (http, router)
    })
}

async fn terminated() {
    if let Ok(mut signal) =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
    {
        signal.recv().await;
    }
}

async fn run_link_or_serve(command: &str, fake: Arc<Fake>, style: TerminalStyleConfig) -> u8 {
    let (out, err) = (&mut std::io::stdout(), &mut std::io::stderr());
    let argument = fake.argument.clone();
    if command == "link" {
        let settings = "/home/settings.toml".to_owned();
        return PrepareLinkCommand::new(fake, "127.0.0.1".to_owned(), settings, style)
            .run(&argument, out, err)
            .await;
    }
    let log = "/home/requests.log".to_owned();
    ServeCommand::new(fake, factory(), "0.1.0".to_owned(), log, style)
        .run(terminated(), out, err)
        .await
}

async fn run(command: &str, fake: Arc<Fake>, style: TerminalStyleConfig) -> u8 {
    let (out, err) = (&mut std::io::stdout(), &mut std::io::stderr());
    match command {
        "link" | "serve" => run_link_or_serve(command, fake, style).await,
        "install" => {
            let (file, program) = (
                "/home/agent.plist".to_owned(),
                "/bin/http2scheme".to_owned(),
            );
            let address = "http://127.0.0.1:7777".to_owned();
            InstallAgentCommand::new(fake, file, program, address, style)
                .run(out, err)
                .await
        }
        "uninstall" => UninstallAgentCommand::new(fake, style).run(out, err).await,
        "start" => StartAgentCommand::new(fake, style).run(out, err).await,
        "stop" => StopAgentCommand::new(fake, style).run(out, err).await,
        "status" => {
            let (address, log) = (
                "127.0.0.1:7777".to_owned(),
                "/home/agent.err.log".to_owned(),
            );
            AgentStatusCommand::new(fake, address, log, style)
                .run(out, err)
                .await
        }
        _ => 2,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args()
        .skip(1)
        .filter(|argument| argument != "--no-color")
        .collect();
    let no_color = std::env::args().any(|argument| argument == "--no-color");
    let style = TerminalStyleConfig::detect(no_color, std::io::stdout().is_terminal(), |name| {
        std::env::var(name).ok()
    });
    let [command, case, rest @ ..] = arguments.as_slice() else {
        return ExitCode::from(2);
    };
    let fake = Arc::new(Fake {
        case: case.clone(),
        argument: rest.first().cloned().unwrap_or_default(),
    });
    ExitCode::from(run(command, fake, style).await)
}
