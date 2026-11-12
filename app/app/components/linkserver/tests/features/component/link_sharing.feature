@link_sharing
Feature: Link sharing through the server
  A link the link command prints opens its scheme link when the browser follows it.

  Rule: The link command prints the open link for a scheme link

    @component
    Scenario: A printed link opens through the server
      Given the settings hold a free port and the IntelliJ scheme on the allowed list
      When the user asks for the open link of an IntelliJ link whose file path holds spaces and non-Latin letters
      And the browser follows the printed link through the server
      Then the Mac opens exactly that IntelliJ link
