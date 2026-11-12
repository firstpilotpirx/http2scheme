# Spec: IDE link helper

A small local program that lets you open a file in IntelliJ IDEA or GoLand, at the right line, with one click on a link in the Claude chat.

Date: 2026-11-12.

## 1. The problem

### 1.1. The work flow we want

1. You ask Claude a question about the code, for example "where does the token balance come from?".
2. Claude finds the answer in several places: a few files, or a few lines in one file.
3. Claude gives you a list of these places as links.
4. You click the links one by one. Each click opens the file at the right line in your IDE.
5. You read the code, go back to the chat, click the next link, and so on.

You don't want to type "open this file" each time. One mouse click per place is the goal.

### 1.2. Why it doesn't work today

- **JetBrains IDEs have their own links.** IntelliJ handles `idea://open?file=<path>&line=<n>`, and GoLand handles `goland://open?file=<path>&line=<n>`. macOS starts the right IDE for such a link.
- **These links work on your Mac.** We tested this: `open "idea://open?file=…&line=17"` in Terminal opened `TokenController.java` in IntelliJ at line 17.
- **The Claude chat blocks them.** It shows `idea://` links as plain text, not as links. Buttons in chat widgets that use them don't work either.
- **The chat lets through only two kinds of links:**
  - web links (`http://`, `https://`), which open in the default browser;
  - file links, which open in the Claude app's own file viewer, not in IntelliJ.
- **Why the chat blocks them (our guess).** A link with a custom scheme can start any app on the Mac and pass it any data. So, for safety, the app lets through only web and file links. This is a guess from how the app behaves. We found nothing about it in the docs.

### 1.3. The general picture

This is not a JetBrains problem. It's a general one:

- macOS (and Windows, and Linux) lets an app register its own link scheme: `slack://`, `idea://`, `vscode://` and so on.
- Each app decides what goes after `://`. There is no common format for "file + line".
- The Claude chat blocks all such schemes. So no app can be opened from the chat directly.

## 2. Options we looked at

| Option | What it is | Why we did not pick it |
| --- | --- | --- |
| `idea://` links in chat | Direct links to IntelliJ | The chat blocks them |
| Buttons in a chat widget | A widget with buttons that open `idea://` links, or that send Claude a message | The chat blocks both kinds of buttons |
| File links in chat | Links like `File.java:17` | They open the Claude app's own viewer, not IntelliJ |
| The app's "Open in" menu | Claude opens a file in your editor | It supports only VS Code, Cursor, Windsurf and Zed. A request to add JetBrains was closed as "not planned" |
| A page in Safari | Claude makes an HTML page with `idea://` links and opens it | Works, but it's clumsy: you leave the chat for a browser page, and each answer needs a new page |
| IntelliJ's built-in server (`localhost:63342/api/file`) | IntelliJ can open a file from a web link | On current versions it needs the "IDE Remote Control" plugin. Without it, it returns "not found" (we checked) |
| Finicky | A free macOS app set as the "default browser". It turns some web links into `idea://` links and passes the rest to Safari | It needs a change to the default browser, so every link on the Mac goes through it. That feels wrong |
| Ask Claude in words | "Open place 2", and Claude runs `open idea://…` | Works now, but it's one message per place, not one click |
| Ready-made packages (npm) | `launch-editor-middleware` (0.8 million downloads per week, Vite team) and others | Needs Node.js and outside packages. Its main value, support for many editors and Windows, is not needed here |
| **Our own small helper** | **A local web server that turns web links into `idea://` links** | **Picked: small, no outside packages, every line is easy to check** |

### 2.1. Notes on ready-made packages

| Package | Downloads per week | Last release | Verdict |
| --- | --- | --- | --- |
| launch-editor | 29 million | Oct 2026 | Popular, kept up by the Vite team |
| launch-editor-middleware | 0.8 million | Oct 2026 | The web server part of launch-editor |
| open-editor | 0.28 million | Nov 2025 | No web server part |
| express-open-in-editor | about 500 | 2017 | Abandoned |
| open-in-editor-connect | about 20 | 2018 | Abandoned |

