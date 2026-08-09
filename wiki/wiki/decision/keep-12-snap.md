---
type: decision
status: accepted
updated: 2026-08-06
---
# Decision: Include 1/2 snap patterns in spacing demand

**Date:** ~2026-07 · **Status:** accepted

## Context
The spacing demand prototype considered excluding 1/2 snap patterns to isolate rhythmic density. Tested on **AngelMaker — A Dark Omen** (220 BPM deathcore): excluding 1/2 snap reduced the scorable set from **22 → 2 patterns**.

## Decision
**Include 1/2 snap patterns.** Do not filter by snap label; do not pass the prototype's `exclude_snaps` flag when wiring into production.

## Rationale
The metric should measure the map's actual spacing variation, not filter by rhythmic density. 1/2 bursts on fast maps carry real spacing inconsistency that affects reading difficulty.

## Consequences
- More scorable patterns per map → more robust statistics on high-BPM maps
- Rhythm distinction is preserved by *reporting* snap alongside data (per [[Data-Philosophy]]), not by filtering it out

## Related
- [[spacing-demand]] · [[Data-Philosophy]]

_Sources: memory: spacing-demand-keep-12-snap (2026-07), raw/spacing-demand-handoff_5.md_
