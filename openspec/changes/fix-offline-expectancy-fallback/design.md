## Context

See proposal.md for motivation. The crawler currently reads `default_expectancy.json` through `current_dir`, which is unavailable from arbitrary installed-app working directories. Core profile validation and nation resolution depend on crawler data and therefore inherit this startup failure.

## Goals / Non-Goals

**Goals:**
- Make default expectancy data available from the compiled crawler crate.
- Retain cache-first and successful-fetch behavior.
- Prove the fallback without reading the real user cache or making a network request.

**Non-Goals:**
- Changing the default dataset contents or cache location.
- Refreshing cached data, changing profile schema, or adding retry behavior.

## Decisions

### Embed the default dataset

The crawler will embed `default_expectancy.json` at compile time and deserialize those bytes when cached and fetched data are unavailable. This makes fallback independent of both process location and package installation layout.

### Inject data acquisition in tests

The fallback loader will be factored so tests can exercise a missing-cache, failed-fetch path without mutating `~/.config/live_progress` or depending on network state. Core tests will validate profile and nation behavior against the bundled result.

## Risks / Trade-offs

- The embedded JSON modestly increases binary size, but removes a launch-critical external file dependency.
- Bundled data can become stale; existing cache and refresh paths remain preferred to minimize that exposure.
