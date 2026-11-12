## 1. Workspaces and tooling

- [x] 1.1 Create `app/app/` (workspace: `apps/http2scheme` bin, `components/linkserver` lib with empty `core/{domain,application}` and `adapters/{primary,secondary}` modules, `[workspace.package] version = "0.0.0"`, the `[workspace.lints]` table of `tech-matrix-rust.md`, `clippy.toml` with the size thresholds of R-130/R-132/R-133/R-134 and `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests = true`), `app/e2e/` (workspace with the same `[workspace.lints]` table and `clippy.toml`, every crate `lints.workspace = true`, cucumber harness `cucumber_e2e`), `clap` and `env_logger` without their `color` features, `rust-toolchain.toml` pinned to 1.99.0, `deny.toml`, `.gitignore`; every dependency pinned with `=`; `cargo build` succeeds in both workspaces
- [x] 1.2 Add `justfile` with `test-one`, `test`, `e2e`, `fmt`, `lint`, `deny` and the `_require-<tool>` guards
- [x] 1.3 Commit (`kit ship`)

## 2. Domain

- [x] 2.1 Red: unit tests for `SchemeNameValueObject` and `SchemeLinkValueObject` (D4: length 1–4096, control characters, space, scheme grammar, lower-case scheme, exact text kept); `just test-one` fails
- [x] 2.2 Green: implement both; the tests pass
- [x] 2.3 Red: unit tests for `AccessKeyValueObject` (64 lower-case hex, from 32 bytes, `matches` true only for the same key, false for wrong, empty and other-length candidates, redacted `Debug`) and `PortValueObject` (1024–65535); they fail
- [x] 2.4 Green: implement both with `subtle::ConstantTimeEq` (D5); the tests pass
- [x] 2.5 Red: unit tests for `SchemeAllowlistPolicy` (permits listed schemes, never `http`/`https`/`file`), `ServerSettingsValueObject`, `PreparedLinkValueObject`, `AgentDefinitionValueObject`, `AgentStatusValueObject` (healthy only when installed, loaded and answering), `OpenOutcomeEnum` labels, `LinkOpenAttemptedDomainEvent`, `LinkRejectedError`, `SettingsInvalidError` messages; they fail
- [x] 2.6 Green: implement them; every domain test passes
- [x] 2.7 Commit (`kit ship`)

## 3. Application

- [x] 3.1 Red: sibling `*_test.rs` for `OpenLinkCommand` (fields kept, `Debug` hides the key) and the errors `OpenLinkError`, `SettingsError`, `AgentControlError` (messages); they fail
- [x] 3.1a Green: the `#[async_trait]` port traits (`SettingsRepository`, `OpenAttemptRepository`, `AgentDefinitionRepository`, `SchemeLinkOpenerService`, `BrowserTabCloserService`, `SecretGeneratorService`, `AgentSupervisorService`, `ServerHealthService`) with `mockall::automock`, `OpenLinkCommand` in `use_cases/`, the three errors; the tests pass
- [x] 3.2 Red: unit tests for `OpenLinkUseCase` (opens on right key and allowed scheme; wrong or missing key, malformed link, scheme not allowed open nothing; opener failure maps to `OpenFailed`; every path records one event without the key; the mocked `BrowserTabCloserService` is called once with the port after an opened link, never after a refused or failed one, never when `close_tab` is off, and its failure keeps the outcome `Opened`); they fail
- [x] 3.3 Green: implement `OpenLink` + `OpenLinkUseCase` in separate files; the tests pass
- [x] 3.4 Red: unit tests for `PrepareSettingsUseCase` (creates defaults and a key from the generator when missing, keeps existing ones, passes repository errors on) and `PrepareLinkUseCase` (returns port, key and link for an allowed link; refuses malformed and not-allowed links); they fail
- [x] 3.5 Green: implement both; the tests pass
- [x] 3.6 Red: unit tests for `InstallAgentUseCase` (writes the definition, unloads an old one, loads), `UninstallAgentUseCase` (unloads when loaded, deletes, harmless when absent), `StartAgentUseCase` and `StopAgentUseCase` (refuse when not installed), `CheckAgentStatusUseCase` (three facts); they fail
- [x] 3.7 Green: implement the five use cases and their traits; every application test passes
- [x] 3.8 Commit (`kit ship`)

## 4. Secondary adapters

