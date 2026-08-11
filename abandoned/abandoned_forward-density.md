# Abandoned: Forward-Looking Note Density

**Summary:** For each hit object, count how many upcoming notes fall within a short time window (prototyped at 1000ms, later tuned to 300ms). Captures "what's immediately coming up" cognitive load that the existing visual density (which counts currently-visible notes at the hit moment) misses.

**Status:** Abandoned after testing against real maps — the metric did not provide useful signal for the effort of maintaining it.

---

## Concept

The existing visual density counts how many notes are **currently visible on screen** at each hit moment. Forward density instead looks **ahead** in time: for note *N*, count how many notes *N+1, N+2, ...* fall within the next *W* milliseconds (inclusive).

This was intended to measure anticipatory reading load — how many notes the player has to mentally queue up while hitting the current object.

## Algorithm Design

- Sliding window per note: for each index `i`, scan forward `j >= i` while `nodes[j].start_time <= nodes[i].start_time + window_ms`
- **Inclusive boundary:** a note exactly `window_ms` later counts as within the window
- **Early break:** since notes are time-sorted, inner scan stops at the first note outside the window
- Worst-case O(n²) for dense maps where all notes fit in the window; O(n) average (break fires quickly in sparse sections)
- Tunable parameter: `window_ms` (f64)

## Data Shape

Per-object time series:
```
{ time: f64, forward_count: usize }
```

Bucketed into 4 tiers (same as visual density for comparability):
| Tier | Count range | Visual color |
|---|---|---|
| Isolated / Sparse | 0-2 | Emerald |
| Chunking / Moderate | 3-5 | Blue |
| Clutter / Dense | 6-8 | Yellow |
| Overload / Overload | 9+ | Red |

Summary stats: mean, median, stddev, peak, min.

## Frontend UX

- **Section name:** "Anticipatory Load"
- **Placement:** in the Reading profile card, between Visual Clutter and Trajectory Chaos
- **Layout:** Summary stats row (small muted text), then `StatBar` grid (same 4 bars as Visual Clutter), then optional timeline chart (single purple `Line` via Recharts, unlike Visual Clutter's 4-line chart)
- All wrapped in an optional check since the data may not exist on older analysis results

## Tuning History

- **1000ms** — original prototype. Produced avg ~10.8 on dense deathstream maps, ~5.6 on moderate maps. Bucket distribution was heavily skewed to Overload (9+) on dense maps, making it less discriminating.
- **300ms** — adjusted to focus on truly immediate upcoming notes. Tighter window, more responsive to local density changes.

The right window size was the key design lever — too wide collapses everything into one bucket, too narrow mirrors existing density. No size was found that added useful signal beyond what existing metrics provide.

## Why Abandoned

*Fill in specifics from your testing here. General assessment: after running against multiple real maps, the metric correlated too closely with existing density and BPM to justify its own maintenance surface (Rust module + JSON contract + frontend UI).*

## Test Maps Used for Validation

- **AngelMaker - A Dark Omen [Demonic Colossus]** — 1824 objects, dense deathstream map
- **YOASOBI - Yoru ni Kakeru [Collab Extra]** — 1097 objects, moderate difficulty

## Test Coverage (if revisited)

Unit tests were written for these scenarios (Rust, `forward_density.rs`):
- Empty input → empty output
- Single node → count = 1
- Three nodes within window → first sees all 3, last sees 1
- Boundary inclusive → node exactly at `window_ms` counts
- Outside window → only self-count
- Last objects in map → trailing notes correctly report 1
- Sparse map (notes 3s apart) → all report 1
