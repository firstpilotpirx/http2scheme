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

  Rule: Serve Links turns the server off and on

    @slice
    Scenario: Turning the server off
      Given the app runs and its server answers
      When the user chooses Serve Links
      Then the server no longer answers
      And the status line reads "Off"
      And Serve Links shows no check mark

    @slice
    Scenario: Turning the server on again
      Given the user turned the server off
      When the user chooses Serve Links
      Then the server answers again
      And Serve Links shows a check mark

    @slice
    Scenario: Turning the server off in the Settings window
      Given the app runs and its server answers
      When the user turns the Serve links switch off in the Settings window
      Then the server no longer answers
      And the line under the switch reads "Off"

  Rule: The History tab lists the latest requests

    @slice
    Scenario: Requests are listed newest first
      Given the request log holds an opened IntelliJ link and, after it, a GoLand link with the wrong key
      When the user opens the History tab
      Then the first row shows the GoLand link with the result "wrong key"
      And the second row shows the IntelliJ link with the result "opened"

    @slice
    Scenario: Only the latest 100 requests are listed
      Given the request log holds 105 requests
      When the user opens the History tab
      Then it lists 100 rows, the latest 100

    @slice
    Scenario: An unreadable log line has no row
      Given the request log holds two requests and a damaged line between them
      When the user opens the History tab
      Then it lists the two requests

    @slice
    Scenario: No request yet
      Given the request log does not exist
      When the user opens the History tab
      Then the tab says no link has been opened yet

  Rule: A row's link opens again

    @slice
    Scenario: Opening an IntelliJ link again
      Given the History tab lists an opened IntelliJ link
      When the user chooses ▶ on its row
      Then the Mac opens that IntelliJ link
      And the first row shows that IntelliJ link with the result "opened"

    @slice
    Scenario: A link whose scheme is not allowed stays closed
      Given the History tab lists a Slack link with the result "scheme not allowed"
      When the user chooses ▶ on its row
      Then the Mac opens nothing
      And the first row shows that Slack link with the result "scheme not allowed"

  Rule: Start at Login follows the system's login items

    @slice
    Scenario: Turning Start at Login on
      Given the app is not in the login items
      When the user chooses Start at Login
      Then the app is in the login items
      And Start at Login shows a check mark

    @slice
    Scenario: Turning Start at login on in the Settings window
      Given the app is not in the login items
      When the user turns the Start at login switch on in the Settings window
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

  Rule: The Settings window shows the switches and the history

    @slice
    Scenario: Settings opens the General tab
      Given the app runs and its server listens on port 7777
      When the user chooses Settings…
      Then the Settings window shows its General tab
      And the Serve links switch is on, with the line under it saying the server runs on port 7777

    @slice
    Scenario: Link History opens the History tab
      Given the app runs
      When the user chooses Link History…
      Then the Settings window shows its History tab
