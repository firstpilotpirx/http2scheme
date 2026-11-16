@agent_skill
Feature: Agent skill
  Teach coding agents to write a clickable IDE link next to every place in code they mention,
  with a skill the program prints, installs into a Claude Code folder and keeps current.

  Rule: The skill tells the agent how to write IDE links

    @slice
    Scenario: The printed skill names the program and the link style
      Given the program is installed at a known path
      When the user asks the program for the skill text
      Then the text starts with a header naming the skill http2scheme and saying when to use it
      And it names that program path for the link command
      And its example shows a file link followed by a link labelled IDE

  Rule: The skill installs into a Claude Code folder

    @slice
    Scenario: Install writes the skill into the default folder
      Given the user's Claude Code folder has no skill of this program
      When the user installs the skill without naming a folder
      Then the skill file appears in the default Claude Code folder's skills
      And the command prints the path of that file and succeeds

    @slice
    Scenario: Install into a chosen work profile folder
      Given the user keeps a second Claude Code folder for work
      When the user installs the skill naming that folder
      Then the skill file appears in that folder's skills
      And the default Claude Code folder stays untouched

    @slice
    Scenario: Install over an older skill replaces it
      Given the Claude Code folder holds an older text of the skill and another skill beside it
      When the user installs the skill
      Then the skill file holds the current text
      And the other skill is unchanged

    @slice
    Scenario: A linked skills folder is named in the output
      Given the Claude Code folder's skills folder is a link to a folder elsewhere
      When the user installs the skill
      Then the skill file appears in the folder the link points to
      And the command also prints that real folder
