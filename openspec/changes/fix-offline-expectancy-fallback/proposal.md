## Why

A cache miss combined with unavailable network access makes the current fallback depend on the process working directory. Installed desktop applications then reject otherwise valid profiles because bundled life-expectancy data cannot be found.

## What Changes

- Embed the default life-expectancy dataset in the crawler crate so fallback loading never depends on the current working directory.
- Preserve cached and successfully fetched data as preferred sources, while making offline fallback available to profile validation and lifetime calculations.
- Add clean-cache, offline regression coverage for canonical nation resolution and profile loading.

## Capabilities

### New Capabilities
- `offline-expectancy-fallback`: Reliable bundled life-expectancy data when cached and network sources are unavailable.

### Modified Capabilities
- `shared-profile-configuration`: Valid profiles remain loadable when the expectancy cache is absent and the network is unavailable.
- `nation-resolution`: Canonical nation resolution remains available from bundled expectancy data offline.

## Impact

- Affected code: `src-tauri/crates/lifespan-crawler/src/lib.rs`, its bundled dataset, and core profile tests.
- No user-profile schema or tray behavior changes.
