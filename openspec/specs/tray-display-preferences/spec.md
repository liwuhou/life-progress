# tray-display-preferences Specification

## Purpose

Let users choose how their remaining or elapsed lifetime is represented in the tray while keeping private profile data separate from presentation settings.

## Requirements

### Requirement: Life metric selection

The settings surface SHALL let users choose remaining or elapsed lifetime and update the tray accordingly.

#### Scenario: Selecting a life metric

- **WHEN** the user saves a selected metric
- **THEN** the tray uses the corresponding percentage and day count

### Requirement: Tray style selection

The settings surface SHALL offer percentage text, short text, custom text, ring progress, horizontal progress bar, threshold bar, and bar chart styles.

#### Scenario: Selecting a graphical style

- **WHEN** the user saves a graphical tray style
- **THEN** the tray renders that style using the selected metric

### Requirement: Right-side title template

The settings surface SHALL allow users to show or hide a right-side tray title and configure `{mode}`, `{percent}`, and `{days}` values.

#### Scenario: Saving a title template

- **WHEN** the user saves a title template
- **THEN** the tray displays the rendered title or hides it when disabled

### Requirement: Separate presentation persistence

The application SHALL persist display preferences separately from the shared profile and SHALL not add display fields to `profile.toml`.

#### Scenario: Reopening settings

- **WHEN** the user reopens settings after saving display preferences
- **THEN** the selected metric, style, and title preference are restored
