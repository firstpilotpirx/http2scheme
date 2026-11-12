## Why

Some apps, the Claude chat among them, make only `http://` and `https://` links clickable: a custom URL scheme link such as `idea://open?file=…&line=17` or `slack://channel?…` shows as plain text, so one click cannot open a place in IntelliJ, GoLand, Slack or any other app. A local server that takes an `http://127.0.0.1` link and opens the scheme link it carries gives back the one-click jump, for any app that registers a scheme. The first design (`assets/original-spec.md`, 2026-11-12) targeted only JetBrains IDEs and Python; this change makes it general and builds it in Rust. The owner, 2026-11-12, in the chat (translated): "This tool is not for the IDE in particular; it is general, for the programs that cannot open a slack:// link but can open an http link." On the browser tab, the same day: "If you can close this tab by itself, so the tabs do not pile up, then close it." This order replaces the first design's JetBrains-only checks (an existing full file path, a number for the line, an encoded path; original-spec.md lines 112–114) and its out-of-scope line "Other editors" (line 149) with the whole-link checks of design D1 and D4.

## What Changes

- New program `http2scheme`: a local HTTP server on `127.0.0.1` (port 7777 by default) that answers `GET /open?key=<key>&url=<scheme link>` by running macOS `open` on the scheme link, without a shell, and then closes the browser tab the click opened (Safari, Chrome, Arc, Brave, Edge; the user allows it once per browser), so tabs do not pile up.
- A link opens only when its key matches the secret key (compared in constant time) and its scheme is on the allowed list (`idea` and `goland` by default); a wrong key, a malformed link, a scheme not on the list, any other path and a failed `open` each get their own answer.
- First start creates the settings folder (`~/Library/Application Support/http2scheme/`) with a settings file (port, allowed schemes, tab closing switch) and a key file of 256 random bits readable only by the user.
- Every open request is written to a log file (`~/Library/Logs/http2scheme/requests.log`): time, result, scheme link; the key is never written.
- `http2scheme link <scheme link>` prints the `http://127.0.0.1` link for a scheme link, with the key filled in, so the agent (or the user) writes links without reading the key file by hand.
- `http2scheme install | uninstall | start | stop | status` manage a macOS LaunchAgent that starts the server at login and restarts it after a crash; `status` says whether the agent is installed and loaded and whether the server answers.
- `http2scheme serve` runs the server in the foreground (what the LaunchAgent runs).
- A Mac app, `http2scheme.app`, installed the way VoicePuncher is: a disk image `http2scheme-vX.Y.Z.dmg` with its `.sha256` on each GitHub release, dragged to Applications, Open Anyway once per version. The app lives in the menu bar (no Dock icon), runs the server inside itself, shows its state (`Running on http://127.0.0.1:7777`, `Stopped: <reason>`), offers Serve Links (turns the server off and on), Link History (a window with the latest 100 requests of the request log, each with a ▶ button that opens its link again), Start at Login (macOS login items, on by itself at the first launch from Applications), Open Settings Folder, Open Logs Folder and Quit; its program is also the command line tool. The owner, 2026-11-12, in the chat (translated): "build it like VoicePuncher, the way I already have it — not a separate change, in this current task". On the menu, the same day (translated): "a tray icon with the possibility to switch it on and off, and to switch start at login on and off … in the GUI also a log of who called which URLs, at least the last 100, shown in the GUI, with the possibility to run it again — a play arrow that runs the line again … do it in this task, the same technologies as VoicePuncher".
- The repository gets its code layout (hexagonal component, composition-root app, E2E workspace), the full test pyramid (unit, adapter integration, slice, component, E2E) and the check gate (`scripts/qa.sh` behind `just check`: format, clippy, the static analysis stack, the architecture and test-layout rules, all tests, release build).

