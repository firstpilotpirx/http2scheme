# Design

## Context

`http2scheme link` prints one open link per call; coding agents do not know about it. This change adds a skill text (open `SKILL.md` format), a `skill` command that prints and installs it, a many-link `link`, and an Agents tab in the Settings window. Two components keep their roles: `linkserver` owns every rule about the skill (its text, where it lands, whether a copy is current, writing it); `menubar` only draws the tab and remembers the folder, and reaches `linkserver` through `MenubarHooks` closures that `main.rs` wires, as `replay` already does.

## Decisions

| # | Question | Chosen | Why | Rejected |
|---|---|---|---|---|
| D1 | Where the skill text lives | `skills/http2scheme/SKILL.md` at the repository root, built into the binary with `include_str!` | one universal file people can read and copy; `cargo install` users get it too; no bundle resource to keep in step | a file in `Contents/Resources` (missing outside the app); text inside Rust code (not readable as a skill) |
| D2 | How the skill names the program | the template holds `{{program}}`; the domain renders it with the running program's full path (`std::env::current_exe`) | the agent runs the program even when it is not on `PATH` (the app's binary lives in the bundle) | asking the user to add the binary to `PATH` |
| D3 | Many links, one refused | the whole call fails, prints a reason per refused link and no link | line *n* of the output always belongs to input *n*; the skill's fallback handles a failure in one rule | printing an empty line for the refused one (agents misread blanks) |
| D4 | IDE scheme per language | in the skill text: Go → `goland`, every other language → `idea`; `rustrover` not added to the defaults | the two default schemes; IntelliJ IDEA opens Rust; RustRover is not installed on the owner's Mac | `rustrover` default (a scheme nobody here can open) |
| D5 | Skill state | `linkserver` compares the installed file's bytes with the rendered text: absent → `NotInstalled`, equal → `Installed`, different → `Outdated` | no version field to keep; a hand edit also reads as outdated, and Update restores the text | a version header in the file |
| D6 | Linked `skills` folder | the repository resolves the real path of `<claude dir>/skills` (`std::fs::canonicalize` when it is a link) and returns it beside the file path; the command and the tab show it | the owner's `~/.claude/skills` points into the agent-kit repository: the write is allowed, but the user must see where it goes | refusing to write; writing the link target silently |
| D7 | Remembering the folder | `menubar` keeps the last installed-to folder in `agent-skill-folder` (one line, the path) in the settings folder; the Agents tab starts from it, else `~/.claude` | the server settings file belongs to `linkserver` and is read only at start | a field in `settings.toml` |
| D8 | Refresh at launch | at launch, before the menu runs, `menubar` asks for the state of the remembered folder and installs only when it is `Outdated` | an app update updates the skill; a deleted file stays deleted (`NotInstalled` → nothing) | refreshing every known Claude folder |
| D9 | Clipboard | the Agents tab handler puts the text on `NSPasteboard.generalPasteboard` itself | it is drawing-side AppKit work, like the window; no port needed | a clipboard service port |
| D11 | `unsafe` for the new tab | a fifth unsafe file, `agents_tab_handler.rs` (`#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]`, `// SAFETY:` on every block); ADR-001 lists five files; `metrics/unsafe.tsv` gets its row and the new count of `settings_window_handler.rs` in the same commit | its buttons need the same target/action wiring as the General tab; keeping it out of `settings_window_handler.rs` keeps that file small | squeezing the wiring into `settings_window_handler.rs` |
| D12 | New crate features | `objc2-app-kit`: `NSOpenPanel`, `NSSavePanel`, `NSPasteboard`; `objc2-foundation`: `NSURL` — no new crates | the folder panel and the clipboard | a file-dialog crate |
| D13 | Where paths come from | `main.rs` reads `HOME` and `std::env::current_exe` and hands them in (`LinkserverSetup.program`, a new `LinkserverSetup.claude_dir` = `~/.claude`, and `MenubarSetup.claude_dir`); `core` never reads the environment | as the first change's D3 and D8 do for every path | reading `HOME` in the use case |
| D14 | Cross-component calls | `linkserver` gets three primary in-process handlers, one per use case (R-036): `agentskill/inprocess/skill_text_handler.rs`, `inspect_skill_handler.rs`, `install_skill_handler.rs`; inspect and install answer plain values (file, real file, state name), so `main.rs` imports no core type of either component (R-031) and turns them into the three `MenubarHooks` closures through `menubar_hooks::skill_place`; `menubar`'s `in_process_agent_skill_gateway.rs` calls the hooks | the replay pair (`replay_link_handler` ↔ `in_process_link_replay_gateway`) does the same; each component owns its data | `menubar` writing the skill file itself |
| D10 | Tab layout | a third segment `Agents` in the existing tab bar; three rows in the General row style (name, grey line, a button on the right instead of a switch); the window is as high as the shown tab (design D19 of the first change) | matches the agreed mock and the window's look | a separate window |

## New and changed code

`linkserver` (hexagonal, as `architecture-rust.md`):

