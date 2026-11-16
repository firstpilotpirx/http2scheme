# link-sharing Specification

## Purpose
Let the agent or the user turn a custom URL scheme link into the keyed http link the local server opens, with one command and without reading the key file by hand.

## Requirements

### Requirement: The link command prints the open link for a scheme link

The link command SHALL take one or more scheme links, check each with the same rules the server applies to an open request, and print one line per scheme link, in the order given: the http link to the open address on the loopback address and the configured port, carrying the secret key and the scheme link percent-encoded. It SHALL create the settings and the key first when they do not exist yet.

#### Scenario: A scheme link becomes an open link

- **GIVEN** the settings hold port 7777 and the IntelliJ scheme on the allowed list
- **WHEN** the user asks for the open link of an IntelliJ link to a file at line 42
- **THEN** the command prints one open link to the loopback address on port 7777
- **AND** following that link opens exactly that IntelliJ link

#### Scenario: Several scheme links become open links in order

- **GIVEN** the settings hold the IntelliJ and GoLand schemes on the allowed list
- **WHEN** the user asks in one call for the open links of an IntelliJ link, a GoLand link and a second IntelliJ link
- **THEN** the command prints three open links, one per line
- **AND** following the second line opens exactly the GoLand link

#### Scenario: A path with spaces survives the round trip

- **GIVEN** the settings hold the IntelliJ scheme on the allowed list
- **WHEN** the user asks for the open link of an IntelliJ link whose file path holds spaces written as `%20` and non-Latin letters
- **THEN** the printed link holds only printable ASCII characters
- **AND** following it opens the IntelliJ link with the same path

#### Scenario: The first link command creates the key

- **GIVEN** no settings exist yet
- **WHEN** the user asks for the open link of an IntelliJ link
- **THEN** the settings and a new key are created
- **AND** the printed link carries that key

### Requirement: The link command refuses links the server would refuse

The link command SHALL print no link and exit with a non-zero status when any of the scheme links it got is malformed or has a scheme not on the allowed list; it SHALL print a one-line reason for each such link.

#### Scenario: A scheme not on the list prints the reason

- **GIVEN** the settings hold only the IntelliJ scheme on the allowed list
- **WHEN** the user asks for the open link of a Slack link
- **THEN** the command prints no link
- **AND** it says the Slack scheme is not allowed and how to allow it
- **AND** it ends with a failure status

#### Scenario: One refused link among several prints no link

- **GIVEN** the settings hold only the IntelliJ scheme on the allowed list
- **WHEN** the user asks in one call for the open links of an IntelliJ link and a Slack link
- **THEN** the command prints no link
- **AND** it says the Slack scheme is not allowed
- **AND** it ends with a failure status
