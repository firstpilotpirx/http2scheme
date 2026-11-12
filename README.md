# http2scheme

Some apps, like the Claude chat, open only `http://` and `https://` links. `http2scheme` is a tiny
local server for macOS that turns a click on such a link into a custom scheme link
(`idea://`, `goland://`, `slack://`, …): it runs `open` on it and closes the browser tab.
It listens only on `127.0.0.1`, needs a secret key in every link, and opens only the schemes it allows.

## Install

1. Download `http2scheme-vX.Y.Z.dmg` from the [latest release](https://github.com/firstpilotpirx/http2scheme/releases/latest) and check it against the `.sha256` beside it: `shasum -a 256 -c http2scheme-vX.Y.Z.dmg.sha256`.
2. Open it and drag **http2scheme** onto **Applications**.
3. Open http2scheme from Applications. macOS says it cannot verify that http2scheme is free of malware, with the buttons **Move to Trash** and **Done**: http2scheme is signed with the project's own certificate, not an Apple Developer ID, and is not notarized. Click **Done**, open **System Settings → Privacy & Security**, scroll to the message about http2scheme and click **Open Anyway**, then confirm with your password. Needed once per version.

A link icon appears in the menu bar. Its menu shows the version, `Running on http://127.0.0.1:7777`, **Serve Links**, **Link History…**, **Start at Login**, **Open Settings Folder**, **Open Logs Folder** and **Quit http2scheme**. **Serve Links** turns the server off (the status line reads `Off`) and on again; the app always starts with it on. **Link History…** opens a window with the latest 100 requests, newest first: time, result and link, with a ▶ button that opens the row's link again (the allowed schemes still apply, and the new attempt is logged). The first launch from Applications adds the app to the login items, so it runs after a restart; uncheck **Start at Login** to stop that. The app writes the settings (`~/Library/Application Support/http2scheme/settings.toml`) and the key on first use.

The first time a link closes its tab, macOS asks to let http2scheme control the browser (Automation). Allow it once per browser (Safari, Chrome, Arc, Brave, Edge). Until then, or in Firefox, the tab stays open with an "Opened" page.

The command line tool is inside the app: `/Applications/http2scheme.app/Contents/MacOS/http2scheme --help`. If you ran an older `http2scheme install`, remove that login agent with `http2scheme uninstall`, so two servers do not compete for the port.

### Without the app

```bash
cargo install --path app/app/apps/http2scheme
http2scheme install
```

`install` starts the server as a login agent; `http2scheme status` shows three ✓ when all is well and `http2scheme uninstall` removes the agent.

### Build the disk image

```bash
just dmg
```

It writes `dist/http2scheme.app` and `dist/http2scheme-vX.Y.Z.dmg` with its `.sha256` (needs `brew install create-dmg`). Without a certificate it signs ad-hoc, and macOS asks for the browser permission again after each build. To sign with a stable identity, create the self-signed certificate once (macOS asks for your password) and name it:

```bash
just signing-identity
```

```bash
export HTTP2SCHEME_SIGNING_IDENTITY=http2scheme
```

A release needs the certificate. `just icons` redraws the icon and the disk image background from `app/packaging/draw-artwork.swift`.

## Run (dev)

```bash
cargo run --manifest-path app/app/Cargo.toml -p http2scheme -- serve
```

## Run (production)

The app from § Install runs the server inside itself. Without the app, the login agent runs
`http2scheme serve`; control it with `http2scheme start`, `stop` and `status`. Every request is written to `~/Library/Logs/http2scheme/requests.log`.

Settings (`settings.toml`): `port` (default 7777), `allowed_schemes` (default `idea`, `goland`; `http`,
`https` and `file` are never opened) and `close_tab` (default `true`). Quit and reopen the app (or restart the agent) after an edit.

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
