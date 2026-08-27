# tray-display-preferences Specification

## MODIFIED Requirements

### Requirement: Tray style selection

The settings surface SHALL offer percentage text, short text, custom text, ring progress, horizontal progress bar, threshold bar, rainbow threshold bar, and bar chart styles. Graphical styles SHALL render on a transparent canvas with consistent insets, rounded geometry where applicable, and a track/fill treatment that remains visually correct when the configured icon width changes.

#### Scenario: Selecting a graphical style

- **WHEN** the user saves a graphical tray style
- **THEN** the tray renders that style using the selected life metric and the configured graphical preferences

#### Scenario: Resizing a horizontal progress bar

- **WHEN** the user saves a horizontal progress bar width between the supported minimum and maximum
- **THEN** the rendered bar preserves its vertical proportions, transparent outer margins, rounded corners, and correct fill percentage without stretching artifacts

### Requirement: Color mode selection

The settings surface SHALL allow the user to select monochrome or threshold color mode for graphical tray styles. Monochrome mode SHALL use the existing dark fill and gray track appearance. Threshold mode SHALL use red below the lower boundary, orange from the lower boundary through the upper boundary, and green above the upper boundary, with the unfilled portion rendered as a subdued track or translucent color as appropriate to the selected style.

#### Scenario: Saving monochrome mode

- **WHEN** the user saves monochrome color mode
- **THEN** graphical tray styles use monochrome fill and track colors and do not display threshold colors

#### Scenario: Saving threshold mode

- **WHEN** the user saves threshold color mode with valid lower and upper boundaries
- **THEN** the tray applies red, orange, and green according to those boundaries and persists the selected mode and boundaries

### Requirement: Threshold boundary validation

The application SHALL constrain threshold boundaries to the percentage domain, require the lower boundary to be less than the upper boundary, and apply safe defaults when older or invalid persisted preferences are loaded.

#### Scenario: Invalid threshold boundaries

- **WHEN** persisted or submitted boundaries are outside the valid domain or lower is not less than upper
- **THEN** the application clamps or normalizes them to an ordered valid pair and remains usable without exposing profile data

### Requirement: Metric-independent color mapping

Threshold colors SHALL map to the percentage currently displayed by the selected `LifeMetric`; the mapping SHALL NOT be silently inverted when switching between remaining-life and elapsed-life metrics. This makes color a neutral indication of the displayed progress value rather than an undocumented assertion about health or urgency.

#### Scenario: Threshold colors for remaining life

- **WHEN** the selected metric is remaining life and the displayed percentage is 86%
- **THEN** threshold mode uses the upper-range color (green with default boundaries)

#### Scenario: Threshold colors for elapsed life

- **WHEN** the selected metric is elapsed life and the displayed percentage is 86%
- **THEN** threshold mode also uses the upper-range color (green with default boundaries)

### Requirement: Separate presentation persistence

The application SHALL persist display preferences, including color mode and threshold boundaries, separately from the shared profile and SHALL not add display fields to `profile.toml`.

#### Scenario: Reopening settings

- **WHEN** the user reopens settings after saving display preferences
- **THEN** the selected metric, style, color mode, threshold boundaries, title preference, and icon width are restored

### Requirement: macOS tray color rendering

The macOS tray integration SHALL preserve the RGBA colors of color-enabled graphical styles and SHALL NOT unconditionally apply template-icon rendering to those styles. Monochrome styles MAY use platform template behavior only when it produces the same visible monochrome result.

#### Scenario: Displaying a threshold-colored icon on macOS

- **WHEN** a threshold-colored graphical style is displayed in the macOS tray
- **THEN** its red, orange, green, and track colors are visible as rendered rather than being converted into a single template mask

#### Scenario: Updating a colored icon

- **WHEN** the selected metric or percentage changes
- **THEN** the macOS tray replaces the icon while preserving the selected color mode and current RGBA color mapping
