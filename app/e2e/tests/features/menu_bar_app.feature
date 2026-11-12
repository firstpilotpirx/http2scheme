@menu_bar_app
Feature: Menu bar app
  Run the local server as an ordinary Mac app: it lives in the menu bar, serves links
  while it runs, starts at login, and shows its state without a terminal.

  Rule: The app serves links from the menu bar

    @e2e @bundle
    Scenario: The app serves a keyed link
      Given the app is started from its bundle with no command
      When the browser follows a keyed open link to an IntelliJ link
      Then the Mac opens that IntelliJ link

    @e2e @bundle
    Scenario: The bundled program is also the command line tool
      Given the program inside the app bundle
      When the user asks it for the open link of an IntelliJ link
      Then it prints one open link to the loopback address

  Rule: Quit and the system end the app

    @e2e @bundle
    Scenario: A terminate signal ends the app
      Given the app runs and its server answers
      When the app receives a terminate signal
      Then the app ends
      And the server no longer answers
