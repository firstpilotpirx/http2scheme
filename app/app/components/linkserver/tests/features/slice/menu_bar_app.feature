@menu_bar_app
Feature: Menu bar app
  The server's side of the History and Agents tabs: a row's link opens again through the
  same checks as a keyed request, and the attempt is logged; the Agents tab reads, writes
  and copies the skill through the server.

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

  Rule: The Agents tab reaches the skill through the server

    @slice
    Scenario: Choosing a work folder shows its state
      Given a work Claude Code folder holds the current skill
      When the Agents tab asks the server about that folder
      Then the server answers that the skill is installed in that folder

    @slice
    Scenario: Install puts the skill into the shown folder
      Given the user keeps a second Claude Code folder for work
      When the Agents tab asks the server to install the skill into that folder
      Then the skill file appears in that folder's skills
      And the server answers that the skill is installed in that folder

    @slice
    Scenario: Copy puts the skill on the clipboard
      Given the program is installed at a known path
      When the Agents tab asks the server for the skill text
      Then the answer is the text the skill command prints
