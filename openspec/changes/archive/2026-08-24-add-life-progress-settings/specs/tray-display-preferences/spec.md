## Purpose

Let users choose how their remaining or elapsed lifetime is represented in the tray while keeping private profile data separate from presentation settings.

## ADDED Requirements

### Requirement: Life metric selection

The settings surface SHALL let users choose whether tray progress represents remaining lifetime or elapsed lifetime.

#### Scenario: Selecting remaining lifetime

- **WHEN** the user selects remaining lifetime
- **THEN** the tray preview and saved preference use remaining percentage and remaining days

#### Scenario: Selecting elapsed lifetime

- **WHEN** the user selects elapsed lifetime
- **THEN** the tray preview and saved preference use elapsed percentage and elapsed days

### Requirement: Tray style selection

The settings surface SHALL offer percentage text, short text, custom text, ring progress, horizontal progress bar, threshold bar, and bar chart tray styles.

#### Scenario: Selecting a graphical style

- **WHEN** the user selects a graphical tray style
- **THEN** the settings surface shows a preview of that style using the selected life metric

### Requirement: Right-side title template

The settings surface SHALL allow users to show or hide a right-side tray title and configure it with supported life values.

#### Scenario: Previewing a title template

- **WHEN** the user edits a title template
- **THEN** the preview renders supported values and preserves unsupported text literally

### Requirement: Separate presentation persistence

The application SHALL persist display preferences separately from the shared profile and SHALL not add display fields to `profile.toml`.

#### Scenario: Reopening settings

- **WHEN** the user reopens settings after saving display preferences
- **THEN** the selected metric, style, and title preference are restored
