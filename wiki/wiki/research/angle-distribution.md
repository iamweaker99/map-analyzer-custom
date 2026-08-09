---
type: research
status: designed
updated: 2026-08-08
sources: [Temp/reading-handoff-02-angle-distribution.md, Temp/reading-handoff-03-angle-categories.md, Temp/prototype-discussion-01-angle-buckets.md, wiki/raw/reading-handoff-01-forward-density.md, wiki/wiki/research/forward-density.md, wiki/wiki/issue/issue-5-angle-distribution.md, wiki/wiki/module/reading-analysis.md, wiki/wiki/hubs/reading-hub.md]
---
# Research: Angle Distribution (Intra-Pattern)

Reading-side metric family: interior angle at the middle note of note triples, aggregated into a bucket distribution with summary stats. Inspired by the official osu! reading PR (ppy/osu#33196, `ReadingEvaluator.cs`) and the aim_control precedent. Not implemented; scoped in [[issue-5-angle-distribution]], blocked by [[issue-3-intra-pattern-spacing]].

## Design evolution
| Phase | Scope | Key decisions |
|---|---|---|
| Handoff #2 (raw angles) | Every consecutive 3-note window across the whole map | Dot-product interior angle at B, 0-180°; per-object + aggregate; no shape labels; pure geometry; NM only; separate from trajectory entropy (per reading-handoff-02) |
| Handoff #3 (categories) | Same triples + 4-5-note windows | Adds a description layer: linear flow (~180°), curved flow (obtuse), orthogonal (~90°), sharp/acute (<90°), reversal (~0°); multi-window: zig-zag (cross-product sign flips), steady curve, straight series (per reading-handoff-03) |
| Prototype discussion (2026-07-16) | Triples sliding window | Mean/median/stddev settled; entropy orthogonality confirmed; per-node timeline is a feature; pattern-level analysis deferred until the basic metric is validated; bucket size 15° vs 10° open (per prototype-discussion-01) |
| Issue #5 scope (2026-08-08) | Burst(3)/Burst(4) patterns only | 12 bins of 15° (0-15, 15-30, …, 165-180) + mean/median/stddev + per-object series; section name `intra_pattern_angles`; doubles have no interior angle (spacing only, via #3); inter-pattern/transition angles deferred to a future PRD (per issue-5) |

## The metric
For each consecutive triple A-B-C, interior angle at B (degrees, 0-180):
```
vector_AB = (B.x - A.x, B.y - A.y)
vector_BC = (C.x - B.x, C.y - B.y)
angle = arccos((AB·BC) / (|AB|·|BC|))
```
Fixed rules from handoff #2: no predetermined shape labels ("triangle"/"square") — report values and let the consumer interpret; separate from trajectory entropy (entropy measures *change* in angles; this measures the angle *magnitude* itself); pure geometry (no time weighting, no mod factors); NM only.

## Bucket design trail
- aim_control precedent: 0-45° linear / 45-90° wide / 90-135° acute / 135-180° snap_backs (aim_control/mod.rs); vectors.rs classifies parallel/orthogonal/anti_symmetric via dot products, and uses cross-product sign changes ("chirps") for zig-zag detection (per handoff-02, handoff-03).
- Handoff #2 left ranges open ("follow aim_control or reading-specific?"). Handoff #3 proposed categories with example thresholds (e.g. orthogonal = 75-105° or 80-100°?) — undecided.
- Prototype discussion (2026-07-16) framed bucket size as 15° vs 10°: 30° too coarse (would hide AngelMaker's 0-30° spike), 1° too fine (180 buckets, noise), 15° = 12 buckets, 10° = 18 buckets; next step was to test both on AngelMaker and YOASOBI.
- **Resolved: 15° buckets (12 bins)** — issue #5 records that "the 15-degree bucket granularity and companion stats were validated during the prototype phase on real beatmaps (AngelMaker, YOASOBI)". Quantiles (equal-count adaptive groups) remain a possible future alternative, not urgent (per prototype-discussion-01).
- Summary stats settle the interpretation: mean = typical turn sharpness, median = what most angles look like, stddev = consistency; mean-median gap separates rare-extreme spikes (AngelMaker mean 56°, median 24°) from even distributions (YOASOBI mean 77°, median 78°) (per prototype-discussion-01).

## Relationship to trajectory entropy
Prototype finding (2026-07-16): raw angles and trajectory entropy measure different things — "How sharp is each individual turn?" (a square is ~90° regardless of traversal order) vs "How predictable is the turning pattern?" (clockwise square = low entropy, N-order same square = high entropy). **Confirmed no significant overlap** — both kept (per prototype-discussion-01). Handoff #3's usefulness test (tech vs flow vs jump maps must look meaningfully different; compare against trajectory's linear/mild_sharp_kinks/spaghetti) was the acceptance framing.

## Relationship to forward density
Sibling metric in the same reading prototype family: [[forward-density]]'s design-evolution table cites the "15° bucket test family" as the same prototype run that settled forward density's 1000 ms window (per forward-density). Both are NM-only, raw-value metrics designed as *siblings* of existing reading JSON sections, not replacements ([[Data-Philosophy]] via reading-hub).

## Place in the reading pipeline
- Scope change: whole-map triples (handoff #2) narrowed to **intra-pattern** Burst(3)/Burst(4) after pattern-level analysis was deferred at prototype time ("need to segment the map by pattern boundaries first" — prototype-discussion-01) and re-adopted via the pattern-segmentation infra from [[issue-3-intra-pattern-spacing]] (note index ranges per pattern = the #3 prefactor that angles depend on; issue-5).
- Planned output: `intra_pattern_angles` section in `reading::analyze()` JSON — 12-bin distribution + mean/median/stddev + per-object time series (issue-5).
- Quirk to respect: aim_control already emits an `angle_distribution` field (aim_control/mod.rs:82) — unrelated to intra-pattern angles; do not conflate (issue-5).
- Not in the pipeline yet: `reading::analyze()` currently emits density/trajectory/traps/topography/sequence_motor only (per [[reading-analysis]] module page).

## Superseded / not carried forward
- Whole-map consecutive-triple scope → replaced by intra-pattern Burst(3)/Burst(4) scope (issue-5, later than handoff #2).
- Handoff #3's category labels (linear/curved/orthogonal/anti-symmetry, both approaches A and B) are **absent from issue #5's scope** — the agreed scope is a plain bucket distribution + stats. No explicit decision document records the drop.
- 10° buckets (18 bins) — not adopted; 15° won.
- Per official PR influence: `getConstantAngleNerfFactor` (repeated angles = predictable = nerfed) and the velocity factor were the motivation for angle categorization (handoff-03), but no repetition/velocity factors are in the agreed scope.

## Status
Open, not implemented, blocked by #3 (issue-5; reading-hub). **Prototype code missing**: prototype-discussion-01 (2026-07-16) cites `backend/src/analysis/reading/proto_angle.rs` and `backend/src/bin/prototype_reading_angle.rs`; neither exists in the tree as of 2026-08-08 (issue-5 report; verified by glob — reading/ holds only mod/visuals/density/trajectory/traps/strain/sequence_motor, bin/ holds only prototype_spacing_demand + prototype_sequence_motor). Test patterns to reuse: straight line = 180°, right angle = 90°, reversal = 0° (issue-5).

## Open questions
- Timeline window for the per-node angle series: existing trajectory_timeline uses 5 s windows, flagged too coarse; ~1 s candidate "to test later" (prototype-discussion-01) — unresolved.
- Angle analysis for non-burst patterns (streams, jumps per finger_control classes) was the stated future goal when pattern-awareness was deferred (prototype-discussion-01); issue-5 scopes bursts only.
- Whether handoff #3's category layer returns in a later phase.

## Related
[[forward-density]] · [[reading-analysis]] · [[reading-hub]] · [[Analysis-Type]] · [[issue-5-angle-distribution]] · [[issue-3-intra-pattern-spacing]]
