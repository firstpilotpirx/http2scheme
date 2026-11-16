---
name: http2scheme
description: Write a clickable IDE link next to every file and line you mention in a reply. Use whenever a reply points to a place in code (a file, a file and a line, a function you found).
---

# IDE links with http2scheme

The user reads your replies in a chat that opens `http://` links but not IDE links (`idea://`, `goland://`). The program `{{program}}` turns an IDE link into a local `http://127.0.0.1:…/open?…` link; a click on it opens the file at that line in the IDE.

## The rule

Every time a reply points to a file and a line, write two links side by side:

1. the short file link, as usual: `[server.rs:42](app/src/server.rs:42)`;
2. right after it, a small link labelled `IDE` to the open link the program printed.

Example:

```markdown
The port is read in [server.rs:42](app/src/server.rs:42) [IDE](http://127.0.0.1:7777/open?key=…&url=idea%3A%2F%2Fopen%3Ffile%3D…)
```

Without a line number, use line 1.

## Making the IDE links

Collect every place of the reply first, then make all their links with one call, one IDE link per argument, each in single quotes:

```bash
'{{program}}' link 'idea://open?file=/abs/path/app/src/server.rs&line=42' 'idea://open?file=/abs/path/app/src/main.rs&line=7'
```

- It prints one open link per line, in the order of the arguments: line *n* belongs to argument *n*.
- The file path is absolute. Write a space in it as `%20`.
- The scheme follows the project's main language: Go → `goland://`; every other language → `idea://`.
- Paste the printed links exactly as printed; never build or change one by hand.

## When it does not work

When the program is missing, the call fails or it prints a reason instead of links, write only the file links and add one line at the end of the reply:

- the program is missing: `IDE links: install http2scheme (see its README).`
- `scheme not allowed: <scheme>`: `IDE links: add "<scheme>" to allowed_schemes in <the settings file it named>.`
- any other reason: `IDE links: <the reason it printed>.`

A link that opens a page saying the server is off needs no fix from you: the user starts the http2scheme app.
