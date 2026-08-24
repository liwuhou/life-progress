## Context

See proposal.md for motivation. The desktop app currently only checks an unimplemented legacy configuration path. `life-progress-core` already owns birthday parsing, gender, life-expectancy data lookup, and fuzzy nation search; the CLI has its own future-facing configuration flags but does not persist a profile. The profile contract must be usable by both without coupling either application to tray or UI settings.

## Goals / Non-Goals

**Goals:**
- Put the profile data model, TOML serialization, validation, and nation normalization in `life-progress-core`.
- Give desktop startup one explicit result for configured, absent, and invalid profiles.
- Produce a stable v1 file suitable for a future CLI reader.

**Non-Goals:**
- Changing the CLI repository or adding command-line profile management.
- Reading the old profile path, creating a settings UI, or persisting presentation preferences.
- Fetching or persisting life-expectancy data as part of profile writes.

## Decisions

### Core owns the shared profile contract

`life-progress-core` will expose a serializable `Profile` model and load/save functions for the canonical path. The model contains `schema_version`, `birthday`, optional `gender`, and canonical `nation`; display settings remain application-owned. This puts every future consumer behind one parser and validator instead of duplicating a TOML schema.

### Validate before persistence and after parsing

Birthday validation will use the domain parser and reject future dates. Gender is encoded as `male` or `female` when present. Nation input resolves through a new core resolver before a profile is created, while parsed profile nations must already be exact dataset names. Missing files are a distinct not-configured outcome; malformed or semantically invalid files are errors.

### Make nation resolution deterministic

The resolver checks canonical names first. It otherwise gathers fuzzy candidates, sorts by score descending then canonical name ascending, and returns the first candidate. Inputs with no candidates return an error; they never persist as free text or fall back to `Common`.

### Make writes durable and private

Profile writes create `~/.config/live_progress` as needed, serialize to a temporary file in that directory, then replace the destination atomically. On Unix the file is created with owner-only permissions. This prevents a partial profile from becoming the shared state and limits exposure of personal data.

## Risks / Trade-offs

- The repository can establish the contract but cannot make the separate CLI consume it; compatibility is preserved through the documented v1 TOML shape and core API.
- Fuzzy nation selection is intentionally convenient but can select an unexpected top match; callers that provide a choice list may display ranked candidates before saving.
- The legacy path is deliberately not migrated because it has never held a defined profile schema; treating arbitrary existing files as personal data would be unsafe.
