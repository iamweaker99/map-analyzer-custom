---
type: decision
status: accepted
updated: 2026-08-10
---
# Decision: Unify finger control analysis on the updated pattern segmentation

**Date:** 2026-08-10 · **Status:** accepted

## Context
Two parallel pattern-classification paths produce overlapping output from the same beatmaps with untested divergence (index gap "2 parallel pattern-classification paths"; [[rhythm-segmentation]] §Two parallel paths):
- **Path A — `finger_control::patterns::extract_patterns`** (old): global `map.bpm()`, fixed gap, 25px slider-swallowing; drives the finger-control card's transitions/timeline.
- **Path B — `rhythm_segmentation::extract_pattern_indices`** (new): per-timing-point BPM, ±10% tolerance, gap + R boundaries; drives the reading pipeline. The `--exp` type-boundary rules (2026-08-09/10) fix Path B's known classification bugs.

## Decision
**Unify: finger control analysis uses the current updated pattern segmentation** (rhythm_segmentation + the exp type rules: asymmetric type-change boundary, s→c R-suppression, `PatternType::SliderChain`, 2× diameter engulf). Path A (`finger_control::patterns`) retires. Implementation lands with the production port — still gated on the user's other-map prototype testing ([[2026-08-10-handoff]]).

## Rationale
Two engines for the same concept is a documented contradiction (Path A already engulfs, Path B was type-blind — resolved by the asymmetric rule); keeping both means the finger-control card and reading diverge on the same map.

## Consequences
- finger-control transitions/timeline will use the new boundaries once the port lands; card output may shift (frontend impact unassessed — kept open)
- `patterns.rs` retires at port time (nothing deleted before that)
- 02:37→19 discrepancy resolves via the same port (production R-mode already yields 19)

## Related
- [[rhythm-segmentation]] · [[finger-control]] · [[run-start-engulf-known-limits]] · [[2026-08-10-handoff]]

_Sources: rhythm_segmentation.rs, patterns.rs, [[rhythm-segmentation]] §Two parallel paths, handoff_11:76, user decision 2026-08-10_
