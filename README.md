# Life Progress

Life Progress is a native Tauri system-tray application that visualizes the user's remaining lifetime as a progress indicator, encouraging deliberate use of time.

## Development

Initialize the Rust workspace submodules before building.

The repository pins Rust 1.88 through `rust-toolchain.toml`; Rustup installs it automatically when running the commands below.

```sh
git submodule update --init --recursive
cargo run --manifest-path src-tauri/Cargo.toml
```

The application is tray-only; it does not run a browser server or use a JavaScript frontend.

## Validation

```sh
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml --workspace
cargo build --manifest-path src-tauri/Cargo.toml --features custom-protocol
```

The workspace includes unit tests in `src-tauri/crates/life-progress-core`. No coverage threshold is configured. Manually verify the tray icon and its progress/menu behavior after launching the application.
