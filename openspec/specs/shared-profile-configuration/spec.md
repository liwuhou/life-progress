# shared-profile-configuration Specification

## Purpose

Provide one validated, versioned personal profile file that the desktop application writes and the CLI can consume without translating user data.

## Requirements

### Requirement: Canonical profile location

The system SHALL use `~/.config/life_progress/profile.toml` as the sole canonical location for a shared user profile. It SHALL not treat the legacy `~/.life_progress/config.toml` path as initialized profile data.

The life-expectancy cache is separate from the profile and SHALL use `~/.config/life_progress/.tmp_expectancy.json`.

#### Scenario: Loading a configured profile

- **WHEN** `~/.config/life_progress/profile.toml` exists and is valid
- **THEN** the application loads that profile as its user data source

#### Scenario: Loading without a profile

- **WHEN** the canonical profile file does not exist
- **THEN** the application reports that no profile is configured without creating a profile or reading the legacy path

### Requirement: Versioned profile schema

A profile SHALL contain `schema_version = 1`, a birthday in `YYYY-MM-DD` form, a canonical nation name, and an optional gender of `male` or `female`. The profile SHALL contain no display preferences or life-expectancy cache data.

#### Scenario: Saving a valid profile

- **WHEN** a valid birthday, gender selection, and canonical nation are saved
- **THEN** the resulting TOML file conforms to schema version 1 and can be consumed unchanged by either application

### Requirement: Profile validation

The system SHALL reject malformed TOML, unsupported schema versions, missing required fields, invalid or future birthdays, invalid gender values, and unknown canonical nation names. It SHALL return a recoverable validation error rather than using partial or fallback profile data.

#### Scenario: Loading invalid profile data

- **WHEN** a profile contains an invalid field or unsupported schema version
- **THEN** loading fails with a recoverable validation error and no partial profile is returned
