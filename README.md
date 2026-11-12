# http2scheme

Some apps, like the Claude chat, open only `http://` and `https://` links. `http2scheme` is a tiny
local server for macOS that turns a click on such a link into a custom scheme link
(`idea://`, `goland://`, `slack://`, …): it runs `open` on it and closes the browser tab.
It listens only on `127.0.0.1`, needs a secret key in every link, and opens only the schemes it allows.

## Install

```bash
cargo install --path app/app/apps/http2scheme
http2scheme install
```

`install` writes the settings (`~/Library/Application Support/http2scheme/settings.toml`) and the key on
first use, then starts the server as a login agent, so it also runs after a restart.
`http2scheme status` shows three ✓ when all is well; `http2scheme uninstall` removes the agent.

The first time a link closes its tab, macOS asks to let `http2scheme` control the browser
(Automation). Allow it once per browser (Safari, Chrome, Arc, Brave, Edge). Until then, or in Firefox,
the tab stays open with an "Opened" page.

## Run (dev)

```bash
cargo run --manifest-path app/app/Cargo.toml -p http2scheme -- serve
```

## Run (production)

The login agent from § Install runs `http2scheme serve`. Control it with `http2scheme start`, `stop`
and `status`. Every request is written to `~/Library/Logs/http2scheme/requests.log`.

Settings (`settings.toml`): `port` (default 7777), `allowed_schemes` (default `idea`, `goland`; `http`,
`https` and `file` are never opened) and `close_tab` (default `true`). Restart the agent after an edit.

## Test

```bash
just test
```

```bash
just e2e
```

```bash
just check
```

`just check` runs the whole gate (`scripts/qa.sh --help` lists its stages and flags).

## Links from an AI agent

An agent that wants to give you a clickable place in code runs:

```bash
http2scheme link 'idea://open?file=/path/to/File.kt&line=42'
```

and pastes the `http://127.0.0.1:7777/open?key=…&url=…` link it prints into the chat. A click opens the
file in the IDE at that line. Put the scheme link in single quotes; write a space in it as `%20`.