- [x] 4.1 Red: `filesystem_settings_repository_integration_test.rs` on a temp home (creates `settings.toml` and a 0600 `key` with `O_EXCL`, keeps existing files, rejects unknown fields, a bad port, a bad scheme, a key readable by others, naming the file and field; `close_tab` default true); sibling `*_test.rs` unit tests for every implementation file this group adds (R-071) (`SettingsRecord`, `SettingsRecordToDomainMapper`); they fail
- [x] 4.2 Green: `FilesystemSettingsRepository`, `SettingsRecord`, `SettingsRecordToDomainMapper`; the test passes
- [x] 4.3 Red: `filesystem_open_attempt_repository_integration_test.rs` (appends one line per event with an RFC 3339 time, result and link, creates the folder); `os_random_secret_generator_gateway_integration_test.rs` (32 bytes, two calls differ); sibling `*_test.rs` unit tests for every implementation file this group adds (R-071); they fail
- [x] 4.4 Green: `FilesystemOpenAttemptRepository`, `OsRandomSecretGeneratorGateway`; the tests pass
- [x] 4.5 Red: `launch_services_scheme_link_opener_gateway_integration_test.rs` with a recording script as the program (one argument, exact bytes for a link with spaces, quotes, `;`, `$(...)`; a non-zero exit is an error) plus an `#[ignore]` run of the real `/usr/bin/open` on an unregistered scheme; `apple_script_browser_tab_closer_gateway_integration_test.rs` with a recording `osascript` (the script names the five browsers, checks `is running`, matches both open-address prefixes with the port; a non-zero exit is an error) plus an `#[ignore]` run of the real `osacompile` on the script (D8); sibling `*_test.rs` unit tests for every implementation file this group adds (R-071); they fail
- [x] 4.6 Green: `LaunchServicesSchemeLinkOpenerGateway`, `AppleScriptBrowserTabCloserGateway` (`tokio::process::Command`, no shell); the tests pass
- [x] 4.7 Red: `filesystem_agent_definition_repository_integration_test.rs` (writes a plist with label, `serve` arguments, `RunAtLoad`, `KeepAlive`, log paths; reads existence; deletes), `launchd_agent_supervisor_gateway_integration_test.rs` with a recording `launchctl` script (`bootstrap gui/<uid> <plist>`, `bootout gui/<uid>/<label>`, `print` exit code as loaded) plus an `#[ignore]` run of the real `launchctl` with a throw-away label running `/usr/bin/true`, `http_server_health_client_integration_test.rs` against `wiremock` (answering, error status, timeout) and a closed port (refused); sibling `*_test.rs` unit tests for every implementation file this group adds (R-071) (`AgentDefinitionRecord`); they fail
- [x] 4.8 Green: `FilesystemAgentDefinitionRepository`, `AgentDefinitionRecord`, `LaunchdAgentSupervisorGateway`, `HttpServerHealthClient`; the tests pass
- [x] 4.9 Commit (`kit ship`)

## 5. HTTP primary adapter and link-opening

- [x] 5.1 Red: `open_link_handler_integration_test.rs` — real axum on an ephemeral loopback port, mocked `OpenLink`: status and page per outcome (200, 403, 400 both kinds, 502), 404, 405, 421 for a foreign `Host`, `/health`, `Cache-Control: no-store`, `Referrer-Policy: no-referrer`, HTML escaping of the scheme, the bind address is 127.0.0.1; sibling `*_test.rs` unit tests for every implementation file this group adds (R-071) (`OpenLinkRequest`, `OpenLinkResponse`, `AxumConfig`); they fail
- [x] 5.2 Green: `OpenLinkHandler`, `OpenLinkRequest`, `OpenLinkResponse` (pages as in the agreed mock, light and dark), `AxumConfig`; the test passes
- [x] 5.3 Red: `tests/features/slice/link_opening.feature` with one `Rule:` per requirement of `link-opening` and every scenario but the network one, steps in `tests/steps/link_opening.rs`, harness `tests/cucumber_integration.rs` (World holds fake opener, fake log, server handle; the fake tab closer is wired through the real use case); `just test-one "scenario: …"` fails
- [x] 5.4 Green: `LinkserverConfig::router` wires the real use case with the fakes; the slice scenarios pass
- [x] 5.5 Commit (`kit ship`)

## 6. CLI primary adapters

