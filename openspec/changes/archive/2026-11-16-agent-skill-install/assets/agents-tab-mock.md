# Agents tab mock (agreed in chat, 2026-11-16)

Window `http2scheme Settings`, tab bar `General | History | Agents`, Agents selected. Three rows, each a name, a grey line under it and a button on the right, the same row style as the General tab:

| Row | Name | Line under it | Button |
|---|---|---|---|
| 1 | Claude Code folder | the folder, home shown as `~` (default `~/.claude`) | Choose… |
| 2 | Skill · `<state>` (`Not installed` grey, `Installed` green, `An older version is installed` orange) | the skill file's path; when `skills` is a link: `Lands in <real path> (skills is a link)`; after a failed write: the reason, red | Install, or Update (default button, blue) when an older version is installed |
| 3 | Skill text | `Paste it into any agent's skill folder.`; after a click: `Copied to the clipboard.` green | Copy skill text |

Light and dark follow the system appearance (AppKit system colors).
