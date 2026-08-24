## Purpose

Ensure life-expectancy data remains available to installed applications without relying on the current working directory, a cache file, or network access.

## ADDED Requirements

### Requirement: Bundled offline expectancy data

The system SHALL load a bundled default life-expectancy dataset when no cache is available and the remote refresh fails. The fallback SHALL be independent of the process working directory.

#### Scenario: Starting offline without a cache

- **WHEN** no expectancy cache exists and the remote dataset cannot be fetched
- **THEN** the system returns the bundled default dataset

#### Scenario: Launching from an arbitrary directory

- **WHEN** the application starts from a directory that does not contain an expectancy data file
- **THEN** offline fallback still returns the bundled default dataset
