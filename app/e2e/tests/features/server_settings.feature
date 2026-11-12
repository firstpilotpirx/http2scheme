@server_settings
Feature: Server settings
  Keep the settings and the secret key in one folder of the user's home,
  create them safely on first use, and let the user change them in one file.

  Rule: First use creates the settings and a secret key

    @e2e @serial
    Scenario: A fresh home gets default settings and a key
      Given no settings exist yet
      When the server starts
      Then the settings hold port 7777, the allowed schemes IntelliJ and GoLand, and tab closing on
      And a key of 64 hexadecimal characters exists that only the user can read

    @e2e @serial
    Scenario: Two first uses create two different keys
      Given two homes with no settings
      When the server starts once in each
      Then the two keys differ

    @e2e
    Scenario: Existing settings and key are kept
      Given the settings hold port 9000 and a key exists
      When the server starts
      Then the settings still hold port 9000
      And the key is unchanged

  Rule: The user changes the port, the allowed schemes and tab closing in the settings file

    @e2e
    Scenario: An allowed scheme added by hand is honored
      Given the user added the Slack scheme to the allowed list in the settings
      When the server starts and the browser follows a keyed open link to a Slack link
      Then the Mac opens that Slack link

    @e2e
    Scenario: A broken settings file stops the start with its reason
      Given the settings hold port 80
      When the server starts
      Then it does not start
      And it says the port in the settings file must be between 1024 and 65535

    @e2e
    Scenario: A key file readable by others stops the start
      Given the key file is readable by other users
      When the server starts
      Then it does not start
      And it says the key file must be readable only by its owner
