## Purpose

Teach coding agents to write a clickable IDE link next to every place in code they mention, by giving them a skill the program prints, installs into a Claude Code folder and keeps current.

## ADDED Requirements

### Requirement: The skill tells the agent how to write IDE links

The skill text SHALL be in the open agent skill format (a header naming `http2scheme`, then instructions) and SHALL name the full path of the program that printed it. It SHALL tell the agent to write each place in code as a short file link followed by an `IDE` link, made for a whole reply in one link call, with `goland` for Go projects and `idea` otherwise, and to write only the file link plus one fix line when the call fails.

#### Scenario: The printed skill names the program and the link style

- **GIVEN** the program is installed at a known path
- **WHEN** the user asks the program for the skill text
- **THEN** the text starts with a header naming the skill `http2scheme` and saying when to use it
- **AND** it names that program path for the link command
- **AND** its example shows a file link followed by a link labelled `IDE`

### Requirement: The skill installs into a Claude Code folder

Installing the skill SHALL write its text to the file `SKILL.md` in the folder `skills/http2scheme` of the chosen Claude Code folder, creating the folders it needs, replacing an older text, and leaving every other file there alone. The default Claude Code folder SHALL be `.claude` in the user's home folder. The command SHALL print the file it wrote; when the `skills` folder is a link to another folder, it SHALL also print the real folder the file landed in.

#### Scenario: Install writes the skill into the default folder

- **GIVEN** the user's Claude Code folder has no skill of this program
- **WHEN** the user installs the skill without naming a folder
- **THEN** the skill file appears in the default Claude Code folder's skills
- **AND** the command prints the path of that file and succeeds

#### Scenario: Install into a chosen work profile folder

- **GIVEN** the user keeps a second Claude Code folder for work
- **WHEN** the user installs the skill naming that folder
- **THEN** the skill file appears in that folder's skills
- **AND** the default Claude Code folder stays untouched

#### Scenario: Install over an older skill replaces it

- **GIVEN** the Claude Code folder holds an older text of the skill and another skill beside it
- **WHEN** the user installs the skill
- **THEN** the skill file holds the current text
- **AND** the other skill is unchanged

#### Scenario: A linked skills folder is named in the output

- **GIVEN** the Claude Code folder's skills folder is a link to a folder elsewhere
- **WHEN** the user installs the skill
- **THEN** the skill file appears in the folder the link points to
- **AND** the command also prints that real folder
