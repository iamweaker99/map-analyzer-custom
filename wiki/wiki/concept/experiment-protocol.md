---
type: concept
updated: 2026-08-11
---

# Experiment Protocol

*Every experiment records its prediction and kill-criteria BEFORE it runs; outcome and verdict after. Adopted 2026-08-11 (session discussion, research-loop wave 1).*

## Why

Trial-and-error rationalizes results after the fact; a pre-written falsification criterion can't be talked around. Without one, every experiment "looks promising" and the loop never converges.

## Entry format

One line per experiment in `log.md` (and prototype/handoff records):

```
prediction | kill-criteria | outcome | verdict
```

- **prediction** — what we expect to observe, concrete (numbers over adjectives)
- **kill-criteria** — the falsifying observation; **mandatory, written before the run**; must be **clue-based** (see below), not damage-based
- **outcome** — what was actually observed, with evidence (file:line, counts, artifacts)
- **verdict** — `confirmed` / `refuted` / `inconclusive` + next step (keep / tune / abandon)

## Kill-criteria: clue-based, not damage-based (adopted 2026-08-11)

A criterion that only fires on measured damage ("any accepted case flips", "net breaks > fixes") is a post-mortem — the damage is spent before it triggers, like stopping the project after the million is gone. A useful criterion detects the **clues** that failure is likely, cheaply, BEFORE the expensive part runs. Three checkpoints, each gating the next stage:

**1. Pre-flight clues (static — raw data + the rule only, no pipeline run; seconds)**
- *Premise coverage*: the evidence the rule relies on must hold on the target map at every site the rule will touch (e.g. NC-on-pivot at subdivision changes). Coverage < 90% → the rule over-generalizes → stop or narrow.
- *Would-be damage on walked controls*: simulate the rule's boundary moves; any move that touches an accepted row of a fully user-verified map, or creates a pattern spanning a combo section → red → stop (or narrow the rule first).
- *Exposure*: a fully-walked map is the canary — if the rule would touch > 5% of its rows, run that map FIRST and alone.

**2. Canary clues (after the FIRST map run, before fanning out the rest)**
- Any accepted row flipped on the canary map → stop; the remaining maps do not run under this rule.
- Direction imbalance: changes all one direction (merges-only or splits-only) on a walked map → suspicious; record + gate.
- Pattern-count swing |Δ| > 5% on any map → investigate before continuing.

**3. Final clues (post-run — confirmation, not first detection)**
- The classic counts stay as the last checkpoint: net-new breaks vs fixes, fragmentation, accepted-case flips.
- If red here but 1+2 were green, the clues failed to fire — that gap is itself a finding (feeds the next prediction line).

Retrospective: with these checkpoints, the 08-11 pivot-rule rerun would have stopped after the YOASOBI canary (106 accepted rows touched) and never run the other 3 maps.

## Exploration runs

Cheap, wide runs deliberately testing nothing are tagged `[explore]` — prediction + outcome only, no kill-criteria. `[explore]` runs never produce verdicts; they generate hypotheses.

## Discipline

- Kill-criteria lines are written before the run starts, never after.
- A `refuted` idea is dead; re-attacking it requires a new prediction line (see [[run-start-engulf-known-limits]]).
