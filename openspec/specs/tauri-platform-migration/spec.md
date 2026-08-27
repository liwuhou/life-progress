# tauri-platform-migration Specification

## Purpose

Move the native shell to Tauri 2 without changing its tray-only product behavior, so later tray features use the current supported platform API.

## Requirements

### Requirement: Tauri 2 native build

The application SHALL build and type-check with a Tauri 2 release and a documented, compatible Rust toolchain. The project SHALL not retain Tauri 1 dependencies, feature flags, or configuration keys.

#### Scenario: Checking the native project

- **WHEN** a developer runs the documented native check command with the documented Rust toolchain
- **THEN** Cargo successfully checks the application and its workspace dependencies using Tauri 2

### Requirement: Tray-only runtime

The migrated application SHALL remain a system-tray application with no configured application windows and exactly one tray icon at startup.

#### Scenario: Launching the migrated application

- **WHEN** the application is launched on a supported desktop platform
- **THEN** it creates one system-tray icon and does not create an application window

### Requirement: macOS agent application

On macOS, the application SHALL run as an accessory agent so it is absent from the Dock and Command-Tab application switcher while remaining available through its tray icon.

#### Scenario: Launching on macOS

- **WHEN** the application is launched on macOS
- **THEN** it does not appear in the Dock or Command-Tab switcher

### Requirement: Functional tray interaction

The migrated tray icon SHALL preserve the existing left-click interaction entry point and accept tray events through the Tauri 2 tray API.

#### Scenario: Clicking the tray icon

- **WHEN** the user left-clicks the system-tray icon
- **THEN** the application receives the tray click event without terminating

### Requirement: On-demand settings window

The application MAY create a settings window only while the user is configuring the application; normal startup remains windowless.

#### Scenario: Opening settings

- **WHEN** the user requests settings from the tray
- **THEN** the application creates or focuses the settings window