`launch-editor` had one serious hole, CVE-2024-52011: a crafted link could run a command. It hit Windows only, and was fixed in version 2.9.0.

## 3. The solution

A small program on the Mac takes `http://127.0.0.1` links (which the chat lets through) and turns them into `idea://` or `goland://` links (which open the IDE).

```
chat link (http) → browser → helper on 127.0.0.1 → open "idea://…" → IntelliJ
```

## 4. How it works

1. The helper waits for requests on `127.0.0.1`, port 7777.
2. Claude writes links like:
   `http://127.0.0.1:7777/open?key=<KEY>&ide=idea&file=/abs/path/File.java&line=42`
3. You click a link. The browser sends the request to the helper.
4. The helper checks the request (see "Safety").
5. The helper runs `open "idea://open?file=<path>&line=<n>"`.
6. The IDE opens the file at that line.
7. The browser tab shows "Opened" and tries to close itself.

## 5. Link fields

| Field | Required | Rule |
| --- | --- | --- |
| `key` | yes | must match the secret key |
| `ide` | no, `idea` by default | only `idea` or `goland` |
| `file` | yes | full path to a file that exists |
| `line` | no | a positive whole number |

Answers:

| Case | Code |
| --- | --- |
| All good, file opened | 200, a short "Opened" page |
| No key or a wrong key | 403 |
| Wrong IDE, missing file or bad line | 400 |
| Any other address | 404 |

## 6. Safety

**The main risk.** Any website open in your browser can also send a request to `127.0.0.1`. Without protection, it could open files in your IDE without you.

| Rule | Why |
| --- | --- |
| Listen only on `127.0.0.1` | Other computers on the network can't reach the helper |
| A secret key in every link; reject requests without it | A random website doesn't know the key, so it can't use the helper |
| The key is long and random, kept in one local file readable only by you | So that it can't be guessed or read by other users |
| Compare the key in constant time | So that the key can't be found by timing the answers |
| Allow only `idea` and `goland` | The helper can't be used to start any other app |
| Allow only full paths to files that exist | No odd paths, no folders |
| Allow only a number for the line | Nothing extra can be added to the IDE link |
| Encode the file path in the IDE link | Spaces and special signs in a path don't break the link |
| Start `open` without a shell | A crafted link can't run a command (the same kind of hole as CVE-2024-52011) |
| Write each request to a log: time, file, result | You can see what was opened and when |

## 7. Running

1. The helper starts on its own when you log in (a macOS LaunchAgent).
2. It restarts on its own if it crashes.
3. Simple commands to start, stop and check it.
4. If the key file is missing, the helper makes a new key on first start.

## 8. What Claude needs

1. A saved rule: when showing places in the code, write them as helper links.
2. A way to read the key, from the local key file.
3. Link text that is easy to read: `File.java:42` plus a short note on what is there.

## 9. Limits

1. Python 3 that comes with macOS, with no outside packages.
2. macOS only.
3. A browser tab may stay open after a click, if the browser doesn't let the page close itself.
4. The browser may ask once to allow the link.

## 10. Done when

1. A click on a chat link opens the file at the right line in IntelliJ.
2. The same works for GoLand.
3. A link with a wrong key, a missing file or a bad line opens nothing and gets the right code.
4. After a Mac restart, links work with no action from you.
5. Each request is in the log.

## 11. Out of scope

- Windows and Linux.
- Other editors (VS Code and others).
- Any page or interface for the helper.
- Opening folders or projects, only files.

## 12. Sources

- [JetBrains forum: /api/file moved to the IDE Remote Control plugin](https://intellij-support.jetbrains.com/hc/en-us/community/posts/207484305/comments/207769089)
- [Request: JetBrains in the Claude desktop "Open in" menu (closed, not planned)](https://claudeissues.com/issue/83693-feature-desktop-app-open-in-menu-support-jetbrains-ides-rider-custom-editors)
- [Finicky](https://github.com/johnste/finicky)
- [vitejs/launch-editor](https://github.com/vitejs/launch-editor)
- [CVE-2024-52011](https://app.opencve.io/cve/CVE-2024-52011)
