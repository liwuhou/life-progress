# nation-resolution Specification

## Purpose

Normalize country input into one canonical life-expectancy dataset name so persisted profiles are stable and directly usable for lifetime calculations.

## Requirements

### Requirement: Canonical nation resolution

The system SHALL resolve a nation input to a canonical dataset name. An exact canonical-name match SHALL take precedence over fuzzy matching.

#### Scenario: Supplying a canonical nation name

- **WHEN** the input exactly matches a dataset nation name
- **THEN** resolution returns that exact canonical name

### Requirement: Deterministic fuzzy matching

When no exact match exists, the system SHALL rank fuzzy matches by descending match score and then by ascending canonical nation name to break equal scores. It SHALL return the first ranked result as the canonical resolution.

#### Scenario: Resolving an abbreviated nation input

- **WHEN** the user supplies `china`
- **THEN** resolution returns `People's Republic of China` for persistence

#### Scenario: Resolving tied fuzzy matches

- **WHEN** two or more fuzzy nation matches have equal scores
- **THEN** resolution selects the lexicographically first canonical nation name among those matches

### Requirement: Unresolvable nation input

The system SHALL return a recoverable error when a nation input has no exact or fuzzy match. It SHALL not persist the original search text or silently substitute `Common`.

#### Scenario: Supplying an unknown nation

- **WHEN** the user supplies text that has no dataset match
- **THEN** profile validation receives a recoverable nation-resolution error
