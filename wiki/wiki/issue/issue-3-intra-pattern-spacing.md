---
type: issue
github: "#3"
status: open
updated: 2026-08-08
---
# Issue #3 — Intra-Pattern Spacing for Short Patterns (Burst 2/3/4)

**GitHub:** #3 · OPEN · enhancement

## Context

Add intra-pattern spacing metrics to the reading analysis: segment the beatmap with `finger_control::patterns::extract_patterns()` and compute spacing statistics per detected short pattern, limited to Burst(2), Burst(3), Burst(4). Doubles (2 notes) report spacing only — no interior angle exists for a note pair. Triples and 4-note patterns report mean, stddev, min, max between consecutive pairs. Spacing uses Euclidean distance between consecutive note positions (same formula as `Movement::distance`); the rhythmic snap (1/2, 1/4, …) from pattern detection carries through. Angle and spacing are reported alongside each other in the final output (see the follow-up, [[issue-5-angle-distribution]]).

Prefactor required: `extract_patterns()` returns pattern metadata (type, time, snap) but not the note indices each pattern covers — extend it or add a parallel `extract_pattern_indices()` so spacing (and later angles) can be computed on the correct notes.

## State

Open, not implemented, not blocked. No `intra_pattern` module exists in `backend/src/` (glob, 2026-08-08). The prefactor appears already satisfied by uncommitted WIP: `extract_pattern_indices()` exists in `backend/src/analysis/finger_control/rhythm_segmentation.rs:124` returning `Vec<(Pattern, Range<usize>)>`, plus an R-vs-T comparison variant `extract_pattern_indices_temporal` (:181). The `Pattern` struct (patterns.rs:45-49) still carries only `p_type`/`time`/`snap`, matching the issue's premise.

## Acceptance criteria (pointer — GitHub is authoritative)

- Prefactor: expose note index ranges per pattern (or add `extract_pattern_indices()`)
- Backend: new `intra_pattern.rs` module (or extend reading) — filter Burst(2/3/4), per-pattern spacing stats (mean, stddev, min, max)
- Wire into `reading::analyze()` output — flat JSON fields per pattern type
- Frontend: `IntraPatternSpacing` fields in `ReadingResult` (`types.ts`); new `ReadingProfile.tsx` section showing spacing per pattern type with snap labels
- Tests: unit tests on known synthetic patterns; edge cases (zero-distance notes, single patterns)

## Relations

- [[issue-5-angle-distribution]] (gh#5) — the follow-up issue this body points at; #5 is blocked by this issue and reuses its segmentation infrastructure ("Slice 2 (Intra-Pattern Spacing)" per gh#5 body)
- [[issue-4-forward-density]] (gh#4) — sibling issue on the same pattern-detection infrastructure

## Links

[[reading-hub]] · [[reading-analysis]] · [[forward-density]] · [[spacing-demand]]
