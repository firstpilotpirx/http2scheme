## MODIFIED Requirements

### Requirement: The Settings window shows the switches and the history

Settings… SHALL bring the window `http2scheme Settings` to the front with its General tab; the window SHALL have the tabs General, History and Agents. General SHALL show the Serve links switch with the status line under it, and the Start at login switch with the line `The app starts when you log in to the Mac.` under it, or the reason it waits or is disabled. The window SHALL read the state again each time it is shown and after each switch.

#### Scenario: Settings opens the General tab

- **GIVEN** the app runs and its server listens on port 7777
- **WHEN** the user chooses Settings…
- **THEN** the Settings window shows its General tab
- **AND** the Serve links switch is on, with the line under it saying the server runs on port 7777

#### Scenario: Link History opens the History tab

- **GIVEN** the app runs
- **WHEN** the user chooses Link History…
- **THEN** the Settings window shows its History tab

## ADDED Requirements

### Requirement: The Agents tab shows the Claude Code folder and the skill state

The Agents tab SHALL show the Claude Code folder (the one last installed to, else `.claude` in the home folder), the file the skill lands in — the real place when the skills folder is a link — and the skill state: `Not installed`, `Installed` or `An older version is installed`. Choose… SHALL let the user pick another folder; the tab SHALL then show that folder and its state.

#### Scenario: A fresh Mac shows the default folder and no skill

- **GIVEN** the user has never installed the skill
- **WHEN** the user opens the Agents tab
- **THEN** it shows the default Claude Code folder
- **AND** it says the skill is not installed

#### Scenario: Choosing a work folder shows its state

- **GIVEN** the Agents tab shows the default folder
- **WHEN** the user chooses a work Claude Code folder that holds the current skill
- **THEN** the tab shows the work folder
- **AND** it says the skill is installed

#### Scenario: A linked skills folder shows where the file lands

- **GIVEN** the Claude Code folder's skills folder is a link to a folder elsewhere
- **WHEN** the user opens the Agents tab
- **THEN** the tab shows the skill file's place inside the folder the link points to

### Requirement: The Agents tab installs the skill

Install SHALL write the skill into the shown Claude Code folder as the skill install does, remember that folder, and show the new state. The button SHALL read `Update` when an older version is installed. A failed write SHALL show its reason in the tab and change nothing else.

#### Scenario: Install puts the skill into the shown folder

- **GIVEN** the Agents tab shows a folder without the skill
- **WHEN** the user clicks Install
- **THEN** the skill file appears in that folder's skills
- **AND** the tab says the skill is installed

#### Scenario: Update replaces an older skill

- **GIVEN** the shown folder holds an older version of the skill
- **WHEN** the user clicks Update
- **THEN** the skill file holds the current text
- **AND** the tab says the skill is installed

### Requirement: The Agents tab copies the skill text

Copy skill text SHALL put the full skill text, the same the skill command prints, on the clipboard and say so in the tab, so the user can install it in any agent by hand.

#### Scenario: Copy puts the skill on the clipboard

- **GIVEN** the Agents tab is shown
- **WHEN** the user clicks Copy skill text
- **THEN** the clipboard holds the skill text
- **AND** the tab says the text was copied

### Requirement: The app keeps the installed skill current

At launch the app SHALL rewrite the skill in the folder it last installed to when the text there differs from its own; it SHALL do nothing when it never installed the skill or the file is gone.

#### Scenario: A new app version refreshes the installed skill

- **GIVEN** the user installed the skill from the app into a folder and that folder holds an older text
- **WHEN** the app launches
- **THEN** that folder's skill holds the app's text

#### Scenario: A removed skill stays removed

- **GIVEN** the user installed the skill from the app and later deleted the skill file
- **WHEN** the app launches
- **THEN** no skill file is written
