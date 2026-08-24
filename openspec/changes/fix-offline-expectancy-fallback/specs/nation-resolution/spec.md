## MODIFIED Requirements

### Requirement: Canonical nation resolution

The system SHALL resolve a nation input to a canonical dataset name. An exact canonical-name match SHALL take precedence over fuzzy matching.

#### Scenario: Supplying a canonical nation name

- **WHEN** the input exactly matches a dataset nation name
- **THEN** resolution returns that exact canonical name

#### Scenario: Resolving without cache or network

- **WHEN** nation input is resolved without an expectancy cache and the network is unavailable
- **THEN** resolution uses the bundled dataset and returns a canonical nation name
