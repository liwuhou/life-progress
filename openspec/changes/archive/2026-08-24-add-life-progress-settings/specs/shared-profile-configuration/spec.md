## MODIFIED Requirements

### Requirement: Canonical profile location

The system SHALL use `~/.config/life_progress/profile.toml` as the sole canonical location for a shared user profile. It SHALL not treat the legacy `~/.life_progress/config.toml` path as initialized profile data.

The life-expectancy cache is separate from the profile and SHALL use `~/.config/life_progress/.tmp_expectancy.json`.

#### Scenario: Loading a configured profile

- **WHEN** `~/.config/life_progress/profile.toml` exists and is valid
- **THEN** the application loads that profile as its user data source

#### Scenario: Loading without a profile

- **WHEN** the canonical profile file does not exist
- **THEN** the application reports that no profile is configured without creating a profile or reading the legacy path

#### Scenario: Saving through settings

- **WHEN** settings saves a validated profile
- **THEN** it replaces the canonical shared profile file
