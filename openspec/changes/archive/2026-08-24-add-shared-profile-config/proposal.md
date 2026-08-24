## Why

The desktop app and the CLI need one durable source of personal profile data, but neither currently reads a usable shared profile. A versioned profile contract now prevents the two applications from drifting as tray and CLI features are added.

## What Changes

- Establish `~/.config/live_progress/profile.toml` as the canonical shared profile location.
- Define and implement the versioned profile fields: birthday, optional gender, and canonical nation name.
- Validate profile data at load and save boundaries, including valid non-future birthdays and nations that resolve to the life-expectancy dataset.
- Resolve country input consistently: exact canonical names take precedence, otherwise fuzzy matches select a deterministic canonical name for persistence.
- **BREAKING**: Replace the desktop app's unimplemented `~/.life_progress/config.toml` initialization check with the shared profile contract.

## Capabilities

### New Capabilities
- `shared-profile-configuration`: Versioned profile persistence and validation shared by the desktop app and consumable by the CLI.
- `nation-resolution`: Deterministic exact and fuzzy country resolution that persists canonical dataset names.

### Modified Capabilities
- None.

## Impact

- Affected code: `src-tauri/src/lib.rs`, the local `life-progress-core` crate, and configuration dependencies.
- Shared-data impact: the CLI will consume the same public TOML contract in follow-up work; it is not modified by this repository-scoped change.
- Out of scope: settings UI, onboarding, display preferences, and tray progress rendering.
