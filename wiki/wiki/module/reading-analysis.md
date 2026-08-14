---
type: module
status: stale
updated: 2026-08-11
---
# Module: reading-analysis

> ⚠️ **STALE** — production port landed 2026-08-11 (commit `15d15ab`): R/T threshold **0.35**, `finger_control::patterns` deleted, finger_control + reading rewired, `canary_segmentation.rs` added. Content below reflects the pre-port state. See [[rhythm-segmentation]] and [[segmentation-unification]].

Purpose: visual reading difficulty of a beatmap — how hard notes are to *see and process*, independent of physical execution. Entry point is `reading::analyze(map)` which returns one JSON object with all sections.

## Files
- `backend/src/analysis/reading/{mod, visuals, density, trajectory, traps, strain, sequence_motor}.rs`
- Prototype CLI: `backend/src/bin/prototype_sequence_motor.rs` (sibling: `prototype_spacing_demand.rs`)

## Pipeline & data flow (verified against code 2026-08-08)

`visuals` → `density` → `trajectory` → `traps` → `strain` → `sequence_motor`

1. **visuals** (`visuals.rs:27-57`) — `extract_visual_nodes(map)` converts hit objects to `VisualNode { start_time, end_time, fade_in_time, x, y, is_slider }`. `fade_in_time = start_time - preempt`; preempt from `ar_to_preempt` (visuals.rs:16-25): AR<5 → 1200+120·(5-AR), AR>5 → 1200-150·(AR-5), AR=5 → 1200. Circle geometry at mod.rs:12-14: `radius = 54.4 - 4.48·cs`, diameter = 2·radius, used to normalize all later metrics. Slider end_time currently defaults to start_time (visuals.rs:41-42, "future slider-clutter updates").
2. **density** (`density.rs:11-53`) — per note, counts every note visible at its hit moment (`fade_in_time <= t <= start_time`). Spatial chunking ("quadratic smoothing"): `effective = 1 + (raw-1)·spread` where spread = 1.0 if the bbox diagonal >= circle diameter (wide spread), else sqrt(diagonal/diameter) (stacks/overlaps count less). O(n²) — full node scan per node.
3. **trajectory** (`trajectory.rs:12-74`) — needs density for dynamic window sizing: window = `raw_objects` clamped [4,16]. "Cheese filter": spatial spread via max pairwise distance. Adaptive mean entropy over turning angles (mean of |Δ| between consecutive angle-change magnitudes, in degrees); `final_entropy = mean_entropy · spread_factor`. `is_spaghetti = min_pairwise_dist < diameter && final_entropy > 60` (overlap AND chaos).
4. **traps** (`traps.rs:13-49`) — needs BPM: inertia-reset threshold = 1.5 beats (`60000/bpm · 1.5`). Sliding 3-note windows; skip if `dt_curr > threshold` (break filter). `rhythmic_shock = clamp(dt_curr/dt_prev, 0, 3)`; `magnitude = shock · (distance/100)`. Emits a deceleration trap iff `magnitude > 1.5 && dt_curr > dt_prev`.
5. **strain** (`strain.rs:23-86`) — consumes density, trajectory, traps. Exponential decay with half-life 500 ms. `base_cost = 1 + effective_density·0.2 + (entropy/90)·0.5`, +2 if spaghetti, +3 if decel trap. Klines: 5 s OHLC candles (open/high/low/close/volume). Caller discards the strain points (`let (_strain_points, klines)`, mod.rs:27) — only candles survive.
6. **sequence_motor** (`sequence_motor.rs`) — per-pattern descriptors over pattern ranges from `super::finger_control::rhythm_segmentation::extract_pattern_indices` (mod.rs:35-44; rhythm-based "R" segmentation, default; the temporal "T" variant exists in finger_control but is not used by the pipeline). Header-marked PROTOTYPE but shipped in `analyze()` output. Metrics per pattern (≥2 notes; spacing normalized by diameter so CS doesn't change scores, sequence_motor.rs:10-13):
   - **MPA** (Motor Plan Adjustment) — mean absolute second difference of consecutive spacings; geometric instability; 0.0 if < 4 notes.
   - **MM** (Movement Magnitude) — RMS of spacings, in diameters; 0.0 if < 2 notes.
   - **SC** (Spacing Consistency) — coefficient of variation of spacings; 0.0 if < 3 notes.

## Output JSON sections (mod.rs:173-209)
- `summary`: `peak_strain` (≈p95 of kline highs — `floor(len·0.95)` index, not the max), `ar_preempt_ms`
- `density`: isolated/chunking/clutter/overload pct by rounded `effective_objects` buckets 0-2 / 3-5 / 6-8 / 9+
- `trajectory`: linear / mild_shifts / sharp_kinks / spaghetti pct — entropy <30 / <90 / ≥90; spaghetti overrides the others
- `trajectory_timeline`, `density_timeline`: same buckets re-aggregated into 5 s windows
- `traps`: `count`, `trap_index` (count/total_nodes·1000), `peak_magnitude`, `notable_traps` (top 5 by magnitude)
- `topography`: `{ klines }` — strain candles, not spatial topography (misnomer)
- `sequence_motor`: `timeline` [{time MM:SS, notes, mpa, mm, sc}] + per-metric `summary` {mean, max, p95}

## Quirks
- "topography" section holds only strain klines; the name implies spatial terrain.
- `peak_strain` is a 95th percentile, not the true peak.
- `is_deceleration_trap` is always `true` when a trap is emitted (traps.rs:41) — the field is constant, not a flag.
- Timeline `time` is MM:SS rounded (mod.rs:47-52); the prototype bin uses MM:SS.mmm.
- sequence_motor.rs:172 docstring cites `finger_control::patterns::extract_pattern_indices` — stale; the live path is `finger_control::rhythm_segmentation` (mod.rs:35).
- Early return `{ "error": "Not enough objects..." }` if a map yields zero visual nodes (mod.rs:18-20).

## Contracts / integration points
- reading depends on finger_control ([[finger-control]]) for pattern segmentation.
- New metrics must be **siblings** of existing sections, not replacements ([[Data-Philosophy]]).
- Prototype CLI `prototype_sequence_motor.rs`: loads a .osu, prints per-pattern MPA/MM/SC; flags `--json`, `--patterns` (all patterns incl. singletons, precise ms), `--T` (temporal segmentation) / `--R` (rhythm, default).

## Pending / planned
- [[forward-density]] — planned sibling section (Issue #4, unimplemented; see [[issue-4-forward-density]])
- [[spacing-demand]] — prototype state (`prototype_spacing_demand.rs`); frontend undecided
- Landing context: [[reading-hub]]

_Sources: code (read 2026-08-08), raw/prd-reading-analysis.md, handoffs_
