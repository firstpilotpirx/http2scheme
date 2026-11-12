## Context

The repository holds only the kit setup. The first design (`assets/original-spec.md`) was a Python helper for JetBrains links with `ide`, `file` and `line` fields; this change keeps its safety model (loopback only, secret key, allow list, no shell, request log, LaunchAgent) and makes the link general: the request carries the whole scheme link. The owner, 2026-11-12, in the chat (translated): "This tool is not for the IDE in particular; it is general, for the programs that cannot open a slack:// link but can open an http link." On the browser tab, the same day: "If you can close this tab by itself, so the tabs do not pile up, then close it." This overrules original-spec.md lines 112–114 (full existing file path, numeric line, encoded path) and line 149 ("Other editors" out of scope): D4 and the allow list take their place. The project rules ask for the hexagonal layout, the full test pyramid and the enforcement gate even for a small tool (`~/.claude/rules-custom/architecture*.md`, `testing*.md`, `enforcement-spec.md`).

## Goals / Non-Goals

**Goals:** one small static binary; every safety rule of the original spec kept; business logic only in `core/`; every rule of the gate checked by a tool.

**Non-Goals:** per-scheme field templates (the agent writes the whole scheme link); other platforms; a long-running config reload (settings are read at start).

## Decisions

### D1: the request carries the whole scheme link

`GET /open?key=<key>&url=<percent-encoded scheme link>`. The server checks the key, parses the link (D4) and checks its scheme against the allow list; the link reaches `open` byte for byte. Rejected: the original `ide` + `file` + `line` fields — they fit only JetBrains; a template per scheme in the settings — more checks, but every new app needs a template first; filed as possible later work, not needed for the PoC.

### D2: repository layout

```
app/
  app/                      Cargo workspace: the product
    Cargo.toml              [workspace] members, [workspace.package] version, [workspace.lints]
    apps/http2scheme/       bin crate: composition root (clap entry, env, wiring)
    components/linkserver/  lib crate: the server component (core + adapters + LinkserverConfig)
    components/menubar/     lib crate: the menu bar component (core + AppKit adapters + MenubarConfig, D14)
  e2e/                      Cargo workspace: cucumber E2E on the built binary
scripts/qa.sh               the gate; scripts/arch/*.sh the layout and test-layout rules
justfile                    every check recipe; `just check` = qa.sh
```

Two components. `linkserver`: settings, key, link opening, link sharing and the agent share one data set (the settings) and one lifecycle; splitting them would add gateways with nothing gained (`architecture.md` § Decision Rules). `menubar` (D14) holds the menu bar app; the app wires the two through hooks, and neither depends on the other (R-027). Rejected: one flat crate at the repo root, as the first AGENTS.md said (line 5, written at kit setup) — breaks R-011 and the `{service}/{service}` + `{service}/e2e` layout; the owner's order (proposal, Owner decisions) replaces it, and task 8.5 rewrites AGENTS.md.

### D3: the component's types

`core/domain/` (flat): `SchemeLinkValueObject`, `SchemeNameValueObject`, `AccessKeyValueObject`, `PortValueObject`, `ServerSettingsValueObject`, `PreparedLinkValueObject`, `AgentDefinitionValueObject`, `AgentStatusValueObject`, `SchemeAllowlistPolicy`, `OpenOutcomeEnum`, `LinkOpenAttemptedDomainEvent`, `LinkRejectedError`, `SettingsInvalidError`.

`core/application/` (subfolders): `use_cases/` — one input trait and one struct per use case: `OpenLink`/`OpenLinkUseCase`, `PrepareLink`/`PrepareLinkUseCase`, `PrepareSettings`/`PrepareSettingsUseCase`, `InstallAgent`/`InstallAgentUseCase`, `UninstallAgent`/`UninstallAgentUseCase`, `StartAgent`/`StartAgentUseCase`, `StopAgent`/`StopAgentUseCase`, `CheckAgentStatus`/`CheckAgentStatusUseCase`; `repositories/` — `SettingsRepository`, `OpenAttemptRepository`, `AgentDefinitionRepository`; `services/` — `SchemeLinkOpenerService`, `BrowserTabCloserService`, `SecretGeneratorService`, `AgentSupervisorService`, `ServerHealthService`; `OpenLinkCommand` sits in `use_cases/` next to `OpenLink` (R-015 allows only `use_cases/`, `repositories/`, `services/`, `exceptions/`); the trait and the struct of a use case sit in separate files (R-054); `exceptions/` — `OpenLinkError`, `PrepareLinkError`, `SettingsError`, `AgentControlError`.

Adapters (`<feature>/<tech>/`):

