---
type: module
status: stable
updated: 2026-08-08
---
# Module: jumps

Purpose: quantify jump usage — share of objects involved in jumps, spacing distribution, consecutive-jump chains, and timing consistency.

## Files
- `backend/src/analysis/jumps.rs` — the analysis
- `backend/src/analysis/mod.rs` — `Movement` struct (mod.rs:10-14), `create_movements()` (mod.rs:20-37), `get_diameter()` (mod.rs:16-18)

## Pipeline / data flow
1. `create_movements()` walks consecutive hit objects: `time_gap = obj2.start_time - obj1.start_time`, `distance` = euclidean distance between positions. Only start times are used — slider body time is invisible to this module.
2. `get_diameter(cs) = 108.8 - 8.96*cs` (mod.rs:16-18).
3. A movement is a jump (jumps.rs:30) when `time_gap <= 60000/bpm` (one beat) AND (`time_gap > 1.5 × quarter-beat` OR `distance > 2.5*d`), and `distance > 0`.
4. Jumps accumulate into chains: a chain is a run of consecutive qualifying movements; any non-jump movement resets it (jumps.rs:42-46).
5. Output is flat, unweighted JSON ([[Data-Philosophy]]).

## Thresholds (d = circle diameter)
| Concept | Boundary |
|---|---|
| stream_threshold | `(60000/bpm/4) * 1.5` ms — 1.5 × quarter-beat (jumps.rs:6) |
| jump_rhythm_threshold | `60000/bpm` ms — one beat (jumps.rs:7) |
| narrow / moderate / wide / extreme | `< 2.0d` / `< 3.5d` / `< 5.0d` / `≥ 5.0d` (jumps.rs:37-40) |
| short / medium / long chains | `3-5` / `6-11` / `≥ 12` notes; chain count = consecutive jump movements + 1 (jumps.rs:18-27) |

## Output (serde_json Value, jumps.rs:56-67)
- `overall_confidence` = jump movements / total_obj — classification share
- `avg_spacing` = mean distance of jump movements
- `narrow/moderate/wide/extreme_count` + `*_dens` (count / total_obj)
- `max_jump_length`, `short_jumps` / `medium_jumps` / `long_jumps` (chain counts)
- `bpm_consistency` = `1.0 - CV` of jump time gaps, clamped ≥ 0 (jumps.rs:49-54)
- `circle_diameter`, `jump_density`

## Quirks
- `jump_density` and `overall_confidence` are the same value (j_cnt / total_obj) — duplicated key.
- Jumps are movements, not patterns: there is no notion of note placement/angles here (that lives in [[aim-control]]-adjacent work); a jump is purely "fast enough and far enough".
- Membership is the exact complement of [[streams]]: streams require `time_gap <= stream_threshold` AND `distance <= 2.5d`; jumps require the inverse OR-condition. The two modules partition the same movement list.

## Fits [[Analysis-Type]]
Jump is one of the six analysis dimensions; this module is the classification-layer analysis (quantity-based, lossy by design — [[Analysis-Type]] three layers). Rendered by the frontend as the **jump profile** card ([[jump-profile]]).

## Related
[[Analysis-Type]] · [[Data-Philosophy]] · [[streams]] · [[sliders]] · [[jump-profile]]

_Sources: code (read 2026-08-07)_
