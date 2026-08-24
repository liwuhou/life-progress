## Context

The native shell currently uses Tauri 1.5 APIs (`SystemTray`, `SystemTrayEvent`) and the Tauri 1 configuration layout. It has no configured windows and is run directly with Cargo. Tauri 2.11.5 and `tauri-build` 2.6.3 require Rust 1.77.2, but the resolved dependency graph requires Rust 1.88; Rust 1.88 therefore replaces the Rust 1.67 baseline.

## Goals / Non-Goals

**Goals:**
- Perform a clean Tauri 2 cutover with a pinned Rust toolchain.
- Keep startup tray-only and retain the existing left-click event path.
- Make the migrated configuration explicit and valid for Cargo-driven development and packaging.

**Non-Goals:**
- Adding a settings window, webview, frontend build pipeline, IPC commands, profile persistence, or life-progress rendering.
- Preserving Tauri 1 configuration or API compatibility.

## Decisions

### Pin the platform toolchain

Pin Tauri and `tauri-build` to their matching 2.11.5 / 2.6.3 releases and add a repository Rust toolchain declaration for Rust 1.88. Keep `rust-version` aligned with that toolchain so local development and CI use the same compiler contract.

### Perform a configuration cutover

Replace the Tauri 1 configuration with the v2 schema and key layout. Preserve the static `assets` directory as the frontend distribution only to satisfy Tauri's asset contract; do not add a frontend development server. Keep `app.windows` empty and create the sole tray icon at runtime so configuration cannot create a second tray icon.

### Replace the tray runtime atomically

Replace the v1 global `SystemTray` registration with a single `TrayIconBuilder` instance created during application setup. Enable Tauri's built-in `tray-icon` feature, give the icon a stable `main` identifier, and route left-click events through the v2 tray event type. Do not retain v1 imports or adapter code.

### Hide the macOS application process

Set the macOS activation policy to `Accessory` at runtime and merge `LSUIElement` into the bundled application Info.plist. The runtime policy covers Cargo-launched builds; the plist preserves the agent behavior in packaged applications.

## Risks / Trade-offs

- Rust 1.88 is a substantial toolchain jump from the current baseline; it is required by the resolved Tauri 2 dependency graph, and pinning it prevents different local compiler versions from producing inconsistent migration results.
- The application remains windowless, so native launch verification must inspect the actual system tray rather than a webview surface.
- Tauri 2 configuration validation is stricter than v1; invalid conversion of asset or bundle fields can prevent startup or packaging, so the migration must validate both Cargo checks and a launched tray process.
