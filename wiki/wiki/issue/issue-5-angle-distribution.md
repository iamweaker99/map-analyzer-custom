---
type: issue
github: "#5"
status: open
updated: 2026-08-08
---
# Issue #5 — Intra-Pattern Angle Distribution for Short Patterns (Burst 3/4)

**GitHub:** #5 · OPEN · enhancement

## Context

Lift the prototyped angle computation (`proto_angle.rs`) into production: for each detected Burst(3) or Burst(4) pattern compute the interior angle at each internal node with the dot-product formula (acos of normalized dot product, degrees 0–180), then aggregate into a 15-degree bucket distribution (12 bins: 0–15, 15–30, …, 165–180) with companion statistics (mean, median, stddev). Doubles (Burst 2) have no interior angle by definition — spacing only, done in [[issue-3-intra-pattern-spacing]]. Per-object angle values are kept for timeline plotting alongside the aggregate; inter-pattern/transition angles are explicitly deferred to a future PRD.

The 15-degree bucket granularity and companion stats were validated during the prototype phase on real beatmaps (AngelMaker, YOASOBI) — the same prototype family referenced in [[forward-density]]'s design-evolution table ("15° bucket test family").

## State

Open, not implemented, blocked by #3. `proto_angle.rs` was not found in the current tree (`backend/src/bin/` contains only `prototype_spacing_demand.rs` and `prototype_sequence_motor.rs`) — the prototype to lift is absent or already cleaned up; verify before starting. No `intra_pattern_angles` section exists in the reading pipeline yet.

## Acceptance criteria (pointer — GitHub is authoritative)

- Backend: lift proto_angle logic into production — Burst(3)/Burst(4) only
- Wire into `reading::analyze()` output as an `intra_pattern_angles` section: 15-degree bucket distribution + mean/median/stddev + per-object time series
- Frontend: `IntraPatternAngleData` interface in `types.ts`, extend `ReadingResult`; angle distribution chart (12-bin) per pattern type in `ReadingProfile.tsx`
- Tests: reuse and extend proto_angle test patterns (straight line = 180°, right angle = 90°, reversal = 0°)
- Cleanup: remove temp proto_angle modules and prototype binaries once production code is verified

## Relations

- [[issue-3-intra-pattern-spacing]] (gh#3) — blocker; body: "Reuse the pattern segmentation infrastructure from Slice 2 (Intra-Pattern Spacing) — not duplicating that work"; #3's prefactor (note index ranges per pattern) is the prerequisite for computing angles on the correct notes

## Quirk

- `aim_control` already emits an `angle_distribution` field (`backend/src/analysis/aim_control/mod.rs:82`) — unrelated to this issue's intra-pattern angles; do not conflate the two when wiring frontend output.

## Links

[[reading-hub]] · [[reading-analysis]] · [[forward-density]] · [[spacing-demand]]
