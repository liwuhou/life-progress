# tray-menu-details Specification

## Purpose

为 Life Progress 的原生托盘菜单提供低打扰的生命进度摘要，让用户无需打开设置页即可查看已过天数、剩余天数和当前显示百分比，同时保留设置与退出操作。

## Requirements

### Requirement: Life progress summary items

The native tray menu SHALL display non-actionable summary items for elapsed days, remaining days, and the currently selected metric percentage. Summary items SHALL appear above the actionable menu items and SHALL be visually separated from them.

#### Scenario: Configured profile opens the tray menu

- **WHEN** the user opens the tray menu with a configured profile
- **THEN** the menu shows the elapsed day count, remaining day count, and the percentage for the selected life metric before the “设置” and “退出” actions

#### Scenario: Summary items are not actionable

- **WHEN** the user clicks an elapsed-day, remaining-day, or percentage summary item
- **THEN** no settings window, profile mutation, or application exit is triggered

### Requirement: Metric-aware percentage

The percentage summary SHALL use the same `LifeMetric` selected for the tray display. The elapsed and remaining day summaries SHALL always both remain visible regardless of the selected metric.

#### Scenario: Remaining metric

- **WHEN** the selected metric is remaining life
- **THEN** the percentage summary represents remaining-life percentage while both elapsed and remaining day counts are shown

#### Scenario: Elapsed metric

- **WHEN** the selected metric is elapsed life
- **THEN** the percentage summary represents elapsed-life percentage while both elapsed and remaining day counts are shown

### Requirement: Summary refresh

The tray summary SHALL refresh after application startup, after settings are saved, and when the calendar day changes while the application remains running. Refreshing the summary SHALL use the same profile and validated preferences as the tray title and icon.

#### Scenario: Settings change refreshes summary

- **WHEN** the user saves a changed life metric or profile
- **THEN** the tray menu immediately displays counts and percentage based on the saved values

#### Scenario: Date boundary refreshes summary

- **WHEN** the local calendar date advances while the application remains running
- **THEN** the tray summary updates to the new elapsed and remaining day counts without requiring the user to reopen the application

### Requirement: Unconfigured profile behavior

The application SHALL keep the tray menu usable before profile setup is complete and SHALL not display fabricated personal progress values.

#### Scenario: No configured profile

- **WHEN** the user opens the tray menu before completing profile setup
- **THEN** the menu omits personal summary values or shows an explicit unavailable state, while retaining the “设置” and “退出” actions

### Requirement: Left-click menu behavior

A left click on the tray icon SHALL open the native tray menu without also opening the settings window. The settings window SHALL open only when the user selects the “设置” menu action.

#### Scenario: Left-clicking the tray icon

- **WHEN** the user releases the primary mouse button on the tray icon
- **THEN** the native tray menu is shown and the settings window remains closed

#### Scenario: Selecting settings

- **WHEN** the user selects “设置” from the native tray menu
- **THEN** the settings window opens or is brought to focus

### Requirement: Local calendar date consistency

The elapsed-day calculation and date-change refresh trigger SHALL use the same local calendar date semantics. A local midnight SHALL be sufficient to advance the displayed elapsed and remaining day values; UTC boundaries SHALL NOT delay or duplicate that refresh.

#### Scenario: Local midnight in a non-UTC timezone

- **WHEN** the local calendar date advances while the UTC calendar date has not advanced yet
- **THEN** the next summary refresh uses the new local date and updates elapsed and remaining day values exactly once
