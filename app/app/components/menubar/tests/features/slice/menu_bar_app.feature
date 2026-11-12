@menu_bar_app
Feature: Menu bar app
  Run the local server as an ordinary Mac app: it lives in the menu bar, serves links
  while it runs, starts at login, and shows its state without a terminal.

  Rule: The menu shows the app and the server state

    @slice
    Scenario: A running server
      Given the server listens on port 7777
      When the user opens the menu
      Then the menu shows the app name with its version
      And the status line says the server runs on the loopback address with port 7777

    @slice
    Scenario: A taken port
      Given another program holds the configured port 7777
      When the user opens the menu
      Then the status line reads "Stopped: port 7777 is taken"

  Rule: Start at Login follows the system's login items

    @slice
    Scenario: Turning Start at Login on
      Given the app is not in the login items
      When the user chooses Start at Login
      Then the app is in the login items
      And Start at Login shows a check mark

    @slice
    Scenario: Turning Start at Login off
      Given the app is in the login items
      When the user chooses Start at Login
      Then the app is not in the login items
      And Start at Login shows no check mark

    @slice
    Scenario: A login item that waits for approval
      Given macOS waits for the user's approval of the app's login item
      When the user chooses Start at Login
      Then the Login Items pane of System Settings opens

    @slice
    Scenario: Not an installed app
      Given the program does not run from an app bundle
      When the user opens the menu
      Then Start at Login is disabled

  Rule: The first launch from Applications starts the app at login

    @slice
    Scenario: The first launch from Applications
      Given the app runs from the Applications folder for the first time
      When the app starts
      Then the app is in the login items

    @slice
    Scenario: A login item the user removed stays removed
      Given the app added itself to the login items before and the user removed it
      When the app starts again
      Then the app is not in the login items

    @slice
    Scenario: A launch from another folder
      Given the app runs from a folder other than Applications for the first time
      When the app starts
      Then the login items are unchanged

  Rule: The menu opens the app's folders and quits

    @slice
    Scenario: Open the settings folder
      Given the app runs
      When the user chooses Open Settings Folder
      Then the Finder opens the settings folder

    @slice
    Scenario: Open the logs folder before the first request
      Given the app runs and no request was logged yet
      When the user chooses Open Logs Folder
      Then the logs folder exists
      And the Finder opens it