Current stage: Idea. Target stage: PoC.
Scope: the server, its settings and key, the request log, the link command, the LaunchAgent commands, the menu bar app with its server switch, its link history window and its login item, the app bundle and its disk image, the test pyramid and the check gate.
Acceptance bar: `just check` passes; the app installs from the disk image by dragging it to Applications; a click on a link printed by `http2scheme link 'idea://open?file=<path>&line=<n>'` opens IntelliJ at that line, the same for GoLand; a wrong key, a scheme not allowed and a malformed link open nothing; the browser tab of an opened link closes by itself; after the app's first launch and a Mac restart, links work with no other step; each request is in the log; Serve Links turns the server off and on; the Link History window lists the latest requests and its ▶ opens a row's link again.
Owner decisions, 2026-11-12, in the chat: recording scripts may stand in for `open`, `launchctl` and `osascript` in the adapter integration tests and the E2E (answer "Yes, replace them"), which overrules `testing-pyramid.md` "Real adapter + real infra. No mocks." and `testing-e2e-rust.md` "Real or production-equivalent" for these three programs only, so tests open no app and touch no LaunchAgent of the user; the real programs run in harmless `--ignored` tests (design D8). The owner's order on the build, the same day (translated): "Although the project is small, everything must be done right: hexagonal, clean architecture, all these adapters, all the tests in full"; it replaces the AGENTS.md line "One Cargo crate at the repository root" and its `tests/features/` paths with the `app/app` + `app/e2e` layout of design D2. The libraries `percent-encoding`, `getrandom`, `subtle` and `tempfile`, not in the tech matrix, are approved (answer "Yes"). The app and its installer follow VoicePuncher (the owner's order above), with VoicePuncher's `objc2` crates and `create-dmg`. The Claude rule from original-spec.md line 127 (write places in code as helper links) goes to the agent-kit repository as its own change (answer "As a separate task"); this change adds only the README section.
Out of scope: Windows and Linux; the Claude link-writing rule (agent-kit change); per-scheme link templates (file + line fields checked per app); a settings window (the settings stay a file the menu opens); a Serve Links state kept across launches (the app always starts serving); HTTPS; the documentation set (arc42 pages, project map, feature pages), filed as its own change; this change writes only ADR-001, which `quality-rust.md` asks for before any `unsafe` (design D15).
Interface: the "Opened" and error pages in the browser (the "Opened" page shows only until its tab closes); the terminal output of the `link`, `install`, `uninstall`, `start`, `stop`, `status` and `serve` commands; the menu bar item and its menu; the Link History window; the disk image window.

## Capabilities

### New Capabilities

- `link-opening`: opening an allowed scheme link from a keyed `http://127.0.0.1` request, closing the browser tab it leaves, the answers to every kind of request, and the request log.
- `link-sharing`: printing the `http://127.0.0.1` link for a scheme link.
- `server-settings`: the settings and key created at first start, and how the user changes them.
- `background-service`: the LaunchAgent that keeps the server running, and the commands that install, remove, start, stop and check it.
- `menu-bar-app`: the app in the menu bar that runs the server, shows its state, turns it off and on, shows the link history and opens a link again, its login item and its menu actions.
- `app-distribution`: the app bundle, its signature, the disk image with its checksum, and their release.

### Modified Capabilities

None.

## Impact

- New Cargo workspace `app/app/` (app `apps/http2scheme`, components `components/linkserver` and `components/menubar`) and E2E workspace `app/e2e/`; `justfile`, `scripts/qa.sh`, `rust-toolchain.toml`, `clippy.toml`, `deny.toml`, `README.md`, `app/packaging/` (Info.plist, icon, disk image background), `docs/decisions/ADR-001-appkit-ffi.html`, `metrics/unsafe.tsv`.
- `AGENTS.md`, `.claude/kit.yaml`: layout, check commands, build inputs, version file, release command.
- Dependencies: `tokio`, `axum`, `clap`, `async-trait`, `serde`, `config`, `percent-encoding`, `getrandom`, `subtle`, `thiserror`, `anyhow`, `log`, `env_logger`, `owo-colors`, `ureq`, `objc2`, `objc2-foundation`, `objc2-app-kit`, `objc2-service-management`; tools: `create-dmg` 1.3.0; test: `mockall`, `cucumber`, `reqwest`, `wiremock`, `assert_cmd`, `tempfile`.

## Release

SemVer: MINOR — the first capability.
Target version: 0.1.0.
