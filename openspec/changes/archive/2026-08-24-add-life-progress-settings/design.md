## Context

See proposal.md for motivation. The app is now Tauri 2, windowless at normal startup, and owns a shared profile at `~/.config/life_progress/profile.toml`. The existing static assets directory can host a small settings surface without adding a frontend build system.

## Goals / Non-Goals

**Goals:**
- Add one on-demand settings window for both first-run and later editing.
- Keep profile validation, persistence, and nation resolution in Rust.
- Persist display preferences separately and preview them before saving.
- Update tray presentation immediately when settings change.

**Non-Goals:**
- Adding a persistent dashboard, browser server, Node dependencies, or a JavaScript framework.
- Sharing desktop display preferences with the CLI.
- Adding synchronization or multiple user profiles.

## Decisions

### Static settings WebView

The app will bundle static HTML, CSS, and minimal browser JavaScript under `src-tauri/assets`. Rust creates a single window with the stable `settings` label only when onboarding or a tray action requests it; subsequent requests focus that window. Closing it hides or destroys the window without changing the tray process.

### Rust command boundary

The WebView uses narrow commands to load settings, search nations, validate and save profiles, and load and save display preferences. Commands return structured validation errors; the WebView never reads profile files directly.

### Separate desktop preferences

Desktop-only preferences live in `~/.config/life_progress/desktop.toml`. The schema stores the selected life metric, display style, title visibility, and title template. Profile writes remain exclusively in `profile.toml`.

### Single display model

A Rust display model derives remaining and elapsed percentages/days from the configured profile. Both the tray renderer and the settings preview consume this model. The tray updates after a successful preference or profile save; colors are based on remaining lifetime so low remaining time is consistently signaled regardless of fill direction.

### Template values

Titles support `{mode}`, `{percent}`, and `{days}`. Unknown placeholders remain literal, and rendered titles are bounded by visible-character count before assignment to the system tray.

## Risks / Trade-offs

- A WebView expands the tray-only runtime surface and requires minimal Tauri command permissions, but avoids platform-specific AppKit form code.
- Static JavaScript has no type sharing with Rust; command payloads remain small and explicit to limit drift.
- Dynamic tray images require platform-specific visual verification in addition to Rust tests.
