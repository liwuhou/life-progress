## MODIFIED Requirements

### Requirement: Tray-only runtime

The migrated application SHALL remain a system-tray application with no configured application windows and exactly one tray icon at startup. It MAY create an on-demand settings window only while the user is configuring the application.

#### Scenario: Launching the migrated application

- **WHEN** the application is launched on a supported desktop platform with a valid profile
- **THEN** it creates one system-tray icon and does not create an application window

#### Scenario: Opening settings

- **WHEN** the user requests settings from the tray
- **THEN** the application creates or focuses the on-demand settings window
