# Proposal

## Why

`http2scheme link` makes a clickable IDE link, but a coding agent (Claude Code, Codex, Gemini CLI) does not know it exists, so its replies still point to code with plain paths. The owner, 2026-11-13 to 2026-11-16, in the chat (translated): the app should teach agents to use it "natively"; the skill lives in this repository, is universal, and the app installs it — "a GUI to choose the folder where Claude lives, if someone has a work one, a default and a choice, plus a button to copy the skill's text so it can be installed some other way". On the link look the owner picked style D: the short file link, then a small `IDE` link.

## What Changes

- A universal agent skill (the open `SKILL.md` format) in this repository, built into the program. It tells the agent: every time a reply points to a file and a line, write the short file link and right after it a small `IDE` link, `[server.rs:42](app/src/server.rs:42) [IDE](http://127.0.0.1:7777/open?…)`; make all IDE links of one reply with one `link` call; pick the scheme from the project's main language (Go → `goland`, every other language → `idea`); when the program is missing, the server is off or the link is refused, write only the file link and one line on how to fix it. The skill names the full path of the program that installed it, so it works without `http2scheme` on the `PATH`.
- `http2scheme link` takes one or more scheme links and prints one open link per line, in the same order. When any link is refused it prints no link, a reason per refused link, and fails.
- `http2scheme skill` prints the skill text; `http2scheme skill install [--claude-dir <folder>]` writes it to `<folder>/skills/http2scheme/SKILL.md` (default folder `~/.claude`) and prints where it wrote it.
- The Settings window gets a third tab, Agents: the Claude Code folder (default `~/.claude`, a Choose… button picks another, for example a work profile), the place the skill lands (the real folder when `skills` is a link to another folder, so the user sees that the write goes there), whether the skill is installed and current, an Install button (Update when an older text is there) and a Copy skill text button that puts the text on the clipboard.
- At launch the app rewrites the installed skill in the folder last installed to when its text differs from the app's text, so the skill follows app updates.
- README: the "Links from an AI agent" section describes the skill and both ways to install it.

Current stage: PoC. Target stage: PoC.
Scope: the skill text, the multi-link `link` command, the `skill` command, the Agents tab, the refresh at launch, the README section.
Acceptance bar: `just check` passes; after Install in the Agents tab, a new Claude Code session that answers a question about code writes each place as a file link followed by an `IDE` link, and a click on `IDE` opens the place in IntelliJ or GoLand; `http2scheme link` with three links prints three links in order; Copy skill text puts the same text on the clipboard as `http2scheme skill` prints.
Decided by the agent: `rustrover` is not added to the default allowed schemes (RustRover is not installed on the owner's Mac and IntelliJ IDEA opens Rust projects), so Rust maps to `idea`; the skill is built into the program (`include_str!`), not read from the bundle's Resources, so `cargo install` users get it too; a refused link in a many-link call fails the whole call, so every printed line maps to its input by position.
Out of scope: a Claude Code plugin and marketplace; an MCP server for agents without a shell (the Claude desktop chat); installing into other agents' folders (Codex, Gemini CLI) — the copied text and `skill` cover them by hand; removing the skill from the Agents tab; the agent-kit copy of the skill.
Interface: the Agents tab of the Settings window; the terminal output of `link` with several links and of `skill` and `skill install`; the agent's replies (file link followed by an `IDE` link).

## Capabilities

### New Capabilities

- `agent-skill`: the skill text that teaches coding agents to write IDE links, printing it, installing it into a Claude Code folder and keeping the installed copy current.

### Modified Capabilities

- `link-sharing`: the link command takes several scheme links at once.
- `menu-bar-app`: the Settings window gets the Agents tab that installs the skill and copies its text.

## Impact

- `app/app/components/linkserver`: the `link` command takes a list; a new `skill` command and use cases to render and install the skill; the skill text under the component.
- `app/app/components/menubar`: the Agents tab (AppKit: folder picker `NSOpenPanel`, clipboard `NSPasteboard`), its use case, a small file remembering the chosen folder, the refresh at launch.
- `app/app/apps/http2scheme/src/main.rs`: CLI arguments and wiring.
- `app/e2e`: scenarios for the new commands.
- `README.md`.
- No new dependencies.

## Release

SemVer: MINOR — new capability.
Target version: 0.2.0.
