## 1. Workspaces and tooling

- [x] 1.1 Create `app/app/` (workspace: `apps/http2scheme` bin, `components/linkserver` lib with empty `core/{domain,application}` and `adapters/{primary,secondary}` modules, `[workspace.package] version = "0.0.0"`, the `[workspace.lints]` table of `tech-matrix-rust.md`, `clippy.toml` with the size thresholds of R-130/R-132/R-133/R-134 and `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests = true`), `app/e2e/` (workspace with the same `[workspace.lints]` table and `clippy.toml`, every crate `lints.workspace = true`, cucumber harness `cucumber_e2e`), `clap` and `env_logger` without their `color` features, `rust-toolchain.toml` pinned to 1.99.0, `deny.toml`, `.gitignore`; every dependency pinned with `=`; `cargo build` succeeds in both workspaces
- [x] 1.2 Add `justfile` with `test-one`, `test`, `e2e`, `fmt`, `lint`, `deny` and the `_require-<tool>` guards
- [x] 1.3 Commit (`kit ship`)

## 2. Domain

- [x] 2.1 Red: unit tests for `SchemeNameValueObject` and `SchemeLinkValueObject` (D4: length 1–4096, control characters, space, scheme grammar, lower-case scheme, exact text kept); `just test-one` fails
- [x] 2.2 Green: implement both; the tests pass
- [x] 2.3 Red: unit tests for `AccessKeyValueObject` (64 hex, from 32 bytes, `matches` true only for the same key, false for wrong, empty and other-length candidates, redacted `Debug`) and `PortValueObject` (1024–65535); they fail
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

## 9. Release

- [x] 9.1 Add `just release` (D12); bump `version` in `app/app/Cargo.toml` to 0.1.0; `kit version-check local-link-server` passes
- [x] 9.2 Commit (`kit ship`)

Manual checks the user performs:

- [ ] 9.3 `cargo install --path app/app/apps/http2scheme`, `http2scheme install`; in the Claude chat, click a link printed by `http2scheme link 'idea://open?file=<a file>&line=<n>'` — IntelliJ opens that file at that line and the browser tab closes (after allowing the Automation prompt once); the same with a `goland://` link
- [ ] 9.4 Restart the Mac; a link still opens with no other step; `http2scheme status` shows three ✓; `~/Library/Logs/http2scheme/requests.log` holds each click
