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
