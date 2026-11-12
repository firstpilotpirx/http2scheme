## Purpose

Keep the local server's settings and secret key in one folder of the user's home, create them safely on first use, and let the user change the port and the allowed schemes by editing one file.

## ADDED Requirements

### Requirement: First use creates the settings and a secret key

When the settings folder, the settings file or the key file is missing, the first command that needs them SHALL create what is missing: the settings file with port 7777, the allowed schemes `idea` and `goland` and tab closing on, and a key of 256 random bits written as 64 lower-case hexadecimal characters. The key file SHALL be readable and writable only by its owner. An existing file SHALL never be overwritten.

#### Scenario: A fresh home gets default settings and a key

- **GIVEN** no settings exist
- **WHEN** the server starts
- **THEN** the settings hold port 7777, the allowed schemes IntelliJ and GoLand, and tab closing on
- **AND** a key of 64 lower-case hexadecimal characters exists that only the user can read

#### Scenario: Two first uses create two different keys

- **GIVEN** two homes with no settings
- **WHEN** the server starts once in each
- **THEN** the two keys differ

#### Scenario: Existing settings and key are kept

- **GIVEN** the settings hold port 9000 and a key exists
- **WHEN** the server starts
- **THEN** the settings still hold port 9000
- **AND** the key is unchanged

### Requirement: The user changes the port, the allowed schemes and tab closing in the settings file

The server and the link command SHALL read the port, the allowed schemes and the tab closing switch from the settings file at start. A settings file that cannot be read, holds an unknown field, a port outside 1024–65535, or a scheme that is not a valid scheme name SHALL stop the command with a one-line reason naming the file and the field. A key file whose content is not 64 lower-case hexadecimal characters, or that others can read, SHALL stop the command the same way.

#### Scenario: An allowed scheme added by hand is honored

- **GIVEN** the user added the Slack scheme to the allowed list in the settings
- **WHEN** the server starts and the browser follows a keyed open link to a Slack link
- **THEN** the Mac opens that Slack link

#### Scenario: A broken settings file stops the start with its reason

- **GIVEN** the settings hold port 80
- **WHEN** the server starts
- **THEN** it does not start
- **AND** it says the port in the settings file must be between 1024 and 65535

#### Scenario: A key file readable by others stops the start

- **GIVEN** the key file is readable by other users
- **WHEN** the server starts
- **THEN** it does not start
- **AND** it says the key file must be readable only by its owner