| Side | Folder | Types |
| --- | --- | --- |
| primary | `linkopening/http/` | `OpenLinkHandler` (axum: `/open`, `/health`, fallback 404, 405), `OpenLinkRequest`, `OpenLinkResponse` (the pages), `AxumConfig` |
| primary | `linksharing/cli/` | `PrepareLinkCommand` |
| primary | `serving/cli/` | `ServeCommand` (prepares settings, hands them to the app's serve loop) |
| primary | `agentcontrol/cli/` | `InstallAgentCommand`, `UninstallAgentCommand`, `StartAgentCommand`, `StopAgentCommand`, `AgentStatusCommand`, `TerminalStyleConfig` |
| secondary | `settings/filesystem/` | `FilesystemSettingsRepository`, `SettingsRecord` (serde, read with `config-rs`; the default file is written from a fixed text), `SettingsRecordToDomainMapper` |
| secondary | `requestlog/filesystem/` | `FilesystemOpenAttemptRepository` |
| secondary | `linkopening/launchservices/` | `LaunchServicesSchemeLinkOpenerGateway` (runs `/usr/bin/open <link>`) |
| secondary | `browsertabs/applescript/` | `AppleScriptBrowserTabCloserGateway` (runs `/usr/bin/osascript` with the D13 script) |
| secondary | `secret/osrandom/` | `OsRandomSecretGeneratorGateway` (the OS random source, read through `getrandom`) |
| secondary | `agentcontrol/filesystem/` | `FilesystemAgentDefinitionRepository`, `AgentDefinitionRecord` (plist) |
| secondary | `agentcontrol/launchd/` | `LaunchdAgentSupervisorGateway` (runs `launchctl bootstrap/bootout/print gui/<uid>`) |
| secondary | `serverhealth/http/` | `HttpServerHealthClient` (ureq, 1 s timeout) |

`LinkserverConfig` (crate root) builds every use case from paths and program names it is given; the app reads `HOME` and the test hooks (D8) and passes them in. Every type ends with a catalog role word (`naming-conventions.md`): the outbound adapters that drive a local macOS system are `Gateway`s (the catalog's Gateway Adapter; `Client` is kept for the network call of `HttpServerHealthClient`, R-046); `TerminalStyleConfig` holds the color decision. New technology words: `launchservices`, `launchd`, `applescript`, `osrandom` — the systems on the other side of the port (`architecture.md` § Technology Vocabulary allows extending it).

### D4: what a well-formed scheme link is

`SchemeLinkValueObject::parse`: 1–4096 characters; no ASCII control character and no space; a scheme by RFC 3986 (`ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`) before the first `:`; the scheme compared lower-case. `SchemeAllowlistPolicy` never permits `http`, `https`, `file`, whatever the settings say (a web or file link needs no helper and `file:` could start a program). Rejected: a full URL parser — it normalizes, so the app would not get the exact text.

### D5: the key

256 bits from `getrandom` as 64 lower-case hex characters in `key` (mode 0600, created with `O_EXCL` so an existing file is never overwritten). `AccessKeyValueObject::matches` compares with `subtle::ConstantTimeEq` over equal-length byte arrays; a candidate of another length is compared against a dummy of the right length so the time does not depend on it. Its `Debug` prints `AccessKey(****)`. The request log gets the event, never the command, so the key cannot reach it.

### D6: settings file

`~/Library/Application Support/http2scheme/settings.toml`: `port = 7777`, `allowed_schemes = ["idea", "goland"]`, `close_tab = true`; read with `config-rs` (`tech-matrix-rust.md`) into a `#[serde(deny_unknown_fields)]` record; every error names the file and the field (`SettingsInvalidError`). Read once at start by `serve` and `link`.

### D7: pages and status codes

Small inline HTML, no external resource, light and dark through `prefers-color-scheme`. 200 "Opened" (also tries `window.close()`, the fallback for Firefox, which D13 cannot reach), 403 "Wrong key", 400 "Malformed link" or "Scheme not allowed: <scheme>" (HTML-escaped), 404 "Not found", 405 "Method not allowed", 502 "Could not open the app". `/health` answers 200 `ok` as plain text. A failed write of the request log keeps the answer: the link opened, so the page still says so (`a_failed_record_keeps_the_outcome`). `Cache-Control: no-store` and `Referrer-Policy: no-referrer` on every answer, so the key does not leak through a cached page or a referrer.

### D8: tests replace the programs, not the code

The app reads three test hooks from the environment and passes them to `LinkserverConfig`: `HTTP2SCHEME_OPEN_PROGRAM` (default `/usr/bin/open`), `HTTP2SCHEME_LAUNCHCTL_PROGRAM` (default `/bin/launchctl`) and `HTTP2SCHEME_OSASCRIPT_PROGRAM` (default `/usr/bin/osascript`); these literals, the `~/Library/...` paths and the host names `127.0.0.1` and `localhost` (the Host check of D7 and the tab prefixes of D13) live only in the app crate, which passes them to `LinkserverConfig` (R-083); the agent definition folder comes from `HOME`. Adapter integration tests and E2E point them at small recording scripts in a temp dir, so no test starts a real app or touches the user's LaunchAgents. One `#[ignore]` integration test per program runs the real one, harmlessly: `open` on an unregistered scheme (an error), `launchctl` with a throw-away agent label that runs `/usr/bin/true`, the D13 script compiled by `/usr/bin/osacompile` (running it with the real `osascript` while a browser runs makes macOS ask for the Automation permission and blocks the test for 120 s); the gate runs them in its `--ignored` stage. The owner approved this on 2026-11-12 (proposal, Owner decisions). Rejected: a mock inside the binary — the E2E would no longer test the shipped code path.

### D9: async boundary

Port traits use `#[async_trait]` and are object-safe (`architecture-rust.md`); the adapters run child programs through `tokio::process::Command` (no shell), the blocking health request in `tokio::task::spawn_blocking`, and their small file reads and writes with `std::fs`.

### D10: terminal output

`owo-colors` with its `supports-colors` feature behind `TerminalStyleConfig`: no color when the stream is not a TTY, `NO_COLOR` is set and not empty, `--no-color` is passed or `TERM=dumb`; `FORCE_COLOR=1` forces it, also with `TERM=dumb`; `NO_COLOR` and `--no-color` win over it. Status lines use `✓ ✗ ●` (`[ok] [fail] [..]` without color). Diagnostic logging: `log` + `env_logger` to stderr (`RUST_LOG`), which launchd writes to `~/Library/Logs/http2scheme/agent.err.log`. `clap` and `env_logger` are built without their `color` features, so `owo-colors` is the one color library (R-112).

### D11: the gate

`just check` runs `scripts/qa.sh`: `fmt`, `lint` (clippy, the lint table of `tech-matrix-rust.md`, `-D warnings`), `cargo check`, `complexity`, `machete`, `unsafe-count`, `modules`, `typos`, `recipes` (shellcheck), `dup`, `deny`, `arch` (scripts/arch: layout R-011–R-018, layer imports R-021–R-024 and R-027, purity R-032–R-035, naming R-040–R-054, sibling tests R-071/R-072, adapter ITs R-061/R-062, slice per primary adapter R-073, one feature per capability R-064/R-065), unit, integration (with the `--ignored` real-program tests of D8), architecture tests, E2E, release build, `coverage` — the R-091 order. `qa.sh` takes `--fix`, `--clean`, `--stage`, `--skip`, `--json`, `--no-color`, `--help`. Every analyzer is pinned and guarded by a `_require-<tool>` recipe that prints the install command (R-139). Every gate check owns `tests/gate_fixtures/<check>/{violation,clean}/` with an `expect.txt`; `scripts/check-gate-fixtures.sh` fails when a check passes its violation tree, misses the expected message, or fails its clean tree; `exempt.tsv` lists checks with no tree and why (R-129). Stage lines use `[ OK | FAIL | WARN ]` (R-092). `semver` is skipped: the component is not a published library.

### D13: closing the browser tab

After a successful open, `OpenLinkUseCase` calls `BrowserTabCloserService::close_open_tabs(port)`, unless `close_tab` is false; the handler calls only the use case (R-036). `AppleScriptBrowserTabCloserGateway` returns at once and, in a background task, waits 300 ms (the page must load first), then runs one `osascript` script: for each of Safari, Google Chrome, Arc, Brave Browser and Microsoft Edge that `is running`, close every tab whose URL starts with `http://127.0.0.1:<port>/open?` or `http://localhost:<port>/open?`. Only browsers already running are touched, so none is started. macOS asks once per browser for the Automation permission; until the user allows it, or in Firefox, the tab stays with the "Opened" page. A failure is written to the diagnostic log only. Refused requests keep their tab, so the reason stays readable. Rejected: closing only the newest tab — a fast second click would close the wrong one; a browser extension — one per browser to install.

### D12: version and release

`version` in `[workspace.package]` of `app/app/Cargo.toml`; `just release` on a clean `origin/main` with `HTTP2SCHEME_SIGNING_IDENTITY` named and in the keychain: the gate, `just dmg` (D17), tag, `gh release create` with `http2scheme-vX.Y.Z.dmg` and its `.sha256`, as VoicePuncher releases. The owner, 2026-11-12, in the chat (translated): "build it like VoicePuncher, the way I already have it — not a separate change, in this current task". This replaces the first plan's tarball of the bare binary and the line of `AGENTS.md` § Release that names it (`http2scheme-vX.Y.Z-aarch64-apple-darwin.tar.gz`); task 11.3 rewrites that line.

### D14: the menu bar app is a second component

The menu bar app is the component `components/menubar` beside `linkserver`; the app crate wires both. ADR-001 is the project's first file under `docs/`; `scripts/arch/adr.sh` checks that every `docs/decisions/ADR-*.html` has the sections Status, Context, Decision, Alternatives and Consequences (R-006), with its gate fixtures (R-129). `menubar` knows nothing of `linkserver`: the app hands it the server's state through `ServerStateService`, whose adapter `ServeOutputServerStateGateway` (reached by the app only through `MenubarConfig::server_output`, R-030) reads the lines the `serve` command writes (`listening on <address>` → `Running`, an error line → `Stopped: <reason>`), so the state and its words are exactly the CLI's. Without a command, `main` checks whether its program is `<x>.app/Contents/MacOS/http2scheme`: then it starts the server on a tokio worker and runs the AppKit loop on the main thread; otherwise it prints the help as before. A terminate signal ends the process; so do Quit and a quit from the system (`NSApplication terminate`). Rejected: the app installing the LaunchAgent of `background-service` for itself — two copies of the program (the one in the bundle and the agent) and a second place that decides the state; SMAppService registers the app itself, as VoicePuncher does. The LaunchAgent commands stay for a CLI-only setup.

`menubar` types: `core/domain` — `ServerStateEnum` (`Starting`, `Running`, `Stopped`), `LoginItemStateEnum` (`On`, `Off`, `NeedsApproval`, `Unavailable`), `MenuViewValueObject` (the menu's words and check mark), `AppFolderEnum` (`Settings`, `Logs`), `MenuCommandEnum` (`Show`, `ToggleLoginItem`, `OpenFolder`); `core/application` — `use_cases/` `OperateMenu`/`OperateMenuUseCase` (one `execute(MenuCommandEnum)` for Show, Toggle Login Item and Open Folder, answering the fresh `MenuViewValueObject`, so the menu adapter reaches the core through one use case, R-036) and `OfferLoginItem`/`OfferLoginItemUseCase` (the first launch from `/Applications`, run by `MenubarConfig` at launch); `services/` `LoginItemService`, `ServerStateService`, `FolderOpenerService`; `repositories/` `FirstLaunchRepository` (`exists`, `record`); `exceptions/` `MenuActionError`. Adapters: primary `statusmenu/appkit/` `StatusMenuHandler` (status item, menu, item target) and `AppKitConfig` (the shared application with the accessory policy, its run loop and the system quit); secondary `loginitem/smappservice/` `SmAppServiceLoginItemGateway`, `folders/launchservices/` `LaunchServicesFolderOpenerGateway` (`/usr/bin/open <folder>`, no shell, the same system the link opener of `linkserver` uses), `firstlaunch/filesystem/` `FilesystemFirstLaunchRepository` (the marker file `login-item-offered` in the settings folder), `serverstate/serveoutput/` `ServeOutputServerStateGateway`. New technology words: `appkit` (the AppKit framework), `smappservice` (ServiceManagement's `SMAppService`), `serveoutput` (the text stream of the in-process `serve` command, the only system on the other side of that port). The ports are `#[async_trait] … : Send + Sync`, as D9 and `architecture-rust.md` ask; their adapters do their short work (a file, one `open`, one ServiceManagement call) without awaiting anything, so `StatusMenuHandler`, which runs on the main thread outside any tokio runtime, drives the use case with a no-op-waker `block_on` of its own: the future is ready at its first poll.

### D15: unsafe code only in the AppKit adapters

AppKit and ServiceManagement are reached through `objc2` 0.6.4, `objc2-foundation`, `objc2-app-kit` and `objc2-service-management` 0.3.2, the versions VoicePuncher runs; their calls are `unsafe`. ADR-001 (`docs/decisions/ADR-001-appkit-ffi.html`) allows `unsafe` in exactly four files, `status_menu_handler.rs`, `settings_window_handler.rs` and `history_tab_handler.rs` (D19) and `sm_app_service_login_item_gateway.rs`, each under `#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]` with a `// SAFETY:` comment above every block and at most three such blocks a file (R-080): the menu item and its target are made in one block, and every ServiceManagement call goes through one block over a private operation enum. Every crate keeps `lints.workspace = true` (R-140); the workspace table sets `unsafe_code = "deny"` in place of `forbid`, and `linkserver` and the app keep `#![forbid(unsafe_code)]` at their crate roots, which `cargo geiger --forbid-only` checks. `just unsafe-count` keeps checking that `http2scheme` and `linkserver` forbid unsafe code and counts the `unsafe` uses per file of `menubar` against `metrics/unsafe.tsv` (R-144): a higher count fails, a lower one must be written back in the same commit. Rejected: `tray-icon` + `muda` — more dependencies and their own event loop for a four-item menu, and the login item would still need ServiceManagement; a Swift helper — a second language and toolchain.

### D16: the login item

`SmAppServiceLoginItemGateway` asks `SMAppService.mainAppService` for the status, registers and unregisters, and opens the Login Items pane (`SMAppService.openSystemSettingsLoginItems`); outside a bundle it reports `Unavailable` and never asks the system. `OfferLoginItemUseCase` runs once at launch: when the program is under `/Applications/`, no marker exists and the state is `Off`, it registers and records the marker; any other case records nothing but the marker when it was under `/Applications/`. So a user who removed the login item keeps it removed, and builds run from `dist/` never touch the login items. No restart after a crash: a login item, unlike `KeepAlive`, is started once per login; the server runs in the app's process and has no crash path of its own besides a panic, which the `serve` tests cover.

### D17: the bundle and the disk image

As `tech-matrix-rust.md` § macOS Desktop says and as VoicePuncher does: `app/packaging/Info.plist` template (`CFBundleIdentifier` `io.github.firstpilotpirx.http2scheme`, `CFBundleExecutable` `http2scheme`, `LSUIElement` true, `LSMinimumSystemVersion` 15.0, `NSAppleEventsUsageDescription` — the tab closer's `osascript` runs as the app's child, so macOS asks in the app's name); `just bundle` builds the release binary, writes `dist/http2scheme.app` (the binary, the icon from `app/packaging/icons/http2scheme.iconset` through `iconutil`, the plist with the version through `plutil -replace`) and signs it (`_sign`: `codesign --force --sign <identity> --timestamp=none`, no hardened runtime, then `codesign --verify --deep --strict`); `just dmg` builds `dist/http2scheme-vX.Y.Z.dmg` through `create-dmg` 1.3.0 (background `app/packaging/dmg-background.png`, the app icon at the left, the Applications link at the right) and its `.sha256`; `just signing-identity` creates the self-signed certificate `http2scheme` once per machine. The pictures are drawn by `app/packaging/draw-artwork.swift` (`swift` of the Xcode command line tools) and committed. The menu bar icon is the SF Symbol `link` as a template picture, so it follows the menu bar's light and dark look. The pictures follow `tech-matrix-rust.md`: the script draws one 1024-pixel picture, `sips` scales it into the ten sizes of the iconset (16, 32, 128, 256 and 512 points, each at 1× and 2×). The gate's `bundle` stage, after `build` and before `coverage` (R-091 "build+packaging"), builds the disk image (ad-hoc signed when no certificate is named) and runs the `@bundle` E2E scenarios on it. README § Install follows VoicePuncher's: download, check the `.sha256`, drag to Applications, Open Anyway once per version.

### D18: Serve Links and the Link History window

The owner asked for both on 2026-11-12, after the menu was built, "the same technologies as VoicePuncher": VoicePuncher's dictation history window (an `NSWindow` built with `objc2-app-kit`, rows newest first, a button per row) is the model.

Serve Links. `ServerStateEnum` gains `Off` (status line `Off`); `MenuCommandEnum` gains `ToggleServer`; `MenuViewValueObject` gains `serve_checked` (true unless `Off`). `OperateMenuUseCase` on `ToggleServer` calls the new port `ServerSwitchService` (`turn_off` when the state is not `Off`, else `turn_on`), which extends `ServerStateService`, so the use case takes one server port and stays at five arguments (clippy `too_many_arguments`). Its adapter is `ServeOutputServerStateGateway` itself, which already owns the state: it now also holds the app's server launcher, `ServeLauncher` (`Arc<dyn Fn(ServeShutdown, ServeOutput, ServeOutput) + Send + Sync>`, where `ServeShutdown` is a boxed `Future<Output = ()> + Send` and each `ServeOutput` a boxed `Write + Send`: the gateway's own `out()` and `err()` writers, so the launcher needs nothing of the gateway), and a `tokio::sync::oneshot` sender per run. `turn_on` sets `Starting`, makes a channel and hands its receiver to the launcher, which spawns `serve_command().run(shutdown, out, err)` on the app's runtime; `turn_off` sends on the channel and sets `Off`, and a line `serve` prints after that does not change `Off`. `MenubarConfig::start_server` turns it on at launch. The app passes the launcher in `MenubarHooks` (`serve`, `replay`) to `MenubarConfig::macos`; `menubar` still knows nothing of `linkserver` (R-027).

Link History. New domain `LinkRecordValueObject` (time, result, link) in `menubar`; port `LinkHistoryRepository` (`find_latest(limit)`), adapter `linkhistory/filesystem/FilesystemLinkHistoryRepository` that reads `requests.log` in the logs folder — the line format of `link-opening` (`<RFC 3339 UTC>\t<result>\t<escaped link>`) is the contract; it keeps the last 100 well-formed lines, unescapes the link, and a missing file is an empty list; a log that cannot be read is a `MenuActionError::History`, which the window shows as `The link history could not be read: <reason>` with no row (`an_unreadable_log_says_why`). Port `LinkReplayService` (`replay(link) -> result`), adapter `linkreplay/inprocess/InProcessLinkReplayGateway` over the `replay` hook, `LinkReplayer` (`Arc<dyn Fn(&str) -> String + Send + Sync>`), which the app runs to its end on its own runtime (`Handle::block_on`), so the menu's no-op-waker `block_on` (D14) still finds the future ready at its first poll. Use case `BrowseHistory`/`BrowseHistoryUseCase`: `execute(HistoryCommandEnum)` with `Show` and `Replay(link)`, answering the fresh `HistoryViewValueObject` (rows newest first, or the note `No link has been opened yet.`). In `linkserver` the replay is the use case `ReplayLink`/`ReplayLinkUseCase` (settings through `PrepareSettingsUseCase`, the allow list check, the opener, the request log; no key, no tab closer) behind the primary adapter `linkreplay/inprocess/ReplayLinkHandler` (`replay(&str) -> String`, the result word), which `LinkserverConfig::replay_handler` builds and the app wraps as the `replay` hook. Primary adapter `statusmenu/appkit/HistoryWindowHandler` (beside the status menu: both share the `AppKit` application and its no-op-waker `ready`): one `NSWindow` titled `Link History`, 720 × 480, built on first use and brought to the front after, with an `NSScrollView` over rows laid out by frame (no table data source, so no delegate `unsafe`): Time (local time through `NSTimeZone`, `2026-11-12 19:42:05`), Result, Link (truncated in the middle, the whole link as tool tip), ▶. The button is made with its target in one `unsafe` block (R-080, ADR-001). The status menu gets `Link History…`, which calls a closure that shows the window, so the two primary adapters stay apart. No live refresh: the window reads the log again when it is shown and after a ▶ (VoicePuncher's once-a-second check is left out, a non-goal for a list the user reopens).

### D19: one Settings window in place of the folder items

The owner, 2026-11-12, after using the first build (translated): "the settings must be in the GUI, not an open-settings-folder item, and the log too, as a table with a run-again button on each row"; "allowed schemes and the port are not needed now". The approved mock is board 3 of the interface mock (below). This replaces the folder items of D14 and the separate Link History window of D18.

Menu: title line, status line, separator, Serve Links, Start at Login, separator, Settings…, Link History…, separator, Quit. Removed with Open Settings Folder and Open Logs Folder: `AppFolderEnum`, `AppFoldersValueObject`, `MenuCommandEnum::OpenFolder`, `FolderOpenerService`, `LaunchServicesFolderOpenerGateway` and their tests; `OperateMenuUseCase` takes the version, the server switch and the login item. `MenuViewValueObject` gains `login_note()`: `The app starts when you log in to the Mac.` while the login item is on or off, `Approve the app in System Settings › Login Items.` while macOS waits, `Only the app installed in Applications can start at login.` outside a bundle.

Window. Primary adapter `statusmenu/appkit/SettingsWindowHandler` (`settings_window_handler.rs`): one `NSWindow` titled `http2scheme Settings`, 720 × 480, built on first use and brought to the front after, its content an `NSTabView` with the items General and History. General: two rows laid out by frame, each a bold name (`Serve links`, `Start at login`), an `NSSwitch` at the right and a secondary-color line under the name (the status line; the login note). The switches call `toggleServer:` and `toggleLogin:` of the window's target, which runs the same `OperateMenu` use case as the menu (one `Arc`, so both read one state) with `ToggleServer` and `ToggleLoginItem`, then shows the fresh view; after turning the server on it reads the view again every 50 ms, at most 1 s, until the line leaves `Starting…`, so the line shows `Running on …` without a timer. History: `HistoryTabHandler` (`history_tab_handler.rs`, the former `HistoryWindowHandler` of D18) draws the same rows and ▶ buttons into its own `NSView`, which the History item holds; it owns no window. `show_general()` and `show_history()` select the item, read again, activate the app and bring the window to the front; the status menu's Settings… and Link History… call two closures that do so, so the menu and the window stay apart. The two switches are wired in one `unsafe` block (`setTarget` and `setAction`), so the window file has three (init, window, wiring), the tab file two (init, button); ADR-001 and `metrics/unsafe.tsv` name both files in place of `history_window_handler.rs`. New `objc2-app-kit` features: `NSSwitch`, `NSTabView`, `NSTabViewItem`, `NSColor`. Rejected: `NSTabViewController` with view controllers, as VoicePuncher's form — a controller per tab for two fixed tabs; check boxes — the owner asked for switches.

### D20: the left app comes back before a link opens

The owner, 2026-11-12 (translated): "it remembers the open was in the browser, Alt-Tab returns to the browser, not to the code". A click in the Claude app shows the browser, then the link's app: Command-Tab then lists the link's app, the browser, the Claude app. Before `OpenLinkUseCase` opens an admitted link, it calls the new port `AppOrderService::bring_back_left_app()`; its failure is ignored, as a failed tab close is. Activating the left app first and then opening the link makes the order: the link's app, the left app, the browser. Adapter `apporder/lsappinfo/LsAppInfoAppOrderGateway` (program `lsappinfo`, the macOS Launch Services tool, and the `open` program): `lsappinfo visibleProcessList` prints the apps front to back (`ASN:0x0-0x9ba9ba-"Claude": ASN:0x0-0xb40b4-"Safari": …`, a space in a name printed as `_`); the pure mapper `process_list_to_left_app_mapper` answers the second entry's ASN only when the first is one of the browsers of D13; `lsappinfo info -only bundleid <ASN>` gives `bundleID="<id>"`; `open -b <id>` activates that running app and the gateway waits for it, so the link's app comes after. Every failure (a program error, no browser in front, no app behind it, no bundle id) is `OpenLinkError::ReorderFailed`; the trait method writes it to the diagnostic log and answers `Ok`, as the tab closer does, and `bring_back_now` returns it for the tests. `LinkserverSetup` gains `lsappinfo_program` (`/usr/bin/lsappinfo` from `main.rs`); the tests replace it with a recording program that prints a fixed list (D8). The replay of a History row brings no app back: no browser is in front there. Rejected: tracking the front app through `NSWorkspace` notifications — it needs the AppKit loop, which the command line server has not; `osascript` `activate` — an Automation prompt for every app; hiding the browser — it stays second in Command-Tab.

## Test levels

Slice: `app/app/components/<component>/tests/features/slice/<capability>.feature` with fake secondary adapters; the `menubar` slice drives the real `StatusMenuHandler` on the main thread (a `harness = false` test), reads the menu's items and chooses them, with fakes behind every secondary port. Component: `…/features/component/link_sharing.feature`. Integration: the adapter's `*_integration_test.rs`. E2E: `app/e2e/tests/features/<capability>.feature` on the built binary with recording `open`, `launchctl` and `osascript` scripts (D8).

| Scenario | Level |
| --- | --- |
| background-service: Install starts the server and keeps it at login | slice + E2E |
| background-service: A second install replaces the first | slice + E2E |
| background-service: Uninstall stops and removes the agent | slice + E2E |
| background-service: Uninstall with nothing installed is harmless | slice + E2E |
| background-service: Stop keeps the definition | slice + E2E |
| background-service: Start loads a stopped agent | slice + E2E |
| background-service: Start without install gives a hint | slice + E2E |
| background-service: A healthy setup | slice + E2E |
| background-service: A stopped server | slice + E2E |
| background-service: Serve on a free port | slice + E2E |
| background-service: Serve on a taken port | slice + E2E |
| link-opening: A keyed link with an allowed scheme opens the app | slice + E2E |
| link-opening: Special characters in the link reach the app unchanged | slice + E2E |
| link-opening: The server cannot be reached from another computer | integration + E2E |
| link-opening: The tab of an opened link closes | slice + E2E |
| link-opening: A refused request keeps its tab | slice + E2E |
| link-opening: The left app comes back behind the opened app | unit (use case order) + slice + integration (gateway) |
| link-opening: A refused request changes no app order | slice |
| link-opening: A failed reorder keeps the link opened | slice |
| link-opening: Tab closing turned off in the settings | slice + E2E |
| link-opening: A failed tab close keeps the link opened | slice + E2E |
| link-opening: A request with a wrong key is refused | slice + E2E |
| link-opening: A request with no key is refused | slice + E2E |
| link-opening: A scheme not on the list is refused | slice + E2E |
| link-opening: A malformed scheme link is refused | slice + E2E |
| link-opening: Web and file links are never opened | slice + E2E |
| link-opening: A request under a foreign host name is refused | slice + E2E |
| link-opening: An unknown address is not found | slice + E2E |
| link-opening: The health address answers without a key | slice + E2E |
| link-opening: A failed open is reported | slice + E2E |
| link-opening: An opened link is logged | slice + E2E |
| link-opening: A refused request is logged without the key | slice + E2E |
| link-sharing: A scheme link becomes an open link | slice + E2E |
| link-sharing: A path with spaces survives the round trip | slice + component + E2E |
| link-sharing: The first link command creates the key | slice + E2E |
| link-sharing: A scheme not on the list prints the reason | slice + E2E |
| server-settings: A fresh home gets default settings and a key | slice + E2E |
| server-settings: Two first uses create two different keys | slice + integration + E2E |
| server-settings: Existing settings and key are kept | slice + E2E |
| server-settings: An allowed scheme added by hand is honored | slice + E2E |
| server-settings: A broken settings file stops the start with its reason | slice + E2E |
| server-settings: A key file readable by others stops the start | slice + E2E |
| menu-bar-app: The app serves a keyed link | E2E (`@bundle`) |
| menu-bar-app: The bundled program is also the command line tool | E2E (`@bundle`) |
| menu-bar-app: A running server | slice (menubar) |
| menu-bar-app: A taken port | slice (menubar) |
| menu-bar-app: Turning the server off | slice (menubar) + integration (gateway) |
| menu-bar-app: Turning the server on again | slice (menubar) + integration (gateway) |
| menu-bar-app: Requests are listed newest first | slice (menubar) + integration (repository) |
| menu-bar-app: Only the latest 100 requests are listed | slice (menubar) + integration (repository) |
| menu-bar-app: An unreadable log line has no row | slice (menubar) + integration (repository) |
| menu-bar-app: No request yet | slice (menubar) + integration (repository) |
| menu-bar-app: Opening an IntelliJ link again | slice (menubar) + slice (linkserver) + integration (linkserver handler) |
| menu-bar-app: A link whose scheme is not allowed stays closed | slice (menubar) + slice (linkserver) + integration (linkserver handler) |
| menu-bar-app: Turning Start at Login on | slice (menubar) |
| menu-bar-app: Turning Start at Login off | slice (menubar) |
| menu-bar-app: A login item that waits for approval | slice (menubar) |
| menu-bar-app: Not an installed app | slice (menubar) |
| menu-bar-app: The first launch from Applications | slice (menubar) |
| menu-bar-app: A login item the user removed stays removed | slice (menubar) |
| menu-bar-app: A launch from another folder | slice (menubar) |
| menu-bar-app: Turning the server off in the Settings window | slice (menubar) + integration (window handler) |
| menu-bar-app: Turning Start at login on in the Settings window | slice (menubar) + integration (window handler) |
| menu-bar-app: Settings opens the General tab | slice (menubar) + integration (window handler) |
| menu-bar-app: Link History opens the History tab | slice (menubar) + integration (window handler) |
| menu-bar-app: A terminate signal ends the app | E2E (`@bundle`) |
| app-distribution: The bundle's information | E2E (`@bundle`) |
| app-distribution: A built bundle passes the signature check | E2E (`@bundle`) |
| app-distribution: A release without a certificate stops | E2E (`@bundle`) |
| app-distribution: The disk image holds the app and the Applications link | E2E (`@bundle`) |
| app-distribution: The checksum file matches the disk image | E2E (`@bundle`) |
| key comparison timing (D5) | not tested: a timing test is too noisy to pass or fail reliably; `AccessKey::matches` uses `subtle::ConstantTimeEq`, and its unit test checks equal keys and keys that differ in the first and in the last byte |
| manual run | the user installs the app from the disk image (Open Anyway once), clicks printed IntelliJ and GoLand links in the Claude chat and the tab closes; the menu shows `Running on …` with Start at Login checked; after a Mac restart, a link still works |

## Interface mock

[http2scheme interface mock](https://claude.ai/artifact/DDotUxHvNFqK8Cbujs7GRh) — the browser pages and the command output, each in light and dark.

Board 3 of the mock, "Menu and Settings window — light and dark", is the approved menu and Settings window (D19); the owner, 2026-11-12: "now I like everything, approved".

```
[link icon]
  http2scheme 0.1.0                       (disabled)
  Running on http://127.0.0.1:7777        (disabled; or "Starting…", "Stopped: port 7777 is taken", "Off")
  ─────────────
  ✓ Serve Links                           (no check mark while "Off")
  ✓ Start at Login                        (or "Start at Login (approve in System Settings)")
  ─────────────
  Settings…                               (the Settings window, General tab)
  Link History…                           (the Settings window, History tab)
  ─────────────
  Quit http2scheme                  ⌘Q

┌ http2scheme Settings ─────────────────────────────────────────────────────┐
│                         [ General ] [ History ]                           │
│  Serve links                                                    (●  ) on  │
│  Running on http://127.0.0.1:7777                                         │
│  Start at login                                                 (●  ) on  │
│  The app starts when you log in to the Mac.                               │
└───────────────────────────────────────────────────────────────────────────┘
  History tab: Time | Result | Link | [▶], newest first, as D18 lays out the rows;
  an empty log: one line "No link has been opened yet."
```

The disk image window: 600 × 400, the app icon at (150, 200), the Applications link at (450, 200), an arrow between them on the background. The LaunchAgent label is `io.github.firstpilotpirx.http2scheme`.

## Risks / Trade-offs

- [Gatekeeper blocks the first launch] → the app is self-signed, not notarized; README says to click Open Anyway once per version, as for VoicePuncher.
- [The app and a CLI LaunchAgent both run] → the second one finds the port taken; the menu shows `Stopped: port 7777 is taken` and README says to run `http2scheme uninstall` after installing the app.
- [The menu slice needs a logged-in desktop] → the gate runs on the owner's Mac, where the window server is there; without it the slice harness fails at its first step, naming the reason.

- [The browser leaves a tab after each click] → D13 closes it in the five supported browsers; Firefox keeps it (the page tries `window.close()`).
- [The Automation permission prompt] → shown once per browser on the first click; `README.md` says to allow it.
- [The key is in browser history] → only local users can read it; history holds the same key the key file holds. Rotating the key = deleting the key file.
- [A program that registers an allowed scheme gets any text after `scheme:`] → only the user's own allow list counts, and `http`, `https`, `file` are never allowed.
- [DNS rebinding: a website resolving its name to 127.0.0.1] → it still needs the key; the server also answers 421 when the `Host` header is not `127.0.0.1:<port>` or `localhost:<port>`.
- [The LaunchAgent points at the binary's path at install time] → `install` prints that path; moving the binary needs a new `install`.
