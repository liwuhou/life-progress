## 1. Rust settings domain

- [ ] 1.1 Add desktop preference storage, display model, title template rendering, and tray rendering helpers; verify with focused unit tests for metrics, templates, preferences, and style output.
- [x] 1.2 Add Rust commands for loading and saving profiles/preferences plus nation search, and update the tray menu and startup flow to open settings on demand; verify with helper tests and `cargo check`.

## 2. Settings surface

- [x] 2.1 Add the minimal Tauri capability configuration and an on-demand `settings` WebView window; verify that valid-profile startup remains windowless and tray Settings opens one focused window.
- [x] 2.2 Build static settings HTML, CSS, and JavaScript for profile editing, nation candidates, display controls, and live preview; verify the rendered surface in the actual WebView.

## 3. Validation

- [x] 3.1 Run formatter, workspace tests, and native build; verify with `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --workspace`, and `cargo build --manifest-path src-tauri/Cargo.toml --features custom-protocol`.
- [ ] 3.2 Launch the application and verify onboarding, profile save, preference persistence, tray update, settings re-open, and windowless normal startup on macOS.