- [x] 6.1 Red: `prepare_link_command_integration_test.rs` (assert_cmd on a tiny test binary target that mounts only this command with a mocked `PrepareLink`: prints one link, percent-encodes, failure status and reason) and the same for `serve_command`, the five agent commands and `TerminalStyleConfig` (no color when not a TTY, `NO_COLOR`, `--no-color`, `TERM=dumb`; `FORCE_COLOR=1`); sibling `*_test.rs` unit tests for every implementation file this group adds (R-071); they fail
- [x] 6.2 Green: `PrepareLinkCommand`, `ServeCommand`, `InstallAgentCommand`, `UninstallAgentCommand`, `StartAgentCommand`, `StopAgentCommand`, `AgentStatusCommand`, `TerminalStyleConfig` writing to an injected writer, output as in the agreed mock; the tests pass
- [x] 6.3 Red: slice features `link_sharing.feature`, `server_settings.feature`, `background_service.feature` (one `Rule:` per requirement, every scenario), steps in `tests/steps/`, fakes for every secondary port, temp home for settings; component feature `tests/features/component/link_sharing.feature` (a link printed by the link command, followed through the HTTP adapter, opens through the fake opener); they fail
- [x] 6.4 Green: `LinkserverConfig` builds the commands (with its sibling `linkserver_config_test.rs`); the scenarios pass
- [x] 6.5 Commit (`kit ship`)

## 7. App and E2E

- [x] 7.1 Red: `app/e2e/tests/features/{link_opening,link_sharing,server_settings,background_service}.feature` (every spec scenario, the capability tag, `@e2e`, `@serial` where a port is fixed), steps spawn `SUT_BIN` with a temp `HOME`, recording `open`, `launchctl` and `osascript` scripts and a free port written to the settings, and kill it in an `after` hook; against the stub `main` they fail
- [x] 7.2 Green: `apps/http2scheme/src/main.rs` — clap entry (`serve`, `link`, `install`, `uninstall`, `start`, `stop`, `status`, `--no-color`), reads `HOME`, `HTTP2SCHEME_OPEN_PROGRAM`, `HTTP2SCHEME_LAUNCHCTL_PROGRAM`, `HTTP2SCHEME_OSASCRIPT_PROGRAM` (every environment literal lives here, R-083), builds `LinkserverConfig`, serves with graceful stop on SIGTERM; `cargo run -- --help` lists the commands
- [x] 7.3 Run the E2E on the built binary (`just e2e`); every E2E scenario passes
- [x] 7.4 Commit (`kit ship`)

## 8. The gate

- [x] 8.1 `scripts/arch/` checks (layout R-011–R-018, layer imports R-021–R-024, R-027, purity R-032–R-035, naming R-040–R-054, R-071/R-072 sibling tests, R-061/R-062 adapter ITs, R-063, R-073 slice per primary adapter, R-064/R-065 features, R-081 no TODO, R-127 exact versions, sizes R-131/R-135/R-137/R-138) each reading `QA_ROOT`; `scripts/check-complexity-ratchet.sh` with `metrics/cyclomatic.tsv`
- [x] 8.2 `tests/gate_fixtures/<check>/{violation,clean}/` with `expect.txt` per check, `exempt.tsv`, `scripts/check-gate-fixtures.sh` (R-129): every check fails its violation tree and passes its clean tree
- [x] 8.3 `scripts/rules.tsv` ledger with one verdict per registry rule (enforced by a named check, or not applicable with a reason — the documentation rules: "not applicable: no docs/ until the documentation change"; R-128: "not applicable: AGENTS.md runs the checks once per change") and `scripts/check-rule-coverage.sh` (R-096)
- [x] 8.4 `scripts/qa.sh` (`--fix`, `--clean`, `--stage`, `--skip`, `--json`, `--no-color`, `--help`; stages in R-091 order as in D11, the `--ignored` real-program stage included, with `[ OK | FAIL | WARN ] <stage> (<n>s)`; both workspaces) and the remaining recipes (`check`, `complexity`, `machete`, `unsafe-count`, `modules`, `typos`, `recipes`, `dup`, `arch`, `coverage`); install `cargo-modules` 0.28.0
- [x] 8.5 `README.md` (one screen: Install, Run (dev), Run (production), Test, allow the Automation prompt once per browser, how the agent writes links); `AGENTS.md` (layout, check commands, Gherkin levels, release) and `.claude/kit.yaml` (build inputs, version file `app/app/Cargo.toml`, release command, manual-run area `adapters/secondary/linkopening/**`, `browsertabs/**` and `agentcontrol/**`); `kit doctor` reports no config problem
- [x] 8.6 Commit (`kit ship`)

