## Purpose

Give users a focused first-run and editable settings surface for maintaining the shared life profile without manually editing TOML.

## ADDED Requirements

### Requirement: First-run profile setup

When no valid shared profile exists, the application SHALL open the settings surface at startup and keep the tray application available without a persistent main window.

#### Scenario: Starting without a profile

- **WHEN** the application starts and the shared profile is absent
- **THEN** it opens the profile setup surface

#### Scenario: Starting with an invalid profile

- **WHEN** the shared profile cannot be loaded because it is invalid
- **THEN** it opens the profile setup surface with a recoverable validation message

### Requirement: Editable profile form

The settings surface SHALL let users edit birthday, optional gender, and nation. Nation search SHALL show canonical candidates, and saving SHALL persist only a validated canonical shared profile.

#### Scenario: Saving profile settings

- **WHEN** the user enters valid profile data and saves
- **THEN** the shared profile is updated and the settings surface reflects the saved canonical nation

#### Scenario: Rejecting invalid profile input

- **WHEN** the user enters an invalid or future birthday, or an unresolvable nation
- **THEN** the settings surface shows the validation error and does not change the saved profile

### Requirement: On-demand settings access

The tray menu SHALL provide a settings action that opens or focuses the settings surface.

#### Scenario: Opening settings from the tray

- **WHEN** the user selects Settings from the tray menu
- **THEN** the settings surface becomes visible without creating a persistent main window
