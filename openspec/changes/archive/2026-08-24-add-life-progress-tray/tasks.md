## 1. Platform and configuration migration

- [x] 1.1 Pin Rust 1.88 and compatible Tauri 2 / `tauri-build` releases, update Cargo feature names, and update documented native commands; verify with `rustc --version` and `cargo check --manifest-path src-tauri/Cargo.toml`.
- [x] 1.2 Convert `tauri.conf.json` to the Tauri 2 schema while retaining the static asset directory and `app.windows: []`; verify the configuration is accepted by the native Cargo check and build commands.

## 2. Tray runtime migration

- [x] 2.1 Replace the Tauri 1 tray setup and event imports with one Tauri 2 `TrayIconBuilder` instance using the stable `main` identifier; verify with `cargo fmt --manifest-path src-tauri/Cargo.toml --check` and `cargo check --manifest-path src-tauri/Cargo.toml`.
- [x] 2.2 Run workspace tests and launch the native application to verify a single tray icon appears, no window opens, left click is handled without terminating, and macOS does not show the app in the Dock or Command-Tab switcher; verify with `cargo test --manifest-path src-tauri/Cargo.toml --workspace` and the launched tray application.
