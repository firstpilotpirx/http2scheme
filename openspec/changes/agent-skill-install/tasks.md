# Tasks

## 1. Skill text and the `skill` command

- [ ] 1.1 Write `skills/http2scheme/SKILL.md`: header (`name: http2scheme`, one-line description), the link style D rule with an example, one `{{program}} link` call per reply, the scheme per language, the fallback line.
- [ ] 1.2 Red: unit test for `AgentSkillValueObject` rendering the template with a program path (header kept, `{{program}}` replaced everywhere).
- [ ] 1.3 Green: `agent_skill_value_object.rs` with `include_str!` of the template.
- [ ] 1.4 Red: slice scenario "The printed skill names the program and the link style" in `tests/features/slice/agent_skill.feature` with its steps.
- [ ] 1.5 Green: `show_skill` port and use case, `show_skill_command.rs`, `Skill` subcommand in `main.rs`, wiring in `linkserver_config.rs`.
- [ ] 1.6 `kit ship` the group.

## 2. Installing the skill

- [ ] 2.1 Red: integration test for `FilesystemSkillRepository`: save creates `skills/http2scheme/SKILL.md`, find reads it back, a linked `skills` folder reports its real path, a sibling skill stays.
- [ ] 2.2 Green: `skill_repository.rs` port, `skill_place_value_object.rs`, `skill_state_enum.rs`, `filesystem_skill_repository.rs`.
- [ ] 2.3 Red: unit tests for `InspectSkillUseCase` (absent, equal, different) and `InstallSkillUseCase` (writes, returns the place, maps a write error).
- [ ] 2.4 Green: `inspect_skill` and `install_skill` ports and use cases, `install_skill_error.rs`.
- [ ] 2.5 Red: slice scenarios "Install writes the skill into the default folder", "Install into a chosen work profile folder", "Install over an older skill replaces it", "A linked skills folder is named in the output".
- [ ] 2.6 Green: `install_skill_command.rs`, `skill install [--claude-dir]` in `main.rs`.
- [ ] 2.7 `kit ship` the group.

## 3. Many links in one `link` call

- [ ] 3.1 Red: slice scenarios "Several scheme links become open links in order" and "One refused link among several prints no link"; unit test for the command printing in order and printing nothing on one refusal.
- [ ] 3.2 Green: `PrepareLinkCommand::run` takes a slice, checks every link first, then prints; `Link { links }` with at least one value in `main.rs`; the existing scenarios keep passing.
- [ ] 3.3 `kit ship` the group.

## 4. Menubar core for the Agents tab

- [ ] 4.1 Red: unit tests for `AgentsViewValueObject` (state line texts, `Install`/`Update` label, linked path line, error line).
- [ ] 4.2 Green: `skill_state_enum.rs`, `skill_place_value_object.rs`, `agents_view_value_object.rs` in `menubar` domain.
- [ ] 4.3 Red: integration test for `FilesystemSkillFolderRepository` (absent → none, save then find).
- [ ] 4.4 Green: `skill_folder_repository.rs` port and `filesystem_skill_folder_repository.rs`.
- [ ] 4.5 Red: unit tests for `ManageSkillUseCase`: view of the remembered folder else `~/.claude`, choose a folder, install remembers the folder, copy returns the text, refresh installs only when outdated.
- [ ] 4.6 Green: `agent_skill_service.rs` port, `manage_skill` port and use case, `in_process_agent_skill_gateway.rs`, the three hooks in `menubar_hooks.rs`, wiring in `menubar_config.rs` and `main.rs`.
- [ ] 4.7 `kit ship` the group.

## 5. Agents tab and refresh at launch

- [ ] 5.1 Red: menubar slice scenarios "A fresh Mac shows the default folder and no skill", "Choosing a work folder shows its state", "A linked skills folder shows where the file lands", "Install puts the skill into the shown folder", "Update replaces an older skill", "Copy puts the skill on the clipboard".
- [ ] 5.2 Green: `agents_tab_handler.rs` (rows, Choose… with `NSOpenPanel`, Install/Update, Copy skill text with `NSPasteboard`), the third segment and tab in `settings_layout_config.rs` and `settings_window_handler.rs`, as the agreed mock; the `objc2-app-kit` and `objc2-foundation` features of D12.
- [ ] 5.3 Update `docs/decisions/ADR-001-appkit-ffi.html` to five unsafe files (D11); add the `agents_tab_handler.rs` row and write back the new count of `settings_window_handler.rs` in `metrics/unsafe.tsv`, in the same commit as 5.2.
- [ ] 5.4 Red: menubar slice scenarios "A new app version refreshes the installed skill" and "A removed skill stays removed".
- [ ] 5.5 Green: refresh at launch in `menubar_config.rs`.
- [ ] 5.6 `kit ship` the group.

## 6. End to end, docs and version

- [ ] 6.1 E2E scenarios on the binary: `skill` prints the skill with the binary's path; `skill install` with a temp `HOME` writes the file; `link` with three links prints three lines.
- [ ] 6.2 AGENTS.md: add `skills/` (the agent skill text, built into the binary) to the layout list.
- [ ] 6.3 README: rewrite "Links from an AI agent" for the skill, the Agents tab, `skill install` and the many-link `link`.
- [ ] 6.4 Bump the version to 0.2.0 in `app/app/Cargo.toml`.
- [ ] 6.5 `kit ship` the group.

## Manual checks

- The agent's visual check of the Agents tab (light and dark), Choose… with the real folder panel, Install into a temp Claude folder.
- One Claude Code session with the installed skill answers a code question with file links followed by `IDE` links; a click on `IDE` opens IntelliJ.
