# Pivot Rule (pattern-segmentation boundary placement)

**Summary:** The pivot rule moved pattern boundaries one note earlier at subdivision speed-up transitions (boundary before the pivot note instead of after), matching the Feral mapper's new-combo convention — but it broke the YOASOBI control irreparably and was abandoned on 2026-08-11 after the user gave up on optimizing it.

## Concept

The R-discontinuity rule fires at snap-ratio changes (e.g. 1/4→1/3, 1/3→1/6) but historically places the boundary AFTER the pivot note (k+1). The user's Excel walk of Feral showed the pivot note belongs to the FASTER side: the mapper starts the new run ON the pivot. Class A errors (7 walked sites on Feral) were exactly this boundary-placement convention. The pivot rule was the fix: speed-up → boundary BEFORE the pivot (k); slow-down → boundary AFTER the pivot (k+1, unchanged).

## Algorithm idea

At an R-firing subdivision transition, choose the boundary side by the direction of the snap change: faster snap follows → boundary at k; slower snap follows → boundary at k+1. Applied identically in the R and T loops. Tunable knobs: the pivot-direction decision itself, and candidate companion gates (see Tuning history).

## Data shape

Input: the note stream with per-note snap fractions and new-combo bits. Output: pattern boundaries shifted by one note position relative to the plain R-discontinuity landing, only at subdivision transitions where R fires.

## Frontend UX

Prototype-only (exp artifact). Would have surfaced as stream-section boundaries appearing one note earlier in the reading/finger-control cards. No frontend work was done.

## Tuning history

- **v1 (adopt as-is):** boundary at k on speed-ups. Feral walk fixed 7/7 Class A sites, zero fragmentation — but YOASOBI control broke: 106 rows flipped (39 merges including 6 pickup-joins spanning 2–5 combo sections, 12 boundary shifts). The old damage-based kill-criteria fired.
- **NC-gate hypothesis:** only pivot when the pivot note carries new-combo. Dead: all 6 YOASOBI join pivots carry NC=1 — the same signal as Feral's pivots, with the opposite walk outcome. NC validates mapper intent but cannot discriminate which convention the mapper used.
- **Combo-boundary gate:** reject moves that create patterns with mid-combo breaks. Kills 6/6 YOASOBI joins but is partial — ≈4 dissolves survive.
- **Attribution:** the pivot rule caused ≈100% of the YOASOBI damage; the R_THRESHOLD 0.5→0.35 drop (kept, user-verified) contributed ~0.

## Why abandoned

No clean mechanical separator exists between the two mapper conventions. The rule encodes Feral's mapper intent; YOASOBI's intent is opposite at identical signals (same NC bits, same rhythm ratios, same distances). Every candidate gate was partial or contradicted. The user decided 2026-08-11: give up optimizing; the R_THRESHOLD 0.35-only fix (Feral Class B, zero YOASOBI damage) is the accepted production change; Feral Class A boundary placement is a documented known limitation.

## Validation maps

- Feral [Veracious] — target; full walk, 7/7 Class A sites accepted with the rule.
- YOASOBI Collab Extra — control; fully user-verified 549-row walk; 106 rows flipped → canary kill.
- Signal [Disturbance] — 02:08.539 stack byte-identical; 21 pivot moves all speed-up; 5 fragments; 413→397.
- Heart Pie Dancehall [3.1415926535] — 638ms stack byte-identical; 19 singletons vs 76 merges; 608→566.

## Test ideas

- Maps whose mapper convention is "pivot belongs to the slower side" (YOASOBI-style) — the exact failure case.
- Boundary dissolves on partial-follow sliders (the ≈4 survivors of the combo-boundary gate).
- Pickup-join pattern: streams spanning 2–5 combo sections created by k+1→k moves.
- A convention detector (per-map or per-mapper signal analysis) instead of a universal rule — the data says the rule is mapper-specific, not signal-universal.
