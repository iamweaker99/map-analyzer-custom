---
type: research
status: implemented
updated: 2026-08-08
sources: [Temp/handoff-three-variable-prototype_10.md, Temp/handoff-rhythm-discontinuity-integration_11.md, Temp/temporal-vs-rhythm-illustrated.md, Prototyping/rhythm_rejection - Signal feat. Such (Mameyudoufu Remix) (NekoShabeta) [Disturbance].csv, Prototyping/temporal_rejection - Signal feat. Such (Mameyudoufu Remix) (NekoShabeta) [Disturbance].csv, backend/src/analysis/finger_control/rhythm_segmentation.rs, backend/src/analysis/finger_control/snap_filter.rs, backend/src/analysis/finger_control/mod.rs, backend/src/analysis/finger_control/patterns.rs, backend/src/analysis/reading/mod.rs]
---
# Research: Rhythm Segmentation (Motor-Continuity Pattern Boundaries)

Rhythm segmentation splits a beatmap's hit objects into **patterns** for the reading pipeline using two signals: a ½-beat **gap threshold** and **rhythm discontinuity R** = |log₂(snap₂/snap₁)|. It comes from a three-variable prototype (Temporal / Velocity / Rhythm discontinuity) in which **Rhythm won** as the primary segmentation signal; the BPM snap infrastructure it required (per-timing-point lookup, ±10% adaptive tolerance) is now shared by all of `finger_control`'s snap analysis. R beats Temporal because it normalizes BPM changes out and still fires where a BPM change sits between two note pairs (T is blind there).

## The three-variable prototype

`Temp/handoff-three-variable-prototype_10.md` (supersedes the four-variable handoff `_9.md`) locked in exactly **three variables** for motor-continuity segmentation (handoff_10:12):

