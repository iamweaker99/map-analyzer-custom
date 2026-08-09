---
type: module
status: stable
updated: 2026-08-08
---
# Module: streams

Purpose: quantify stream usage — burst vs. stream structure, per-pattern spacing and velocity profiles, per-gap density distribution, and timing consistency.

## Files
- `backend/src/analysis/streams.rs` — the analysis
- Depends on `Movement` / `create_movements()` / `get_diameter()` from `backend/src/analysis/mod.rs`

## Pipeline / data flow
1. Same `Movement` input as [[jumps]] (consecutive hit-object start-time deltas, mod.rs:20-37).
2. `stream_threshold = (60000/bpm/4) * 1.5` ms — 1.5 × quarter-beat, identical constant to jumps.rs:6 (streams.rs:6).
3. A movement joins the current stream buffer if `time_gap <= stream_threshold` AND `distance <= 2.5d` AND `distance > 0` (streams.rs:18) — the 2.5d cap is the exact complement of the jump rule.
4. On a break, buffers with `>= 2` gaps (≥ 3 notes) are classified (streams.rs:22); only non-burst streams (≥ 5 notes) feed the spacing/velocity profiles (streams.rs:27-36).
5. Output is flat, unweighted JSON ([[Data-Philosophy]]).

## Thresholds (d = circle diameter)
| Concept | Boundary |
|---|---|
| bursts | note_count `3-4` (streams.rs:25) |
| short / medium / long / death | `5-12` / `13-24` / `25-48` / `≥ 49` notes (streams.rs:27) |
| pattern spacing (mean gap dist) | stack `< 0.5d` · overlap `< 1.0d` · spaced `< 2.0d` · extreme `≥ 2.0d` (streams.rs:29) |
| velocity (CV of gap dists) | steady `< 0.15` · variable `< 0.40` · dynamic `≥ 0.40` (streams.rs:32) |

## Output (serde_json Value, streams.rs:51-60)
- `overall_confidence` = `s_gaps / total_obj` — only gaps inside non-burst streams count
- `avg_stream_spacing` = Σ gap distances / s_gaps
- `s_stacked/overlapping/spaced/extreme_count` (per-pattern) + `s_stack_dens` etc. (per-gap, / total_obj)
- `v_steady/variable/dynamic_count` (per-pattern velocity profile)
- `total_stream_patterns` (short+med+long+death; excludes bursts), `bursts`, `short/medium/long/death_streams`, `max_stream_length`
- `bpm_consistency` = `1.0 - CV` of all stream time gaps, clamped ≥ 0 (streams.rs:44-49)
- `circle_diameter`

## Quirks
- `overall_confidence` undercounts at the classification layer: gaps inside bursts (3-4 note runs) never add to `s_gaps`, so stream share of the map is understated.
- `s_p_*` (per-pattern counts) and `s_n_*` (per-gap densities) share thresholds but measure different units; density bins count each gap individually.
- A high-spacing run (> 2.5d) always classifies as [[jumps]], never streams — the modules are a mutually exclusive partition of the same movement list.

## Fits [[Analysis-Type]]
Stream is one of the six analysis dimensions; classification-layer analysis (quantity-based, lossy by design — [[Analysis-Type]]). Rendered by the frontend as the **stream profile** card ([[stream-profile]]).

## Related
[[Analysis-Type]] · [[Data-Philosophy]] · [[jumps]] · [[sliders]] · [[stream-profile]]

_Sources: code (read 2026-08-07)_
