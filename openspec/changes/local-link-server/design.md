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
    components/linkserver/  lib crate: the one component (core + adapters + LinkserverConfig)
  e2e/                      Cargo workspace: cucumber E2E on the built binary
scripts/qa.sh               the gate; scripts/arch/*.sh the layout and test-layout rules
justfile                    every check recipe; `just check` = qa.sh
```

One component: settings, key, link opening, link sharing and the agent share one data set (the settings) and one lifecycle; splitting them would add gateways with nothing gained (`architecture.md` § Decision Rules). Rejected: one flat crate at the repo root, as the first AGENTS.md said (line 5, written at kit setup) — breaks R-011 and the `{service}/{service}` + `{service}/e2e` layout; the owner's order (proposal, Owner decisions) replaces it, and task 8.5 rewrites AGENTS.md.

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

Small inline HTML, no external resource, light and dark through `prefers-color-scheme`. 200 "Opened" (also tries `window.close()`, the fallback for Firefox, which D13 cannot reach), 403 "Wrong key", 400 "Malformed link" or "Scheme not allowed: <scheme>" (HTML-escaped), 404 "Not found", 405 "Method not allowed", 502 "Could not open the app". `/health` answers 200 `ok` as plain text. `Cache-Control: no-store` and `Referrer-Policy: no-referrer` on every answer, so the key does not leak through a cached page or a referrer.

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

`version` in `[workspace.package]` of `app/app/Cargo.toml`; `just release` on a clean `origin/main`: the gate, a release build, `http2scheme-vX.Y.Z-aarch64-apple-darwin.tar.gz` with its `.sha256`, tag, `gh release create`.

## Test levels

Slice: `app/app/components/linkserver/tests/features/slice/<capability>.feature` with fake secondary adapters. Component: `…/features/component/link_sharing.feature`. Integration: the adapter's `*_integration_test.rs`. E2E: `app/e2e/tests/features/<capability>.feature` on the built binary with recording `open`, `launchctl` and `osascript` scripts (D8).

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
| server-settings: Two first uses create two different keys | integration + E2E |
| server-settings: Existing settings and key are kept | slice + E2E |
| server-settings: An allowed scheme added by hand is honored | slice + E2E |
| server-settings: A broken settings file stops the start with its reason | slice + E2E |
| server-settings: A key file readable by others stops the start | slice + E2E |
| key comparison timing (D5) | not tested: a timing test is too noisy to pass or fail reliably; `AccessKey::matches` uses `subtle::ConstantTimeEq`, and its unit test checks equal keys and keys that differ in the first and in the last byte |
| manual run | the user clicks printed IntelliJ and GoLand links in the Claude chat and the tab closes; after `install` and a Mac restart, a link still works |

## Interface mock

[http2scheme interface mock](https://claude.ai/artifact/DDotUxHvNFqK8Cbujs7GRh) — the browser pages and the command output, each in light and dark. The LaunchAgent label is `io.github.firstpilotpirx.http2scheme`.

## Risks / Trade-offs

- [The browser leaves a tab after each click] → D13 closes it in the five supported browsers; Firefox keeps it (the page tries `window.close()`).
- [The Automation permission prompt] → shown once per browser on the first click; `README.md` says to allow it.
- [The key is in browser history] → only local users can read it; history holds the same key the key file holds. Rotating the key = deleting the key file.
- [A program that registers an allowed scheme gets any text after `scheme:`] → only the user's own allow list counts, and `http`, `https`, `file` are never allowed.
- [DNS rebinding: a website resolving its name to 127.0.0.1] → it still needs the key; the server also answers 421 when the `Host` header is not `127.0.0.1:<port>` or `localhost:<port>`.
- [The LaunchAgent points at the binary's path at install time] → `install` prints that path; moving the binary needs a new `install`.
