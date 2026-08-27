## Why

The tray application now has a shared profile contract but no user-facing way to create or edit it. A focused settings surface is needed for first-run profile setup and control of the life-progress presentation.

## What Changes

- Add an on-demand Tauri WebView settings window, opened automatically when no valid profile exists and from the tray menu thereafter.
- Provide profile editing for birthday, optional gender, and country search, saving the canonical shared profile.
- Add display preferences for remaining versus elapsed life, tray style, and optional right-side title template with a live preview.
- Persist desktop-only display preferences separately from the shared profile.
- **BREAKING**: Evolve the tray-only contract from no WebViews to no persistent application window; the settings window is explicitly on-demand.

## Capabilities

### New Capabilities
- `profile-settings`: First-run and editable profile configuration through a native desktop settings surface.
- `tray-display-preferences`: Persistent selection of life metric, tray visual style, and title text presentation.

### Modified Capabilities
- `shared-profile-configuration`: Settings can create and replace validated shared profiles.
- `tauri-platform-migration`: Tray menu opens the on-demand settings surface without creating a persistent application window.

## Impact

- Affected code: Tauri configuration, tray runtime, Rust profile commands, static settings assets, and desktop preference storage.
- The project gains a small static WebView surface but does not add Node, React, or a frontend build server.
