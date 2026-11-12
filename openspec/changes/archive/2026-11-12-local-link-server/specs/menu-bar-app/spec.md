## Purpose

Run the local server as an ordinary Mac app: it lives in the menu bar, serves links while it runs, starts at login, and shows its state without a terminal.

## ADDED Requirements

### Requirement: The app serves links from the menu bar

Started from its app bundle with no command, the program SHALL run as a menu bar app with no Dock icon: it SHALL run the server of the `link-opening` capability inside itself, with the same settings and key, and put one status item in the menu bar. With any command it SHALL behave as the command line tool.

#### Scenario: The app serves a keyed link

- **GIVEN** the app is started from its bundle with no command
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the Mac opens that IntelliJ link

#### Scenario: The bundled program is also the command line tool

- **GIVEN** the program inside the app bundle
- **WHEN** the user asks it for the open link of an IntelliJ link
- **THEN** it prints one open link to the loopback address

### Requirement: The menu shows the app and the server state

The status item's menu SHALL show, in this order: the app name with its version; one status line; Serve Links; Link History…; Start at Login; Open Settings Folder; Open Logs Folder; Quit http2scheme. The status line SHALL read `Starting…` until the server listens, `Running on <address>` while it listens, `Stopped: <reason>` after the server failed to start or stopped, with the reason the `serve` command would print, and `Off` while the user turned the server off.

#### Scenario: A running server

- **GIVEN** the server listens on port 7777
- **WHEN** the user opens the menu
- **THEN** the menu shows the app name with its version
- **AND** the status line says the server runs on the loopback address with port 7777

#### Scenario: A taken port

- **GIVEN** another program holds the configured port 7777
- **WHEN** the user opens the menu
- **THEN** the status line reads "Stopped: port 7777 is taken"

### Requirement: Serve Links turns the server off and on

Serve Links SHALL show a check mark exactly while the server is not turned off. Choosing it while the server is on SHALL stop the server, so the open address no longer answers, and the status line SHALL read `Off`. Choosing it while the server is off SHALL start the server again with the settings and key as they are then. The app SHALL start with the server on at every launch.

#### Scenario: Turning the server off

- **GIVEN** the app runs and its server answers
- **WHEN** the user chooses Serve Links
- **THEN** the server no longer answers
- **AND** the status line reads "Off"
- **AND** Serve Links shows no check mark

#### Scenario: Turning the server on again

- **GIVEN** the user turned the server off
- **WHEN** the user chooses Serve Links
- **THEN** the server answers again
- **AND** Serve Links shows a check mark

### Requirement: The Link History window lists the latest requests

Link History… SHALL open the Link History window, or bring it to the front. The window SHALL list the latest 100 lines of the request log, newest first, each row showing the local time, the result and the scheme link, and SHALL read the log again each time it comes to the front. A log line it cannot read SHALL have no row. With no line in the log it SHALL say `No link has been opened yet.`; with a log it cannot read it SHALL list no row and say `The link history could not be read: <reason>`.

#### Scenario: Requests are listed newest first

- **GIVEN** the request log holds an opened IntelliJ link and, after it, a GoLand link with the wrong key
- **WHEN** the user opens the Link History window
- **THEN** the first row shows the GoLand link with the result "wrong key"
- **AND** the second row shows the IntelliJ link with the result "opened"

#### Scenario: Only the latest 100 requests are listed

- **GIVEN** the request log holds 105 requests
- **WHEN** the user opens the Link History window
- **THEN** it lists 100 rows, the latest 100

#### Scenario: An unreadable log line has no row

- **GIVEN** the request log holds two requests and a damaged line between them
- **WHEN** the user opens the Link History window
- **THEN** it lists the two requests

#### Scenario: No request yet

- **GIVEN** the request log does not exist
- **WHEN** the user opens the Link History window
- **THEN** the window says no link has been opened yet

### Requirement: A row's link opens again

Each row of the Link History window SHALL have a ▶ button that opens the row's scheme link again, as the server opens a keyed request: the link SHALL be checked against the allowed schemes of the settings, opened with the macOS opener, and the attempt SHALL be appended to the request log with its result. It SHALL need no key and SHALL close no browser tab. The window SHALL then list the new request as its first row.

#### Scenario: Opening an IntelliJ link again

- **GIVEN** the Link History window lists an opened IntelliJ link
- **WHEN** the user chooses ▶ on its row
- **THEN** the Mac opens that IntelliJ link
- **AND** the first row shows that IntelliJ link with the result "opened"

#### Scenario: A link whose scheme is not allowed stays closed

- **GIVEN** the Link History window lists a Slack link with the result "scheme not allowed"
- **WHEN** the user chooses ▶ on its row
- **THEN** the Mac opens nothing
- **AND** the first row shows that Slack link with the result "scheme not allowed"

### Requirement: Start at Login follows the system's login items

Start at Login SHALL show a check mark exactly while macOS lists the app in its login items as enabled. Choosing it SHALL add the app to the login items when it is not there and remove it when it is. When macOS waits for the user's approval, the item SHALL read `Start at Login (approve in System Settings)` and choosing it SHALL open the Login Items pane of System Settings. Outside an installed app bundle the item SHALL be disabled.

#### Scenario: Turning Start at Login on

- **GIVEN** the app is not in the login items
- **WHEN** the user chooses Start at Login
- **THEN** the app is in the login items
- **AND** Start at Login shows a check mark

#### Scenario: Turning Start at Login off

- **GIVEN** the app is in the login items
- **WHEN** the user chooses Start at Login
- **THEN** the app is not in the login items
- **AND** Start at Login shows no check mark

#### Scenario: A login item that waits for approval

- **GIVEN** macOS waits for the user's approval of the app's login item
- **WHEN** the user chooses Start at Login
- **THEN** the Login Items pane of System Settings opens

#### Scenario: Not an installed app

- **GIVEN** the program does not run from an app bundle
- **WHEN** the user opens the menu
- **THEN** Start at Login is disabled

### Requirement: The first launch from Applications starts the app at login

The first time the app runs from the `/Applications` folder, it SHALL add itself to the login items, once: it SHALL record that it did so in the settings folder, and SHALL never add itself again after the user removed it. A launch from any other folder SHALL change no login item.

#### Scenario: The first launch from Applications

- **GIVEN** the app runs from the Applications folder for the first time
- **WHEN** the app starts
- **THEN** the app is in the login items

#### Scenario: A login item the user removed stays removed

- **GIVEN** the app added itself to the login items before and the user removed it
- **WHEN** the app starts again
- **THEN** the app is not in the login items

#### Scenario: A launch from another folder

- **GIVEN** the app runs from a folder other than Applications for the first time
- **WHEN** the app starts
- **THEN** the login items are unchanged

### Requirement: The menu opens the app's folders and quits

Open Settings Folder SHALL open the settings folder in the Finder, and Open Logs Folder the folder of the request log, creating the folder first when it does not exist. Quit http2scheme SHALL stop the server and end the app; so SHALL a quit from the system (logout, shutdown, a terminate signal).

#### Scenario: Open the settings folder

- **GIVEN** the app runs
- **WHEN** the user chooses Open Settings Folder
- **THEN** the Finder opens the settings folder

#### Scenario: Open the logs folder before the first request

- **GIVEN** the app runs and no request was logged yet
- **WHEN** the user chooses Open Logs Folder
- **THEN** the logs folder exists
- **AND** the Finder opens it

#### Scenario: A terminate signal ends the app

- **GIVEN** the app runs and its server answers
- **WHEN** the app receives a terminate signal
- **THEN** the app ends
- **AND** the server no longer answers
