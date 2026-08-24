## MODIFIED Requirements

### Requirement: Profile validation

The system SHALL reject malformed TOML, unsupported schema versions, missing required fields, invalid or future birthdays, invalid gender values, and unknown canonical nation names. It SHALL return a recoverable validation error rather than using partial or fallback profile data.

#### Scenario: Loading invalid profile data

- **WHEN** a profile contains an invalid field or unsupported schema version
- **THEN** loading fails with a recoverable validation error and no partial profile is returned

#### Scenario: Loading a valid profile offline

- **WHEN** a valid profile is loaded without an expectancy cache and the network is unavailable
- **THEN** profile validation succeeds using bundled expectancy data
