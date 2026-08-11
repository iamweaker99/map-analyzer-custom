---
type: decision
status: accepted
updated: 2026-08-10
---
# Decision: Abandon the s→c suppression refinement

**Date:** 2026-08-10 · **Status:** accepted

## Context
`skip_discontinuity` suppresses the R/T discontinuity signal at slider→circle transition windows (asymmetric — circle→slider windows keep it). The 8-mismatch analysis found one over-fire: case 6 — R killed at k=152 across a 346 ms gap (3/4 beat, a real gap, not a pickup).

## Candidate refinement
Un-suppress R at genuine gaps (> ½ beat). Result: fixes case 6 (boundary at 153 restored) but breaks case 8 — R at k=165 (692 ms gap) adds a boundary at 166, splitting the wanted 3n burst.

## Decision
**No gap threshold satisfies both cases — what it fixes = what it breaks. Abandon the refinement.** Keep the asymmetric s→c suppression as-is.

## Consequences
- Case 6 stays a known limitation (accepted with the run-start set — [[run-start-engulf-known-limits]])
- No code change (discussion-only; the prototype already carries the base behavior)

## Related
- [[rhythm-segmentation]] · [[run-start-engulf-known-limits]]

_Sources: prototype_sequence_motor.rs (`skip_discontinuity`), editor cross-check (user), 2026-08-10 session analysis_
