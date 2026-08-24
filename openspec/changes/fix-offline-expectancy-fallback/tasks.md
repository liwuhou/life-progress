## 1. Bundled fallback

- [x] 1.1 Replace working-directory default-data loading with an embedded crawler dataset while retaining cache-first and successful-fetch behavior; verify with clean-cache, failed-fetch fallback tests from an arbitrary working directory.

## 2. Offline profile behavior

- [x] 2.1 Add core regression tests showing canonical nation resolution and valid profile loading succeed against bundled data when cache and network sources are unavailable; verify with focused core tests that do not read the user cache.

## 3. Validation

- [x] 3.1 Run formatting, workspace tests, and native Cargo checks with the pinned toolchain; verify with `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --workspace`, and `cargo check --manifest-path src-tauri/Cargo.toml`.
