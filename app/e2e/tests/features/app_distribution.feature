@app_distribution
Feature: App distribution
  Install http2scheme the standard Mac way: a disk image the user opens and drags the app
  from into Applications, published with each release together with its checksum.

  Rule: The app bundle carries the app's information

    @e2e @bundle
    Scenario: The bundle's information
      Given the app bundle is built
      When the bundle's information is read
      Then it names the bundle identifier "io.github.firstpilotpirx.http2scheme" and the workspace version
      And it marks the app as a menu bar app with no Dock icon
      And it gives the reason for controlling a browser

  Rule: The app bundle carries the license texts

    @e2e @bundle
    Scenario: The bundle holds both license texts
      Given the app bundle is built
      When the bundle's resources are listed
      Then they hold "LICENSE-MIT" and "LICENSE-APACHE" equal to the repository's files

  Rule: The bundle is signed

    @e2e @bundle
    Scenario: A built bundle passes the signature check
      Given the app bundle is built
      When its signature is checked strictly
      Then the check passes

    @e2e @bundle
    Scenario: A release without a certificate stops
      Given no code-signing certificate is named
      When the release checks its signing identity
      Then it stops and names the variable to set

  Rule: The disk image installs the app

    @e2e @bundle
    Scenario: The disk image holds the app and the Applications link
      Given the disk image is built
      When the image is opened
      Then it holds "http2scheme.app" and a link to "/Applications"

    @e2e @bundle
    Scenario: The checksum file matches the disk image
      Given the disk image and its checksum file are built
      When the checksum is checked
      Then the check passes
