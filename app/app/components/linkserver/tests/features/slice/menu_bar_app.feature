@menu_bar_app
Feature: Menu bar app
  The server's side of the History tab: a row's link opens again through the
  same checks as a keyed request, and the attempt is logged.

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
