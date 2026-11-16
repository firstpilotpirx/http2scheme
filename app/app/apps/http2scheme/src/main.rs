//! `http2scheme` entry point: reads the machine's environment, builds the components and runs one
//! command, or, started from its app bundle with no command, the menu bar app (design D14).

#![forbid(unsafe_code)]

use std::io::IsTerminal as _;
use std::os::unix::fs::MetadataExt as _;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use clap::{CommandFactory as _, Parser, Subcommand};
use linkserver::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use linkserver::linkserver_config::LinkserverConfig;
use linkserver::linkserver_ports::LinkserverPorts;
use linkserver::linkserver_programs::LinkserverPrograms;
use linkserver::linkserver_setup::LinkserverSetup;
use menubar::menubar_config::MenubarConfig;
use menubar::menubar_hooks::{self, MenubarHooks, SkillAction as SkillHook, SkillText};
use menubar::menubar_setup::MenubarSetup;
use tokio::runtime::{Handle, Runtime};

const LABEL: &str = "io.github.firstpilotpirx.http2scheme";
const HOST: &str = "127.0.0.1";
const HOSTS: [&str; 2] = ["127.0.0.1", "localhost"];
const SETTINGS_DIR: &str = "Library/Application Support/http2scheme";
const LOG_DIR: &str = "Library/Logs/http2scheme";
const AGENTS_DIR: &str = "Library/LaunchAgents";
const CLAUDE_DIR: &str = ".claude";
const OPEN_PROGRAM: (&str, &str) = ("HTTP2SCHEME_OPEN_PROGRAM", "/usr/bin/open");
const LAUNCHCTL_PROGRAM: (&str, &str) = ("HTTP2SCHEME_LAUNCHCTL_PROGRAM", "/bin/launchctl");
const OSASCRIPT_PROGRAM: (&str, &str) = ("HTTP2SCHEME_OSASCRIPT_PROGRAM", "/usr/bin/osascript");
const LSAPPINFO_PROGRAM: (&str, &str) = ("HTTP2SCHEME_LSAPPINFO_PROGRAM", "/usr/bin/lsappinfo");

