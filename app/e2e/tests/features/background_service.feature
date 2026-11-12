@background_service
Feature: Background service
  Keep the local server running without the user's attention, with simple commands
  to install, remove, start, stop and check it.

  Rule: Install makes the server start at login and restart after a crash

    @e2e
    Scenario: Install starts the server and keeps it at login
      Given the agent is not installed
      When the user installs the agent
      Then the agent definition exists and asks to run at login and to restart after an exit
      And the agent is loaded
      And the command says the agent is installed and running

    @e2e
    Scenario: A second install replaces the first
      Given the agent is installed
      When the user installs the agent again
      Then one agent definition exists, pointing at the current program
      And the agent is loaded

  Rule: Uninstall removes the agent

    @e2e
    Scenario: Uninstall stops and removes the agent
      Given the agent is installed and loaded
      When the user uninstalls the agent
      Then the agent is unloaded and its definition is gone
      And the settings and the key remain

    @e2e
    Scenario: Uninstall with nothing installed is harmless
      Given the agent is not installed
      When the user uninstalls the agent
      Then the command succeeds and says nothing was installed

  Rule: Start and stop control an installed agent

    @e2e
    Scenario: Stop keeps the definition
      Given the agent is installed and loaded
      When the user stops the agent
      Then the agent is unloaded and its definition remains

    @e2e
    Scenario: Start loads a stopped agent
      Given the agent is installed but not loaded and the server does not answer
      When the user starts the agent
      Then the agent is loaded

    @e2e
    Scenario: Start without install gives a hint
      Given the agent is not installed
      When the user starts the agent
      Then the command fails and says to install the agent first

  Rule: Status reports the agent and the server

    @e2e
    Scenario: A healthy setup
      Given the agent is installed and loaded and the server answers
      When the user asks for the status
      Then the command shows installed, loaded and answering
      And it ends with a success status

    @e2e
    Scenario: A stopped server
      Given the agent is installed but not loaded and the server does not answer
      When the user asks for the status
      Then the command shows installed, not loaded and not answering
      And it ends with a failure status

  Rule: Serve runs the server in the foreground

    @e2e
    Scenario: Serve on a free port
      Given the configured port is free
      When the user runs the server in the foreground
      Then it prints the loopback address with the port
      And the health address answers

    @e2e
    Scenario: Serve on a taken port
      Given another program holds the configured port
      When the user runs the server in the foreground
      Then it fails and says the port is taken
