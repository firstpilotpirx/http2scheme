@agent_skill
Feature: Agent skill
  Teach coding agents to write a clickable IDE link next to every place in code they mention,
  by giving them a skill the program prints, installs into a Claude Code folder and keeps current.

  Rule: The skill tells the agent how to write IDE links

    @e2e
    Scenario: The printed skill names the program and the link style
      Given the program is installed at a known path
      When the user asks the program for the skill text
      Then the text starts with a header naming the skill http2scheme and saying when to use it
      And it names that program path for the link command
      And its example shows a file link followed by a link labelled IDE

  Rule: The skill installs into a Claude Code folder

    @e2e
    Scenario: Install writes the skill into the default folder
      Given the user's Claude Code folder has no skill of this program
      When the user installs the skill without naming a folder
      Then the skill file appears in the default Claude Code folder's skills
      And the command prints the path of that file and succeeds