- domain: `agent_skill_value_object.rs` (renders the template for a program path), `skill_state_enum.rs` (`NotInstalled`, `Installed`, `Outdated`), `skill_place_value_object.rs` (the file path, the real file path when the skills folder is a link, the state).
- application: repository port `skill_repository.rs` (`find(claude_dir)` → current text and real path, `save(claude_dir, text)`); input ports and use cases `show_skill` / `show_skill_use_case.rs`, `inspect_skill` / `inspect_skill_use_case.rs`, `install_skill` / `install_skill_use_case.rs`; exception `install_skill_error.rs`. `prepare_link` keeps one link per call; the command loops.
- adapters: secondary `agentskill/filesystem/filesystem_skill_repository.rs`; primary `agentskill/cli/show_skill_command.rs`, `agentskill/cli/install_skill_command.rs`, `agentskill/inprocess/skill_text_handler.rs`, `inspect_skill_handler.rs`, `install_skill_handler.rs` (D14); `linksharing/cli/prepare_link_command.rs` takes a slice of links.
- `linkserver_config.rs`: builds the two commands and exposes the three use cases for the hooks.

`menubar`:

- domain: `skill_place_value_object.rs` and `skill_state_enum.rs` (its own copies of the shapes the hooks return, so the components stay apart), `agents_view_value_object.rs` (the three rows' texts and the button label).
- application: repository port `skill_folder_repository.rs`; service port `agent_skill_service.rs` (text, inspect, install); domain `agents_command_enum.rs` (Show, Install, Copy); use case `manage_skill` / `manage_skill_use_case.rs` with one `execute(AgentsCommandEnum)` (view for a folder, choose a folder, install, copy text; the copied text rides in the view); use case `refresh_skill` / `refresh_skill_use_case.rs` (refresh at launch, D8).
- adapters: secondary `skillfolder/filesystem/filesystem_skill_folder_repository.rs`, `agentskill/inprocess/in_process_agent_skill_gateway.rs` (wraps the hooks); primary `statusmenu/appkit/agents_tab_handler.rs` (rows, `NSOpenPanel` for Choose…, `NSPasteboard` for Copy), `settings_layout_config.rs` (three tabs).
- `menubar_hooks.rs`: `skill_text`, `inspect_skill`, `install_skill` closures.

`apps/http2scheme/src/main.rs`: `Link { links: Vec<String> }` (one or more, `required = true`), `Skill { #[command(subcommand)] Install { --claude-dir } }` (no subcommand prints the text), hook wiring.

## Test levels

| Spec scenario | Level |
|---|---|
| agent-skill: The printed skill names the program and the link style | unit (domain render) + slice (`linkserver` link_sharing-style feature `agent_skill.feature`) + E2E (`http2scheme skill` on the binary) |
| agent-skill: Install writes the skill into the default folder | slice + E2E (`skill install` with a temp `HOME`) |
| agent-skill: Install into a chosen work profile folder | slice |
| agent-skill: Install over an older skill replaces it | slice |
| agent-skill: A linked skills folder is named in the output | integration (`filesystem_skill_repository`) + slice |
| link-sharing: A scheme link becomes an open link | unchanged (existing slice and E2E) |
| link-sharing: A path with spaces survives the round trip | unchanged (existing slice) |
| link-sharing: The first link command creates the key | unchanged (existing slice) |
| link-sharing: A scheme not on the list prints the reason | unchanged (existing slice) |
| link-sharing: Several scheme links become open links in order | slice + E2E |
| link-sharing: One refused link among several prints no link | slice |
| menu-bar-app: Settings opens the General tab; Link History opens the History tab | unchanged (existing slice) |
| menu-bar-app: A fresh Mac shows the default folder and no skill | slice (`menubar`, real Settings window, fake hooks) |
| menu-bar-app: Choosing a work folder shows its state | slice (the folder picker's result injected through the tab's choose entry, `NSOpenPanel` itself in the manual run) |
| menu-bar-app: A linked skills folder shows where the file lands | slice |
| menu-bar-app: Install puts the skill into the shown folder | slice |
| menu-bar-app: Update replaces an older skill | slice |
| menu-bar-app: Copy puts the skill on the clipboard | slice (reads the general pasteboard back) |
| menu-bar-app: A new app version refreshes the installed skill | slice |
| menu-bar-app: A removed skill stays removed | slice |

Manual run (`menu-bar-app` area): the agent's visual check of the Agents tab in light and dark, Choose… with the real folder panel, and one Claude Code session that uses the installed skill.

## Interface mock

The agreed Agents tab, light and dark, approved in the chat on 2026-11-16: [agents-tab-mock.md](assets/agents-tab-mock.md).

## Risks

- `include_str!` reaches outside the crate (`../../../../../skills/…`): a moved file breaks the build at once, which is the wanted signal.
- The skill text is advice; an agent may still skip the `IDE` link. The manual run checks one real session.
- Writing into a linked `skills` folder adds an untracked file to the repository it points to; the tab and the command name the real place so the user decides.
