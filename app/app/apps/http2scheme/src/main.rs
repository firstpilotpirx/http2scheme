//! `http2scheme` entry point: reads the machine's environment, builds the component and runs one command.

#![forbid(unsafe_code)]

use std::io::IsTerminal as _;
use std::os::unix::fs::MetadataExt as _;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use linkserver::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use linkserver::linkserver_config::LinkserverConfig;
use linkserver::linkserver_ports::LinkserverPorts;
use linkserver::linkserver_setup::LinkserverSetup;

const LABEL: &str = "io.github.firstpilotpirx.http2scheme";
const HOST: &str = "127.0.0.1";
const HOSTS: [&str; 2] = ["127.0.0.1", "localhost"];
const SETTINGS_DIR: &str = "Library/Application Support/http2scheme";
const LOG_DIR: &str = "Library/Logs/http2scheme";
const AGENTS_DIR: &str = "Library/LaunchAgents";
const OPEN_PROGRAM: (&str, &str) = ("HTTP2SCHEME_OPEN_PROGRAM", "/usr/bin/open");
const LAUNCHCTL_PROGRAM: (&str, &str) = ("HTTP2SCHEME_LAUNCHCTL_PROGRAM", "/bin/launchctl");
const OSASCRIPT_PROGRAM: (&str, &str) = ("HTTP2SCHEME_OSASCRIPT_PROGRAM", "/usr/bin/osascript");

/// Opens custom URL scheme links (idea://, goland://, slack://…) from http links on 127.0.0.1.
#[derive(Parser)]
#[command(name = "http2scheme", version)]
struct Cli {
    /// Print without colors.
    #[arg(long, global = true)]
    no_color: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the server in the foreground until it is stopped.
    Serve,
    /// Print the http link that opens a scheme link.
    Link {
        /// The scheme link, for example `idea://open?file=/a/B.java&line=3`.
        link: String,
    },
    /// Run the server at login and restart it after a crash.
    Install,
    /// Stop the server and remove it from login; settings, key and logs stay.
    Uninstall,
    /// Start the installed server.
    Start,
    /// Stop the installed server until the next start or login.
    Stop,
    /// Show whether the server is installed, loaded and answering.
    Status,
}

fn program((variable, default): (&str, &str)) -> PathBuf {
    PathBuf::from(std::env::var(variable).unwrap_or_else(|_| default.to_owned()))
}

fn setup() -> Result<LinkserverSetup, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME is not set".to_owned())?;
    let home = PathBuf::from(home);
    let uid = std::fs::metadata(&home)
        .map_err(|error| format!("{}: {error}", home.display()))?
        .uid();
    let current = std::env::current_exe().map_err(|error| format!("own program path: {error}"))?;
    Ok(LinkserverSetup {
        settings_dir: home.join(SETTINGS_DIR),
        log_dir: home.join(LOG_DIR),
        agents_dir: home.join(AGENTS_DIR),
        program: current,
        label: LABEL.to_owned(),
        host: HOST.to_owned(),
        hosts: HOSTS.iter().map(|&host| host.to_owned()).collect(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        uid,
        open_program: program(OPEN_PROGRAM),
        launchctl_program: program(LAUNCHCTL_PROGRAM),
        osascript_program: program(OSASCRIPT_PROGRAM),
    })
}

async fn stopped() {
    let Ok(mut terminate) =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
    else {
        let _ = tokio::signal::ctrl_c().await;
        return;
    };
    tokio::select! {
        _ = terminate.recv() => {}
        _ = tokio::signal::ctrl_c() => {}
    }
}

async fn run(command: Command, root: LinkserverConfig) -> u8 {
    let (out, err) = (&mut std::io::stdout(), &mut std::io::stderr());
    match command {
        Command::Serve => root.serve_command().run(stopped(), out, err).await,
        Command::Link { link } => root.link_command().run(&link, out, err).await,
        Command::Install => root.install_command().await.run(out, err).await,
        Command::Uninstall => root.uninstall_command().run(out, err).await,
        Command::Start => root.start_command().run(out, err).await,
        Command::Stop => root.stop_command().run(out, err).await,
        Command::Status => root.status_command().await.run(out, err).await,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let style =
        TerminalStyleConfig::detect(cli.no_color, std::io::stdout().is_terminal(), |name| {
            std::env::var(name).ok()
        });
    let setup = match setup() {
        Ok(setup) => setup,
        Err(reason) => {
            eprintln!("{}", style.fail(&reason));
            return ExitCode::from(1);
        }
    };
    let ports = LinkserverPorts::macos(&setup);
    let root = LinkserverConfig::new(setup, ports, style);
    ExitCode::from(run(cli.command, root).await)
}