## 10. Menu bar component

- [x] 10.1 Red: crate `components/menubar` (`lints.workspace = true`; the workspace `unsafe_code` becomes `"deny"`, `linkserver` and the app keep `#![forbid(unsafe_code)]`, D15; `objc2` 0.6.4, `objc2-foundation`, `objc2-app-kit`, `objc2-service-management` 0.3.2 pinned with `=`); sibling unit tests for `ServerStateEnum` (status line words), `LoginItemStateEnum`, `MenuViewValueObject` (title with version, status line, login title, check mark, enabled), `AppFolderEnum`, `MenuCommandEnum` and `MenuActionError`; they fail
- [x] 10.2 Green: implement them; the tests pass
- [x] 10.3 Red: unit tests for `OperateMenuUseCase` (`Show` answers the view; `ToggleLoginItem`: `Off` registers, `On` unregisters, `NeedsApproval` opens the Login Items pane, `Unavailable` does nothing; `OpenFolder` opens the folder, which the opener gateway creates first, and an opener failure is a `MenuActionError`) and `OfferLoginItemUseCase` (the D16 cases), with `mockall` ports; they fail
- [x] 10.4 Green: the `#[async_trait]` ports and the two use cases; the tests pass
- [x] 10.5 Red: integration tests `serve_output_server_state_gateway_integration_test.rs` (the `serve` lines in, the states out), `filesystem_first_launch_repository_integration_test.rs` (temp folder), `launch_services_folder_opener_gateway_integration_test.rs` (recording script: one argument, the folder; a non-zero exit is an error), `sm_app_service_login_item_gateway_integration_test.rs` (outside a bundle: `Unavailable`, register refused, the system never asked; the bundle path check), `status_menu_handler_integration_test.rs` (`harness = false`, on the main thread: the items in order with their words; choosing an item runs its command through the use case); sibling unit tests for every implementation file (R-071); they fail
- [x] 10.6 Green: the five adapters and `AppKitConfig`; `docs/decisions/ADR-001-appkit-ffi.html` (`kit report`); `scripts/arch/adr.sh` (R-006: the five ADR sections) with `tests/gate_fixtures/adr/{violation,clean}/` and the R-006 row of `scripts/rules.tsv` turned into `check adr`; `metrics/unsafe.tsv` and the per-file count in `just unsafe-count` (R-144); the tests pass
- [x] 10.7 Red: `tests/features/slice/menu_bar_app.feature` (one `Rule:` per requirement, every slice scenario of the design's table), steps in `tests/steps/menu_bar_app.rs`, harness `tests/cucumber_integration.rs` with `harness = false` on the main thread, fakes for every secondary port; they fail
- [x] 10.8 Green: `MenubarConfig` (with `MenubarSetup` and its sibling tests) builds the handler and the use cases; the scenarios pass
- [x] 10.9 Commit (`kit ship`)

## 11. App mode, bundle and disk image

- [x] 11.1 Red: `app/e2e/tests/features/menu_bar_app.feature` and `app_distribution.feature` with the `@bundle` scenarios of the design's table, steps on `dist/http2scheme.app` and its disk image (`BUNDLE_APP`, `BUNDLE_DMG`), the app started with a temp `HOME` and a recording `open`; they fail
- [x] 11.2 Green: `main.rs` runs the menu bar app when it has no command and runs from a bundle (D14); `app/packaging/Info.plist`, `draw-artwork.swift`, `icons/http2scheme.iconset` (ten sizes by `sips`), `dmg-background.png`; recipes `bundle`, `dmg`, `_sign`, `_require-signing-identity`, `_require-release-identity`, `signing-identity`, `_require-create-dmg`; the `bundle` stage of `scripts/qa.sh` after `build`, before `coverage`; the scenarios pass
- [x] 11.3 `README.md` § Install as VoicePuncher's (download, checksum, drag to Applications, Open Anyway, the menu, `http2scheme uninstall` for a CLI agent, the command line tool in the bundle); `AGENTS.md` (layout, the `bundle` stage, § Release: the disk image and its `.sha256` in place of the tarball, § Gherkin: the slice paths of both components); `.claude/kit.yaml` (build inputs `app/packaging/**`, manual-run area `components/menubar/src/adapters/**`)
- [x] 11.4 Commit (`kit ship`)

## 12. Serve Links and the Link History window

- [ ] 12.1 Red: `menubar` unit tests: `ServerStateEnum::Off` ("Off"), `MenuViewValueObject::serve_checked`, `MenuCommandEnum::ToggleServer`, `LinkRecordValueObject`, `HistoryCommandEnum`, `HistoryViewValueObject` (rows newest first, the empty note); `OperateMenuUseCase` on `ToggleServer` (on → `turn_off`, `Off` → `turn_on`); `BrowseHistoryUseCase` (`Show` reads the latest 100; `Replay` replays, then reads again); `linkserver` unit tests for `ReplayLinkUseCase` (allowed → opened and logged, not allowed → refused and logged, malformed → refused and logged, the opener fails → "open failed"; no tab closed); they fail
- [ ] 12.2 Green: the domain types, the ports `ServerSwitchService`, `LinkHistoryRepository`, `LinkReplayService`, the two use cases in `menubar` and `ReplayLink`/`ReplayLinkUseCase` in `linkserver` (D18); the tests pass
- [ ] 12.3 Red: integration tests `serve_output_server_state_gateway_integration_test.rs` (turn on hands a shutdown to the launcher and sets `Starting`; turn off completes it and sets `Off`; a later `serve` line keeps `Off`), `filesystem_link_history_repository_integration_test.rs` (the log written by `linkserver`'s line format: newest first, the last 100 of 105, a damaged line skipped, a missing file empty), `in_process_link_replay_gateway_integration_test.rs`, `replay_link_handler_integration_test.rs` (`linkserver`, recording `open`: an allowed link opens and is logged, a Slack link is refused and logged), `history_window_handler_integration_test.rs` (`harness = false`, main thread: the rows and their words, the empty note, ▶ replays its row's link); the menu's new items in `status_menu_handler_integration_test.rs`; they fail
- [ ] 12.4 Green: the adapters `ServeOutputServerStateGateway` (+ `ServerSwitchService`), `FilesystemLinkHistoryRepository`, `InProcessLinkReplayGateway`, `HistoryWindowHandler`, `ReplayLinkHandler`; `MenubarHooks`, `MenubarConfig::{macos(setup, hooks), start_server, history_window}`, `LinkserverConfig::replay_handler`; `main.rs` wires the launcher and the replayer; `metrics/unsafe.tsv` and ADR-001 name `history_window_handler.rs`; the tests pass
- [ ] 12.5 Red → Green: the slice scenarios of the design's table in `components/menubar/tests/features/slice/menu_bar_app.feature`, with fakes for the switch, the history and the replayer
- [ ] 12.6 Fixes from the spec-test review: a request with no or an unreadable link logs the reason in the link field (`open_link_use_case_test`, `event_to_log_line_mapper_test`); "A second install replaces the first" starts from a definition naming an old program and asserts it is gone (slice + E2E); D2 names both components; the slice step of "A scheme link becomes an open link" follows the link; the slice of "A failed tab close keeps the link opened" asserts the close was tried; task 10.3 says the gateway creates the folder; the design table adds slice to "Two first uses create two different keys"; D7 says a failed log write keeps the answer
- [ ] 12.7 README § Install and the menu: Serve Links, Link History; commit (`kit ship`)

## 13. Release

- [x] 13.1 Add `just release` (D12); bump `version` in `app/app/Cargo.toml` to 0.1.0; `kit version-check local-link-server` passes
- [x] 13.2 Commit (`kit ship`)
- [x] 13.3 `just release` publishes the disk image and its `.sha256` in place of the tarball (D12); the version stays 0.1.0, the first release
- [x] 13.4 Commit (`kit ship`)

Manual checks the user performs:

- [ ] 13.5 Open `dist/http2scheme-v0.1.0.dmg`, drag the app to Applications, open it (Open Anyway once); the menu bar shows the link icon, the menu reads `Running on http://127.0.0.1:7777` Serve Links and Start at Login have a check mark; in the Claude chat, click a link printed by `/Applications/http2scheme.app/Contents/MacOS/http2scheme link 'idea://open?file=<a file>&line=<n>'` — IntelliJ opens that file at that line and the browser tab closes (after allowing the Automation prompt once); the same with a `goland://` link
- [ ] 13.6 Restart the Mac; the app is in the menu bar and a link still opens with no other step; Open Logs Folder shows `requests.log` holding each click; Link History lists the clicks newest first and ▶ on a row opens its link again; Serve Links off makes a click fail to connect, on again makes it work
