# link-opening Specification

## Purpose
Open a custom URL scheme link on the Mac when an app that allows only http links follows a keyed link to the local server, and refuse every request that could let another website or a crafted link start an app.

## Requirements

### Requirement: An allowed scheme link opens from a keyed request

The server SHALL listen only on the loopback address. A GET request to the open address that carries the secret key and a scheme link whose scheme is on the allowed list SHALL hand that scheme link, unchanged, to the macOS link opener as one argument, with no shell, and answer 200 with a short "Opened" page.

#### Scenario: A keyed link with an allowed scheme opens the app

- **GIVEN** the server runs with the IntelliJ scheme on the allowed list
- **WHEN** the browser follows an open link with the right key and an IntelliJ link to a file at line 17
- **THEN** the Mac opens exactly that IntelliJ link
- **AND** the browser shows the "Opened" page

#### Scenario: Special characters in the link reach the app unchanged

- **GIVEN** the server runs with the IntelliJ scheme on the allowed list
- **WHEN** the browser follows an open link whose IntelliJ link names a file path with spaces written as `%20`, quotes and a semicolon
- **THEN** the Mac opens the IntelliJ link with exactly those characters, as one link
- **AND** no command other than the link opener runs

#### Scenario: The server cannot be reached from another computer

- **GIVEN** the server runs
- **WHEN** a request comes to the Mac's network address instead of the loopback address
- **THEN** the connection is refused

### Requirement: The browser tab of an opened link closes by itself

After a link opens, the server SHALL ask every running supported browser (Safari, Google Chrome, Arc, Brave, Microsoft Edge) to close its tabs that show the server's open address, unless the settings turn tab closing off. A refused or failed request SHALL keep its tab, so the user can read why. A failed tab close SHALL not change the answer or the log.

#### Scenario: The tab of an opened link closes

- **GIVEN** the server runs with tab closing on
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the Mac opens that IntelliJ link
- **AND** the server asks the browsers to close the tabs that show its open address

#### Scenario: A refused request keeps its tab

- **GIVEN** the server runs with tab closing on
- **WHEN** the browser follows an open link with a wrong key
- **THEN** the server asks no browser to close a tab

#### Scenario: Tab closing turned off in the settings

- **GIVEN** the settings turn tab closing off
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the Mac opens that IntelliJ link
- **AND** the server asks no browser to close a tab

#### Scenario: A failed tab close keeps the link opened

- **GIVEN** the server runs with tab closing on and the browser refuses to close tabs
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the browser shows the "Opened" page
- **AND** the request log ends with the result "opened"

### Requirement: The app the user left for the browser comes back behind the opened app

Before an allowed link opens, when a supported browser is the front app, the server SHALL bring to the front the app that was in front before that browser, so that the app switcher (Command-Tab) lists the link's app first and the left app second, before the browser. When no supported browser is in front, or no other app is behind it, the server SHALL change no app's order. A failed reorder SHALL not change the answer or the log.

#### Scenario: The left app comes back behind the opened app

- **GIVEN** the user left the Claude app for the browser
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the server brings the Claude app to the front
- **AND** then the Mac opens that IntelliJ link

#### Scenario: A refused request changes no app order

- **GIVEN** the user left the Claude app for the browser
- **WHEN** the browser follows an open link with a wrong key
- **THEN** the server brings no app to the front

#### Scenario: A failed reorder keeps the link opened

- **GIVEN** the app order cannot be changed
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the browser shows the "Opened" page
- **AND** the request log ends with the result "opened"

### Requirement: A request without the right key opens nothing

A request whose key is missing or differs from the secret key SHALL open nothing and get 403 with a short "Wrong key" page. The key comparison SHALL take the same time wherever the keys first differ.

#### Scenario: A request with a wrong key is refused

- **GIVEN** the server runs
- **WHEN** the browser follows an open link with a wrong key
- **THEN** nothing opens
- **AND** the browser shows the "Wrong key" page

#### Scenario: A request with no key is refused

- **GIVEN** the server runs
- **WHEN** the browser follows an open link without a key
- **THEN** nothing opens
- **AND** the browser shows the "Wrong key" page

### Requirement: Only well-formed links with an allowed scheme open

A keyed request SHALL open nothing and get 400 with a short page naming the reason when its scheme link is missing, longer than 4096 characters, holds a control character or a space, has no scheme, uses a scheme not on the allowed list, or uses `http`, `https` or `file`, which are never allowed.

#### Scenario: A scheme not on the list is refused

- **GIVEN** the server runs with only the IntelliJ scheme on the allowed list
- **WHEN** the browser follows a keyed open link to a Slack link
- **THEN** nothing opens
- **AND** the browser shows a page saying the Slack scheme is not allowed

#### Scenario Outline: A malformed scheme link is refused

- **GIVEN** the server runs with the IntelliJ scheme on the allowed list
- **WHEN** the browser follows a keyed open link whose scheme link is <bad link>
- **THEN** nothing opens
- **AND** the browser shows a page saying the link is malformed

Examples: missing; empty; without a scheme; with a line break inside; with a raw space; longer than 4096 characters.

#### Scenario: Web and file links are never opened

- **GIVEN** the settings list `https` and `file` as allowed schemes
- **WHEN** the browser follows a keyed open link to a web page or a local file
- **THEN** nothing opens
- **AND** the browser shows a page saying the scheme is not allowed

### Requirement: Other requests get a clear answer

A request to any address other than the open address and the health address SHALL get 404. A request with a method other than GET SHALL get 405. A request whose host is not the loopback address or localhost with the server's port SHALL get 421 and open nothing. The health address SHALL answer 200 without a key and open nothing. When the link opener fails, the server SHALL answer 502 with a page saying the app could not be opened.

#### Scenario: A request under a foreign host name is refused

- **GIVEN** the server runs
- **WHEN** a keyed open link arrives under a host name other than the loopback address or localhost
- **THEN** nothing opens
- **AND** the server answers that the host is not served

#### Scenario: An unknown address is not found

- **GIVEN** the server runs
- **WHEN** the browser asks for an address the server does not serve
- **THEN** the browser shows a "Not found" page

#### Scenario: The health address answers without a key

- **GIVEN** the server runs
- **WHEN** a client asks the health address
- **THEN** the server answers that it is running
- **AND** nothing opens

#### Scenario: A failed open is reported

- **GIVEN** the server runs and the Mac's link opener fails for the IntelliJ link
- **WHEN** the browser follows a keyed open link to that IntelliJ link
- **THEN** the browser shows a page saying the app could not be opened

### Requirement: Every open request is logged without the key

Every request to the open address SHALL append one line to the request log: the time, the result and the scheme link (or the reason it was unreadable), in that order. The key SHALL never appear in the log.

#### Scenario: An opened link is logged

- **GIVEN** the server runs with the IntelliJ scheme on the allowed list
- **WHEN** the browser follows a keyed open link to an IntelliJ link
- **THEN** the request log ends with a line holding the time, the result "opened" and that IntelliJ link

#### Scenario: A refused request is logged without the key

- **GIVEN** the server runs
- **WHEN** the browser follows an open link with a wrong key
- **THEN** the request log ends with a line holding the result "wrong key"
- **AND** neither the secret key nor the wrong key appears in the log
