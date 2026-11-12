@link_sharing
Feature: Link sharing
  Turn a custom URL scheme link into the keyed http link the local server opens,
  with one command and without reading the key file by hand.

  Rule: The link command prints the open link for a scheme link

    @e2e
    Scenario: A scheme link becomes an open link
      Given the settings hold a free port and the IntelliJ scheme on the allowed list
      When the user asks for the open link of an IntelliJ link to a file at line 42
      Then the command prints one open link to the loopback address on that port
      And following that link opens exactly that IntelliJ link

    @e2e
    Scenario: A path with spaces survives the round trip
      Given the settings hold a free port and the IntelliJ scheme on the allowed list
      When the user asks for the open link of an IntelliJ link whose file path holds spaces written as %20 and non-Latin letters
      Then the printed link holds only printable ASCII characters
      And following that link opens exactly that IntelliJ link

    @e2e
    Scenario: The first link command creates the key
      Given no settings exist yet
      When the user asks for the open link of an IntelliJ link to a file at line 42
      Then the settings and a new key are created
      And the printed link carries that key

  Rule: The link command refuses links the server would refuse

    @e2e
    Scenario: A scheme not on the list prints the reason
      Given the settings hold a free port and the IntelliJ scheme on the allowed list
      When the user asks for the open link of a Slack link
      Then the command prints no link
      And it says the Slack scheme is not allowed and how to allow it
      And it ends with a failure status