/// Opens custom URL scheme links (idea://, goland://, slack://…) from http links on 127.0.0.1.
#[derive(Parser)]
#[command(name = "http2scheme", version)]
struct Cli {
    /// Print without colors.
    #[arg(long, global = true)]
    no_color: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run the server in the foreground until it is stopped.
    Serve,
    /// Print the http links that open scheme links, one per line in the same order.
    Link {
        /// The scheme links, for example `idea://open?file=/a/B.java&line=3`.
        #[arg(required = true)]
        links: Vec<String>,
    },
    /// Print the agent skill that teaches coding agents to write IDE links.
    Skill {
        #[command(subcommand)]
        action: Option<SkillAction>,
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

#[derive(Subcommand)]
enum SkillAction {
    /// Write the skill to `<folder>/skills/http2scheme/SKILL.md`.
    Install {
        /// The Claude Code folder; `~/.claude` when not given.
        #[arg(long)]
        claude_dir: Option<PathBuf>,
    },
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
        claude_dir: home.join(CLAUDE_DIR),
        program: current,
        label: LABEL.to_owned(),
        host: HOST.to_owned(),
        hosts: HOSTS.iter().map(|&host| host.to_owned()).collect(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        uid,
        programs: LinkserverPrograms {
            open: program(OPEN_PROGRAM),
            launchctl: program(LAUNCHCTL_PROGRAM),
            osascript: program(OSASCRIPT_PROGRAM),
            lsappinfo: program(LSAPPINFO_PROGRAM),
        },
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
        Command::Link { links } => root.link_command().run(&links, out, err).await,
        Command::Skill { action } => skill(action, &root).await,
        service => manage(service, &root).await,
    }
}

/// `skill` prints the skill; `skill install` writes it into a Claude Code folder.
async fn skill(action: Option<SkillAction>, root: &LinkserverConfig) -> u8 {
    let (out, err) = (&mut std::io::stdout(), &mut std::io::stderr());
    match action {
        None => root.skill_command().run(out, err).await,
        Some(SkillAction::Install { claude_dir }) => {
            let command = root.install_skill_command();
            command.run(claude_dir.as_deref(), out, err).await
        }
    }
}

/// The commands that manage the background service.
async fn manage(command: Command, root: &LinkserverConfig) -> u8 {
    let (out, err) = (&mut std::io::stdout(), &mut std::io::stderr());
    match command {
        Command::Install => root.install_command().await.run(out, err).await,
        Command::Uninstall => root.uninstall_command().run(out, err).await,
        Command::Start => root.start_command().run(out, err).await,
        Command::Stop => root.stop_command().run(out, err).await,
        _ => root.status_command().await.run(out, err).await,
    }
}

/// The three skill hooks of the Agents tab, each run on the async runtime.
fn skill_hooks(handle: &Handle, root: &LinkserverConfig) -> (SkillText, SkillHook, SkillHook) {
    let (text, inspect, install) = (
        root.skill_text_handler(),
        root.inspect_skill_handler(),
        root.install_skill_handler(),
    );
    let (text_handle, inspect_handle, install_handle) =
        (handle.clone(), handle.clone(), handle.clone());
    (
        Arc::new(move || text_handle.block_on(text.text())),
        Arc::new(move |dir| {
            inspect_handle
                .block_on(inspect.inspect(dir))
                .and_then(menubar_hooks::skill_place)
        }),
        Arc::new(move |dir| {
            install_handle
                .block_on(install.install(dir))
                .and_then(menubar_hooks::skill_place)
        }),
    )
}

fn menubar(runtime: &Runtime, setup: &LinkserverSetup, root: LinkserverConfig) -> MenubarConfig {
    let root = Arc::new(root);
    let replayer = root.replay_handler();
    let handle = runtime.handle().clone();
    let replay_handle = handle.clone();
    let (skill_text, inspect_skill, install_skill) = skill_hooks(&handle, &root);
    let hooks = MenubarHooks {
        serve: Arc::new(move |shutdown, mut out, mut err| {
            let root = Arc::clone(&root);
            handle
                .spawn(async move { root.serve_command().run(shutdown, &mut out, &mut err).await });
        }),
        replay: Arc::new(move |link| replay_handle.block_on(replayer.replay(link))),
        skill_text,
        inspect_skill,
        install_skill,
    };
    MenubarConfig::macos(
        MenubarSetup {
            version: setup.version.clone(),
            settings_dir: setup.settings_dir.clone(),
            log_dir: setup.log_dir.clone(),
            claude_dir: setup.claude_dir.clone(),
            program: setup.program.clone(),
        },
        hooks,
    )
}

/// Offers the login item and refreshes the installed skill; a failure is printed, not fatal.
fn prepare_menu(runtime: &Runtime, menubar: &MenubarConfig) {
    let login = runtime
        .block_on(menubar.offer_login_item())
        .map_err(|error| error.to_string());
    let skill = menubar
        .refresh_skill()
        .map_err(|error| format!("skill refresh: {error}"));
    for reason in [login, skill].into_iter().filter_map(Result::err) {
        eprintln!("{reason}");
    }
}

fn app(runtime: &Runtime, setup: &LinkserverSetup) -> ExitCode {
    let Some(mtm) = MenubarConfig::main_thread() else {
        eprintln!("the menu bar app runs only on the main thread");
        return ExitCode::from(1);
    };
    let ports = LinkserverPorts::macos(setup);
    let root = LinkserverConfig::new(
        setup.clone(),
        ports,
        TerminalStyleConfig::detect(true, false, |_| None),
    );
    let menubar = menubar(runtime, setup, root);
    menubar.start_server();
    runtime.spawn(async {
        stopped().await;
        std::process::exit(0);
    });
    prepare_menu(runtime, &menubar);
    menubar.run(mtm);
    ExitCode::SUCCESS
}

/// The setup and the async runtime every path needs, or why they could not be made.
fn prepare() -> Result<(LinkserverSetup, Runtime), String> {
    let setup = setup()?;
    let runtime = Runtime::new().map_err(|error| format!("async runtime: {error}"))?;
    Ok((setup, runtime))
}

/// Runs `command`, or with none the menu bar app from a bundle, or else prints the help.
fn dispatch(
    command: Option<Command>,
    setup: LinkserverSetup,
    runtime: &Runtime,
    style: TerminalStyleConfig,
) -> ExitCode {
    match command {
        Some(command) => {
            let ports = LinkserverPorts::macos(&setup);
            let root = LinkserverConfig::new(setup, ports, style);
            ExitCode::from(runtime.block_on(run(command, root)))
        }
        None if MenubarConfig::is_bundle_program(&setup.program) => app(runtime, &setup),
        None => {
            let _ = Cli::command().print_help();
            ExitCode::from(2)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let style =
        TerminalStyleConfig::detect(cli.no_color, std::io::stdout().is_terminal(), |name| {
            std::env::var(name).ok()
        });
    match prepare() {
        Ok((setup, runtime)) => dispatch(cli.command, setup, &runtime, style),
        Err(reason) => {
            eprintln!("{}", style.fail(&reason));
            ExitCode::from(1)
        }
    }
}
