---
type: module
status: stale
updated: 2026-08-11
---
# Module: finger-control

> ⚠️ **STALE** — production port landed 2026-08-11 (commit `15d15ab`): `finger_control::patterns` (Path A) **deleted**, R/T threshold **0.35**, finger_control + reading rewired, `canary_segmentation.rs` added. Content below reflects the pre-port state (`patterns::extract_patterns()` no longer exists; R > 0.5 → 0.35). See [[rhythm-segmentation]] and [[segmentation-unification]].

Purpose: physical execution difficulty — snap precision, burst structure, pattern transitions. Registered in `analysis` (analysis/mod.rs:4); `analyze()` is served by the API for `fingercontrol` and `all` types (api/get/beatmap.rs:260, 284).

## Submodules (registered in mod.rs:5-9)
- `snap_filter` — snap identification + foundation analysis (snap distribution, off-grid notes)
- `patterns` — `extract_patterns()` action-pattern list
- `rhythm_segmentation` — R/T-based pattern-range segmentation for the reading pipeline
- `transitions` — transition matrices (BPM, top, rhythmic resets, delta groups)
- `timeline` — per-time-point curve data

**On disk but NOT registered** (no `pub mod`, nothing calls them — orphaned, never compiled):
- `complexity.rs` — `calculate_complexity()`: groups notes by <200ms gaps, returns burst-size variance score ×10 + even/odd ratio (complexity.rs:3-44)
- `morphology.rs` — `calculate_morphology_index()`: circle↔non-circle type-switch count / total objects ×10 (morphology.rs:4-25)

## Output contract
`FingerControlAnalysis` (mod.rs:14-23): `beatmap_md5, overall_confidence, snap_distribution, burst_histogram, off_grid_details, off_grid_buckets, transition_matrix, timeline`. `SnapBucket {label, percentage}` (mod.rs:27-30).

## Pipeline (mod.rs:32-71)
1. `snap_filter::analyze_foundation(map)` → snap counts, off-grid notes, 10 buckets
2. `patterns::extract_patterns(map)` → action pattern list
3. `transitions::analyze(&pattern_list)` → transition matrix
4. Burst histogram: the one from step 1 is empty; it is cleared and repopulated **strictly from the pattern list** (`Burst(n)` variants only, mod.rs:39-45)
5. `timeline::generate_timeline(&pattern_list, map_duration)`
6. `overall_confidence` = technical density (see quirks)

## snap_filter
- Snap labels 1/1, 1/2, 1/3, 1/4, 1/6, 1/8 with adaptive ±10% tolerance of the target delta (snap_filter.rs:14-24, 40-49); no match → off-grid
- Per-timing-point BPM lookup via binary search `timing_point_at()`, 500ms (120 BPM) fallback (snap_filter.rs:29-34, 98-99)
- Off-grid notes recorded as `OffGridNote{time, delta}` and bucketed into 10 time-quantile buckets by relative position (snap_filter.rs:104-115)
- `note_pair_snaps()`: continuous snap fractions (delta/beat_len) per adjacent pair with active beat_len — consumed by rhythm_segmentation (snap_filter.rs:58-77)

## patterns::extract_patterns
- Groups consecutive circles while delta ≤ beat_len/2 + 10ms (patterns.rs:56-57)
- A trailing slider within the gap threshold AND ≤25px of the last circle is "swallowed" into the burst (patterns.rs:80-93)
- count ≥ 7 → Stream, 2-6 → Burst(n); isolated circle → Jump, isolated slider → Slider (patterns.rs:100, 122)
- `PatternType::as_str()`: "Jump" / "Slider" / **"{n}n Burst"** (e.g. "3n Burst") / "Stream" (patterns.rs:13-20)
- Snap: `identify_snap(avg_delta)` with "Unstable" fallback; last object gets "End" (patterns.rs:97-98, 118-120)
- `note_count()`: Jump/Slider → 1, Burst(n) → n, **Stream hardcoded → 7** (patterns.rs:22-28)

## rhythm_segmentation (consumed by reading)
- `extract_pattern_indices(map)` — R-based: gap threshold (½ beat + 10ms) + rhythm discontinuity R = |log₂(snap₂/snap₁)| > 0.5 → boundary at the middle note (rhythm_segmentation.rs:30, 140-168). R normalizes for BPM changes via per-timing-point snaps
- `extract_pattern_indices_temporal(map)` — T-based: T = |log₂(Δt₂/Δt₁)| > 0.5, pure time ratios, no BPM info; identical to R on single-BPM maps, false boundaries at BPM transitions (rhythm_segmentation.rs:181-222)
- Shared `group_into_patterns()` (rhythm_segmentation.rs:45-113) classifies Stream≥7 / Burst≥2 / Slider / Jump with its own snap labeling — a **second classification path parallel to `extract_patterns()`**
- Reading's sequence motor consumes the R-based variant (reading/mod.rs:35)

## transitions
- `TransitionMatrix` (transitions.rs:6-33): `bpm_transitions` + categorized `bpm_ordinary/minor/major`, `top_transitions`, `rhythmic_resets`, `delta_groups` (0-3), `category_counts` (odd/even parities)
- `get_bpm_category()` sorts the snap pair, maps to Ordinary/Minor/Major, defaults unknown/extreme to Major (transitions.rs:36-45)
- Labels like "Jump (1/2) <-> Stream (1/4)"; top-10 lists by percentage of total transitions ×100 (transitions.rs:85-89, 122-131)

## timeline
- `TimelinePoint`: `pattern_sma`, `bpm_sma`, `bpm_ordinary/minor/major_sma`, `note_delta_0_cons/reset_sma`, `note_delta_1..3_sma` (timeline.rs:6-18)
- 1000ms steps over the full map duration; window = max(20, patterns/40) objects; points with no pattern within a 5s buffer emit a zeroed point (timeline.rs:24-28, 44-47)

## Quirks
- `overall_confidence` is actually **technical density** (share of non-1/1, non-1/2 snaps) — name kept as-is (mod.rs:52-56, 64)
- Stale doc comments: rhythm_segmentation.rs:10,122 and sequence_motor.rs:172 reference `patterns::extract_pattern_indices`, which no longer exists in patterns.rs (the function moved to rhythm_segmentation)
- Dev leftovers in mod.rs: "// NEW: Register timeline" and "// REMOVED: The problematic recursive call line was here" (mod.rs:9, 37)
- `analyze_foundation` computes a burst histogram that is always empty and immediately discarded (snap_filter.rs:83; mod.rs:39-40)

## Integration points
- Provides pattern ranges to reading's sequence motor via `rhythm_segmentation::extract_pattern_indices` ([[reading-analysis]])
- Same segmentation output drives spacing-demand prototyping (prototype_spacing_demand.rs:122) and the keep-1/2-snap filtering decision ([[spacing-demand]], [[keep-12-snap]])
- `extract_patterns()` remains the intended shared pattern source for intra-pattern metrics (PRD scope: Burst 2/3/4) — see [[issue-4-forward-density]]
- Profile aggregation across maps lives in [[finger-control-profile]]
- Naming stance: technical density labeled "confidence" is a deliberate data-philosophy tradeoff ([[Data-Philosophy]])

## Files
`backend/src/analysis/finger_control/{mod, snap_filter, patterns, rhythm_segmentation, transitions, timeline, complexity, morphology}.rs` — first six are registered and compiled; `complexity.rs` and `morphology.rs` exist but are orphaned (no `pub mod`).

_Sources: code (read 2026-08-08), raw/prd-reading-analysis.md_