- **Temporal Discontinuity (Variable 1)** — *Signed* (faster vs slower matters). Δt, ratio of Δt, and log-ratio of Δt were to be scatter-plotted to pick the most informative representation; representation never decided (handoff_10:17-20). Blocked behind the BPM infra fix, then superseded by Rhythm.
- **Velocity Discontinuity (Variable 2)** — Euclidean distance `(x1,y1)→(x2,y2)`; positions normalized by CS circle diameter (`108.8 − 8.96·CS`, same formula as the rest of the codebase) (handoff_10:22-25). Not implemented; deferred to combine with Direction/Angle for intra-pattern segmentation.
- **Rhythm Discontinuity (Variable 4** — numbered as in the handoff, leftover from the four-variable scheme**) — Ratio-based: `log2(snap2/snap1)` as a continuous signed value; **note-pair-level** granularity (pattern-level aggregation "later") (handoff_10:27-30).
- Direction/angle (the four-variable scheme's Variable 3) — deferred entirely to `Temp/angle-direction-research-notes.md` (handoff_10:13, 42-46).

The prototype's critical path was the **BPM snap fix** (handoff_10:32-36): find the active uninherited (red line) timing point at each object via `rosu_pp::Beatmap::timing_points`; replace the global `map.bpm()` + ±12 ms tolerance with per-timing-point beat_len + **±10% adaptive tolerance** (`beat_len × fraction × 0.10`); export per-adjacent-pair snap; remove the dead burst-histogram computation.

## Discontinuity definitions: R vs T

- **R** = |log₂(snap₂/snap₁)|; each pair's snap is `Δt / beat_len` using the timing point active at the **second** note's timestamp (temporal-vs-rhythm-illustrated.md:7-8; snap_filter.rs:69-73). Continuous fraction, not the quantized label.
- **T** = |log₂(Δt₂/Δt₁)| — pure time ratio, no BPM/timing-point information (rhythm_segmentation.rs:172-175).

Worked examples (temporal-vs-rhythm-illustrated.md):

| Example | T | R | Meaning |
|---|---|---|---|
| 1. Single BPM, 1/4→1/2 | +1.00 | +1.00 | Both detect the boundary |
| 2. BPM change, snap constant | spikes at the transition note | same | — |
| 3. BPM change **between** two pairs | **0.00 flat** | **+0.585** | Only R sees it; T−R = −0.585 = log₂(333/500), a pure BPM-change detector |

Recommendation (temporal-vs-rhythm-illustrated.md:139): **use R as the primary signal**; T is only a fallback when timing points are unavailable. T−R was archived as a BPM-change detector (handoff_11:14; `Temp/temporal-and-tr-deferred-reference.md`).

## Prototype results

Two CSVs in `Prototyping/` (both named "Signal feat. Such (Mameyudoufu Remix) (NekoShabeta) [Disturbance]"), produced by the comparison-study binary `bin/prototype_segmentation.rs` (deleted after serving its purpose, handoff_11:48). 175 BPM throughout (beat_len 342.8571 ms in every row), **1565 adjacent note pairs**.

- **T ≡ R on single-BPM maps**: the two CSVs' `log2_ratio` columns are numerically identical in all 1565 rows (row-by-row diff = 0). Prototype-scale confirmation of rhythm_segmentation.rs:35.
- **|R| distribution** (log2 ratio): 1220 rows in [0, 0.05), 2 in [0.05, 0.1), 0 in [0.1, 0.3), 2 in [0.3, 0.5), 17 in [0.5, 0.6), 198 in [0.6, 1.1), 114 in [1.1, 2.1), 12 ≥ 2.1; max 6.98.
- **Threshold 0.5 → 341 boundaries (21.8%), 1224 non-boundaries (78.2%)**. Only 4 of 1565 rows fall in (0.05, 0.5) — the 0.5 threshold sits in a nearly empty band between "same snap" (≈0) and "snap change" (≥0.5) on this map.
- Snap fractions are continuous with jitter around clean values (0.2479, 0.2508, 0.4987, 0.7496, 1.0004). Note: **3/4 (0.75) is not in the SNAPS label table** (snap_filter.rs:14-21), so those pairs fall to off-grid handling.
- Threshold coverage: catches 1/4→1/2 (2×, R=1.0), 1/4→1/6 (1.5×, R=0.585), 1/8→1/4 (2×); **misses 1/4→1/3 (1.33×, R≈0.415)** (rhythm_segmentation.rs:28-29).

## Integration (handoff_11) and what shipped

Prototype → production integration (`Temp/handoff-rhythm-discontinuity-integration_11.md`): Rhythm chosen as primary; T rejected (false boundaries on multi-BPM maps); Velocity deferred; BPM infra (per-timing-point lookup + ±10% tolerance) validated and merged; `rhythm_segmentation` module created and registered (mod.rs:7); reading pipeline switched to it (reading/mod.rs:35); now-dead `extract_pattern_indices` removed from patterns.rs.

Segmentation algorithm (rhythm_segmentation.rs:124-168): boundary array from (1) gap > `beat_len/2 + 10 ms` at the second note (line 145), (2) R > 0.5 at middle note (lines 153-164); then `group_into_patterns` classifies: ≥7 notes = Stream, ≥2 = Burst(n), singleton slider = Slider, else Jump (rhythm_segmentation.rs:61-69).

### Prototype vs production status

| Item | Prototype spec / handoff | Shipped code (verified 2026-08-08) |
|---|---|---|
| R formula | `log2(snap2/snap1)` signed, pair-level (handoff_10:28-30) | Same, with abs + non-finite guards — rhythm_segmentation.rs:156-157 |
| R_THRESHOLD | 0.5 (handoff_11:36-38) | `const R_THRESHOLD: f64 = 0.5`, "Tune if needed" — rhythm_segmentation.rs:25-30; tuning vs in-game editor still open (handoff_11:73) |
| T_THRESHOLD | comparison-only | 0.5 — rhythm_segmentation.rs:32-36; `extract_pattern_indices_temporal` shipped but used only by prototype_sequence_motor.rs:125 |
| BPM source | per-timing-point lookup (handoff_10:33) | `timing_point_at` binary search — snap_filter.rs:29-34 |
| Snap tolerance | ±10% adaptive, "tune the percentage afterwards" (handoff_10:34, 52) | `TOLERANCE = 0.10` — snap_filter.rs:24, 40-48; the promised ±12 ms off-grid comparison never documented |
| Pair-level snap export | per-adjacent-pair (handoff_10:35) | `note_pair_snaps` — snap_filter.rs:58-77 |
| Gap boundary | ½ beat (handoff_11:35) | `beat_len/2 + 10.0 ms` — rhythm_segmentation.rs:145 |
| Velocity discontinuity | spec'd (handoff_10:22-25) | **Not implemented** — deferred with Direction (handoff_11:13, 75) |
| Temporal as primary | prototype candidate (handoff_10:17-20) | Rejected; R wins (handoff_11:11-13) |
| Dead burst histogram | remove (handoff_10:36-37, 70) | snap_filter.rs:83 returns empty HashMap; **mod.rs:39-45 `bursts.clear()` + repopulate still present** (see Contradictions) |
| 3-signal visualization | 3 synchronized line charts (handoff_10:80, 99) | Not built — `[gap]` |
| Prototype files | — (handoff_11:40-49) | Deleted: `reading/snapper.rs`, `discontinuity_{temporal,velocity,rhythm}.rs`, `proto_angle.rs`, `prototype_segmentation.rs` |

## Two parallel pattern-classification paths

Two live segmentation paths now produce overlapping output from the same beatmaps; their divergence is **untested** (index.md gap "2 parallel pattern-classification paths").

- **Path A — `finger_control::patterns::extract_patterns`** (old, still in `finger_control::analyze`): global `map.bpm()` (patterns.rs:56), gap `ms_per_beat/2 + 10` (patterns.rs:57), `identify_snap` with the **global** beat_len (patterns.rs:97, 116), slider-swallowing into bursts within a 25 px proximity threshold (patterns.rs:58, 81-93). Caller: `finger_control::analyze` (mod.rs:34) → transitions matrix (mod.rs:35), timeline (mod.rs:50). Returns `Vec<Pattern>`, no index ranges.
- **Path B — `rhythm_segmentation::extract_pattern_indices`** (new, drives reading): per-timing-point BPM, ±10% tolerance, gap + R boundaries; also `extract_pattern_indices_temporal` (rhythm_segmentation.rs:181). Caller: `reading::analyze` (reading/mod.rs:35) → `sequence_motor::analyze_patterns` (reading/mod.rs:44). Returns `Vec<(Pattern, Range<usize>)>`.
- Both emit the **same `Pattern` type** from the shared enum (patterns.rs:5-10; rhythm_segmentation.rs:22) — structurally interchangeable outputs. rhythm_segmentation.rs:10-12 documents that it "replaces the old `patterns::extract_pattern_indices` approach" (fixed gap + global BPM).
- The old `extract_pattern_indices` was removed from patterns.rs (handoff_11:28); only `extract_patterns` remains there.
- Handoff's own open item: "Replace `finger_control::patterns` entirely … Could be migrated to `rhythm_segmentation` later" (handoff_11:76).
- Prototype binaries all use Path B: prototype_spacing_demand.rs:122; prototype_sequence_motor.rs:125-127.

## Contradictions

1. **"Catches ≥1.4×" is off by a hair**: handoff_11:38 and rhythm_segmentation.rs:27 say threshold 0.5 catches "snap ratio changes ≥1.4×", but log₂(1.4) = 0.485 < 0.5; 0.5 actually catches ratios ≥ ~1.414×. Cosmetic fuzz repeated in both handoff and code comment.
2. **Dead-burst-removal half-done**: handoff_10:70 says remove "the `bursts.clear()` + repopulate pattern" in mod.rs — but mod.rs:39-45 still contains exactly that block (only the snap_filter.rs side was completed: empty HashMap at snap_filter.rs:83; handoff_11:17 claims only the snap_filter part).
3. **Plan vs execution sequencing**: handoff_10:20 says Temporal "not yet implemented", but handoff_11:44 reports a prototyped `reading/discontinuity_temporal.rs` (archived) and the CSVs contain full T signals — handoff_10 is the plan, handoff_11 the execution report; not a live contradiction, flagged for provenance.
4. **Stale docstring**: sequence_motor.rs:172 still cites `finger_control::patterns::extract_pattern_indices` as its pattern source while the live path is rhythm_segmentation (reading/mod.rs:35) — already flagged in [[sequence-motor]].
5. **Minor**: handoff_10:36 cites the dead burst histogram at "snap_filter.rs:49-64"; no such code exists at those lines in the current file (removed; file renumbered).

## Open questions / `[gap]`s

- **R_THRESHOLD tuning**: run against several beatmaps and compare with in-game editor (handoff_11:73). Only one map's signal data survives on disk (the two CSVs).
- **±10% tolerance never tuned**: off-grid-rate comparison vs the old ±12 ms never documented (handoff_10:52) → `[gap]`.
- **Path A vs Path B divergence untested** (index.md gap); whether to replace `finger_control::patterns` entirely is undecided (handoff_11:76).
- **1/4→1/3 snap transitions missed** by R_THRESHOLD 0.5 (rhythm_segmentation.rs:29).
- **Velocity + Direction intra-pattern segmentation** — framework in `Temp/velocity-spatial-discontinuity-framework.md`, needs its own session (handoff_11:75).
- **Slider detection in segmentation** deferred to the angle session; singletons currently classified only by type (handoff_11:74).
- **Three-signal dataviz** (boundary-distribution visualization) never built (handoff_10:80; handoff_11:82).
- **Frontend impact unassessed** — reading JSON now uses rhythm-based segmentation; the `sequence_motor` section output format is unchanged, but "Frontend may need updates if pattern counts changed noticeably" (handoff_11:77) → `[gap]`; see [[finger-control-profile]] / [[reading-profile]].
- **T−R BPM-change detector** archived (handoff_11:14) — no session planned to use it.

Related: [[finger-control]] · [[reading-analysis]] · [[sequence-motor]] · [[spacing-demand]] · [[reading-hub]]
