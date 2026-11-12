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

The status item's menu SHALL show, in this order: the app name with its version; one status line; Start at Login; Open Settings Folder; Open Logs Folder; Quit http2scheme. The status line SHALL read `Starting…` until the server listens, `Running on <address>` while it listens, and `Stopped: <reason>` after the server failed to start or stopped, with the reason the `serve` command would print.

#### Scenario: A running server

- **GIVEN** the server listens on port 7777
- **WHEN** the user opens the menu
- **THEN** the menu shows the app name with its version
- **AND** the status line says the server runs on the loopback address with port 7777

#### Scenario: A taken port

- **GIVEN** another program holds the configured port 7777
- **WHEN** the user opens the menu
- **THEN** the status line reads "Stopped: port 7777 is taken"

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
