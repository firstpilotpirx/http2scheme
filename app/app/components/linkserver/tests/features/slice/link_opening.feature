@link_opening
Feature: Link opening
  Open a custom URL scheme link on the Mac when an app that allows only http links
  follows a keyed link to the local server, and refuse every other request.

  Rule: An allowed scheme link opens from a keyed request

    @slice
    Scenario: A keyed link with an allowed scheme opens the app
      Given the server runs with the IntelliJ scheme on the allowed list
      When the browser follows an open link with the right key and an IntelliJ link to a file at line 17
      Then the Mac opens exactly that IntelliJ link
      And the browser shows the "Opened" page

    @slice
    Scenario: Special characters in the link reach the app unchanged
      Given the server runs with the IntelliJ scheme on the allowed list
      When the browser follows an open link whose IntelliJ link names a file path with spaces written as %20, quotes and a semicolon
      Then the Mac opens the IntelliJ link with exactly those characters, as one link
      And no command other than the link opener runs

  Rule: The browser tab of an opened link closes by itself

    @slice
    Scenario: The tab of an opened link closes
      Given the server runs with tab closing on
      When the browser follows a keyed open link to an IntelliJ link
      Then the Mac opens that IntelliJ link
      And the server asks the browsers to close the tabs that show its open address

    @slice
    Scenario: A refused request keeps its tab
      Given the server runs with tab closing on
      When the browser follows an open link with a wrong key
      Then the server asks no browser to close a tab

    @slice
    Scenario: Tab closing turned off in the settings
      Given the settings turn tab closing off
      When the browser follows a keyed open link to an IntelliJ link
      Then the Mac opens that IntelliJ link
      And the server asks no browser to close a tab

    @slice
    Scenario: A failed tab close keeps the link opened
      Given the server runs with tab closing on and the browser refuses to close tabs
      When the browser follows a keyed open link to an IntelliJ link
      Then the browser shows the "Opened" page
      And the request log ends with the result "opened"

  Rule: A request without the right key opens nothing

    @slice
    Scenario: A request with a wrong key is refused
      Given the server runs
      When the browser follows an open link with a wrong key
      Then nothing opens
      And the browser shows the "Wrong key" page

    @slice
    Scenario: A request with no key is refused
      Given the server runs
      When the browser follows an open link without a key
      Then nothing opens
      And the browser shows the "Wrong key" page

  Rule: Only well-formed links with an allowed scheme open

    @slice
    Scenario: A scheme not on the list is refused
      Given the server runs with only the IntelliJ scheme on the allowed list
      When the browser follows a keyed open link to a Slack link
      Then nothing opens
      And the browser shows a page saying the Slack scheme is not allowed

    @slice
    Scenario Outline: A malformed scheme link is refused
      Given the server runs with the IntelliJ scheme on the allowed list
      When the browser follows a keyed open link whose scheme link is <bad link>
      Then nothing opens
      And the browser shows a page saying the link is malformed

      Examples:
        | bad link                         |
        | missing                          |
        | empty                            |
        | without a scheme                 |
        | with a line break inside         |
        | with a raw space                 |
        | longer than 4096 characters      |

    @slice
    Scenario Outline: Web and file links are never opened
      Given the settings list https and file as allowed schemes
      When the browser follows a keyed open link to <target>
      Then nothing opens
      And the browser shows a page saying the scheme is not allowed

      Examples:
        | target       |
        | a web page   |
        | a local file |

  Rule: Other requests get a clear answer

    @slice
    Scenario: A request under a foreign host name is refused
      Given the server runs
      When a keyed open link arrives under a host name other than the loopback address or localhost
      Then nothing opens
      And the server answers that the host is not served

    @slice
    Scenario: An unknown address is not found
      Given the server runs
      When the browser asks for an address the server does not serve
      Then the browser shows the "Not found" page

    @slice
    Scenario: The health address answers without a key
      Given the server runs
      When a client asks the health address
      Then the server answers that it is running
      And nothing opens

    @slice
    Scenario: A failed open is reported
      Given the server runs and the Mac's link opener fails for the IntelliJ link
      When the browser follows a keyed open link to that IntelliJ link
      Then the browser shows a page saying the app could not be opened

  Rule: Every open request is logged without the key

    @slice
    Scenario: An opened link is logged
      Given the server runs with the IntelliJ scheme on the allowed list
      When the browser follows a keyed open link to an IntelliJ link
      Then the request log ends with a line holding the time, the result "opened" and that IntelliJ link

    @slice
    Scenario: A refused request is logged without the key
      Given the server runs
      When the browser follows an open link with a wrong key
      Then the request log ends with a line holding the result "wrong key"
      And neither the secret key nor the wrong key appears in the log
