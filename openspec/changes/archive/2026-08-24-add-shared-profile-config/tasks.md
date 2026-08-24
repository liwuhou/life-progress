## 1. Shared profile domain

- [x] 1.1 Add the versioned `Profile` model, TOML dependencies, canonical path lookup, parsing, validation, and atomic owner-only writes to `life-progress-core`; verify with focused profile serialization, missing-file, malformed-file, future-birthday, and invalid-field tests.
- [x] 1.2 Add deterministic nation resolution to `life-progress-core`, preserving exact canonical matches and sorting fuzzy results by score then canonical name; verify with canonical, `china`, tie-break, and no-match tests.

## 2. Desktop integration

- [x] 2.1 Replace the desktop legacy initialization-path check with the core shared-profile load result; verify with application helper tests for configured, absent, and invalid profiles.

## 3. Validation

- [x] 3.1 Run formatting, workspace tests, and native Cargo checks against the pinned toolchain; verify with `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --workspace`, and `cargo check --manifest-path src-tauri/Cargo.toml`.
