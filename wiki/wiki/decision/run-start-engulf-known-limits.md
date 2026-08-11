---
type: decision
status: accepted
updated: 2026-08-10
---
# Decision: Accept the run-start engulf mismatches as known limitations

**Date:** 2026-08-10 · **Status:** accepted

## Context
The `--exp` prototype's circle→slider type rule splits a slider as **run-start** when the following slider run ≥ 2, or its head is beyond engulf range (145.92 px = 2× circle diameter at CS 4). Editor cross-check (YOASOBI Collab Extra) found 8 circle→slider cases (00:13–00:42, all 1/4 rhythm) that read as "burst ending in engulfed slider" but were split as run-start.

Re-measured from raw .osu coordinates (round-2 correction — the earlier per-case figures were transcription errors: the diagnostic prints each row's distance to the NEXT object):
- 7 of 8 heads are exact 0px stacks on the last circle; case 4 = 116.5 px (only non-stack)
- Breakdown: 6 = run-start type rule; case 5 = R boundary lands at k+1 (unreachable — no signal fires at k); case 6 = s→c suppression over-fire (346 ms gap)
- All 13 transitions are 115 ms (1/4 beat); the 5 "verified-out" run-start sliders sit at 38.9–97.8 px — inside engulf range too

## Candidate fix evaluated (simulator validated byte-identical to the exp artifact)
"Proximity overrides run-start" (boundary iff head distance > 145.92 px):
- Fixes 7 of the 8 (case 5 unchanged) + matches the user's 02:37 → 19 read (739 was mislabeled "verified-out")
- Breaks 6 user-verified reads: 02:03 → 37 (547), 01:26 → 17 (376), 03:55 → 13 (1066), 02:53 → 13 (804) — plus 2 newly found at 03:38 (964 solo slider absorbed; 982 swallows the verified 8-circle run 974..982 → 9)
- Map-wide: 549 → 471 patterns (−78); 237 of 435 sliders lie within the threshold

## Why no signal can separate the sets
- Distance: in-set 0–116.5 px fully contains out-set 38.9–97.8 px — no threshold exists
- NC: perfect static split on the original 13 (8-in all type 2, 5-out all type 6 incl. 739) but contradicted by the user's own reads both ways: 739 (NC) counted IN; 964 (type 2, no NC) verified OUT
- Rhythm: all 13 transitions identical (115 ms); "followed by a slider" is true for both sets

## Decision
**Reject the proximity override. Keep the run-start type rule as-is. Accept the 8 engulf mismatches as known limitations** — same class as the 02:08:539 acceptance. No rule change in prototype or production.

## Consequences
- Prototype keeps a documented divergence from the user's editor reading in 8 spots (see [[rhythm-segmentation]])
- 02:37 stays 18 in exp-mode; production R-mode already yields 19 (matches the user) — resolved by the production port, not by exp rules
- The "trailing heads 38.9–116 px in vs ≥185 px out" dataset is **retracted** (transcription artifact; the flawed flag included 6 true 0px stacks; the genuine ≥185 px-head set = 30 sliders, 185.3–421.9 px). The 2× diameter engulf threshold itself stands — no evidence against it.

## Related
- [[rhythm-segmentation]] · [[suppression-refinement-abandoned]] · [[2026-08-10-handoff]]

_Sources: YOASOBI .osu raw coordinates, prototype_sequence_motor.rs + validated simulator, `Prototyping/` artifacts, editor cross-check (user), sub-agent re-measurement + override evaluation_
