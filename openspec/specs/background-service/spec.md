# background-service Specification

## Purpose
Keep the local server running without the user's attention: start it at login, restart it after a crash, and give simple commands to install, remove, start, stop and check it.

## Requirements

### Requirement: Install makes the server start at login and restart after a crash

The install command SHALL write a per-user LaunchAgent definition that runs the installed program's serve command at login and again whenever it exits, with its output going to the log folder, then load it so the server runs at once. Running install again SHALL replace the definition and reload it.

#### Scenario: Install starts the server and keeps it at login

- **GIVEN** the agent is not installed
- **WHEN** the user installs the agent
- **THEN** the agent definition exists and asks to run at login and to restart after an exit
- **AND** the agent is loaded
- **AND** the command says the agent is installed and running

#### Scenario: A second install replaces the first

- **GIVEN** the agent is installed
- **WHEN** the user installs the agent again
- **THEN** one agent definition exists, pointing at the current program
- **AND** the agent is loaded

### Requirement: Uninstall removes the agent

The uninstall command SHALL unload the agent when it is loaded and delete its definition. Uninstalling an agent that is not installed SHALL succeed and say so. The settings, the key and the logs SHALL stay.

#### Scenario: Uninstall stops and removes the agent

- **GIVEN** the agent is installed and loaded
- **WHEN** the user uninstalls the agent
- **THEN** the agent is unloaded and its definition is gone
- **AND** the settings and the key remain

#### Scenario: Uninstall with nothing installed is harmless

- **GIVEN** the agent is not installed
- **WHEN** the user uninstalls the agent
- **THEN** the command succeeds and says nothing was installed

### Requirement: Start and stop control an installed agent

The start command SHALL load an installed agent that is not loaded; the stop command SHALL unload a loaded agent until the next start or login. Both SHALL refuse with a hint to install when the agent is not installed.

#### Scenario: Stop keeps the definition

- **GIVEN** the agent is installed and loaded
- **WHEN** the user stops the agent
- **THEN** the agent is unloaded and its definition remains

#### Scenario: Start loads a stopped agent

- **GIVEN** the agent is installed but not loaded and the server does not answer
- **WHEN** the user starts the agent
- **THEN** the agent is loaded

#### Scenario: Start without install gives a hint

- **GIVEN** the agent is not installed
- **WHEN** the user starts the agent
- **THEN** the command fails and says to install the agent first

### Requirement: Status reports the agent and the server

The status command SHALL print whether the agent is installed, whether it is loaded and whether the server answers its health address on the configured port, one line each, and exit with a non-zero status unless all three hold.

#### Scenario: A healthy setup

- **GIVEN** the agent is installed and loaded and the server answers
- **WHEN** the user asks for the status
- **THEN** the command shows installed, loaded and answering
- **AND** it ends with a success status

#### Scenario: A stopped server

- **GIVEN** the agent is installed but not loaded and the server does not answer
- **WHEN** the user asks for the status
- **THEN** the command shows installed, not loaded and not answering
- **AND** it ends with a failure status

### Requirement: Serve runs the server in the foreground

The serve command SHALL prepare the settings and key, start the server on the loopback address and the configured port, print the address, and run until it is stopped. It SHALL fail with a one-line reason when the port is taken.

#### Scenario: Serve on a free port

- **GIVEN** the configured port is free
- **WHEN** the user runs the server in the foreground
- **THEN** it prints the loopback address with the port
- **AND** the health address answers

#### Scenario: Serve on a taken port

- **GIVEN** another program holds the configured port
- **WHEN** the user runs the server in the foreground
- **THEN** it fails and says the port is taken
