## Why

The application is currently pinned to Tauri 1.5 while the intended native tray architecture is based on Tauri 2. Migrating the platform first establishes a supported tray API and a current Rust toolchain before any life-progress behavior is added.

## What Changes

- Upgrade the native application from Tauri 1.5 to Tauri 2, including the v2 configuration schema, `tray-icon` Cargo feature, and Rust tray/menu APIs.
- **BREAKING**: Replace the Tauri 1 configuration shape, system-tray API, and Cargo feature names; no Tauri 1 compatibility layer remains.
- Select and lock a Rust MSRV compatible with the chosen Tauri 2 release, then update development and CI commands to use it.
- Preserve `windows: []` and validate that the migrated application still creates exactly one system-tray icon with no application window; profile persistence, onboarding, and configurable life-progress presentation are explicitly out of scope.

## Capabilities

### New Capabilities
- `tauri-platform-migration`: Build and run the native application on Tauri 2 while retaining its tray-only behavior.

### Modified Capabilities
- None.

## Impact

- Affected code: `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/src/main.rs`, and build or toolchain configuration required by Tauri 2.
- Affected dependencies: `tauri`, `tauri-build`, the built-in `tray-icon` feature, and the Rust toolchain baseline.
- Follow-up work will add the shared `profile.toml`, nation resolution, first-run onboarding, and life-progress tray presentation after the platform migration is complete.
