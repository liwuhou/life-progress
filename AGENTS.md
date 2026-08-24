# Repository Guidelines

## Project Overview

Life Progress is a native Tauri system-tray application. Its primary experience is a progress indicator that visualizes a user's remaining lifetime and encourages deliberate use of time. There is no Next.js, React, webview, or JavaScript frontend.

## Architecture & Data Flow

- **Target ownership:** Rust/Tauri owns life-expectancy inputs, remaining-time calculations, periodic refresh scheduling, persistence, and system-tray rendering.
- **Target data flow:** user profile/settings → validated Rust domain state → remaining-lifetime calculation → tray icon/progress and menu update. Persist only the minimum required personal data.
- **Desktop process:** `src-tauri/src/main.rs` builds the Tauri runtime, configures the tray, and handles tray events. `src-tauri/src/lib.rs` contains filesystem/config helpers for `~/.config/life_progress/profile.toml`; the expectancy cache is `~/.config/life_progress/.tmp_expectancy.json`.
- **Tray-only contract:** `src-tauri/tauri.conf.json` sets `app.windows` to an empty array. Do not add a webview or frontend build pipeline unless the product direction changes explicitly.

## Key Directories

- `src-tauri/src/` — Tauri process entry point and application helpers.
- `src-tauri/assets/` — minimal static asset directory required by Tauri; no webview windows are configured.
- `src-tauri/crates/life-progress-core/` — local domain crate submodule.
- `src-tauri/crates/lifespan-crawler/` — local data crate submodule.

## Development Commands

Initialize the workspace submodules once after cloning:

```sh
git submodule update --init --recursive
cargo run --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml --features custom-protocol
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

No Tauri CLI packaging command is configured in the repository.

## Code Conventions & Common Patterns

- Keep business logic, persistence, timing, and tray presentation in Rust; do not introduce a JavaScript IPC layer.
- Model recoverable failures with `Result<T>` and propagate them with `?`. Reserve `expect` for fatal application-startup failures.
- Keep tray event handlers thin. Put configuration access, lifetime calculations, and icon/progress generation in testable Rust helpers or local workspace crates.
- Treat profile and life-expectancy data as sensitive: validate it at the boundary, store the minimum required fields, and do not log it.
- Rust uses four-space indentation through `.editorconfig`.

## Important Files

- `src-tauri/Cargo.toml` — application manifest, workspace members, Rust version, and Tauri dependencies.
- `src-tauri/build.rs` — invokes `tauri_build::build()`.
- `src-tauri/tauri.conf.json` — tray icon, no-window configuration, and native asset paths.
- `src-tauri/src/main.rs` — Tauri application entry point and tray event handling.
- `src-tauri/src/lib.rs` — application configuration helpers.
- `.gitmodules` — required local workspace crates; initialize them before native builds.

## Runtime/Tooling Preferences

- Use Cargo and Rust only. Rust edition 2021 with `rust-version = "1.88"` is the project baseline; Tauri is 2.11.x.
- Do not reintroduce Node, pnpm, Next.js, React, Tailwind, or a browser build step without an explicit product decision.
- `src-tauri/.gitignore` excludes Cargo build artifacts.

## Testing & QA

- Run workspace tests with `cargo test --manifest-path src-tauri/Cargo.toml --workspace`; current unit tests live in `src-tauri/crates/life-progress-core`.
- No coverage threshold is configured.
- Before submitting a change, run `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --workspace`, and `cargo check --manifest-path src-tauri/Cargo.toml`.
- For tray behavior, launch the application and manually verify the native tray icon, menu actions, and periodic progress refresh. Initialize the workspace submodules first.
