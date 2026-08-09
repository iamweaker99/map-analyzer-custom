---
type: issue
github: "#4"
status: closed
updated: 2026-08-08
---
# Issue #4 — Forward-Looking Note Density (1000ms window)

**Archived 2026-08-08 — gh#4 CLOSED on GitHub (SCHEMA rule 8).** Design notes live on [[forward-density]]; the working branch `Reading_Analysis_Iteration` still carries the uncommitted work.

**GitHub:** #4 · CLOSED · enhancement ("None — can start immediately")

## State
Design agreed, **not implemented**. Working branch: `Reading_Analysis_Iteration`.

## Chosen approach (build-first, then validate, then test)
1. Implement `compute_forward_density()` in `backend/src/analysis/reading/forward_density.rs`
2. Wire into `cargo run` output so a real `.osu` file prints forward density + existing density side by side
3. User picks a map from `D:\osu files` and runs it themselves
4. Compare against prototype expectations by eye
5. Match → add one lightweight regression test as a lock; mismatch → debug with real data

## Why not strict TDD here
- API already fixed by issue spec + prototyping — TDD's API-forcing benefit not needed
- Real-data validation is more trustworthy than synthetic 4-note tests for this metric
- Prototype already proved the approach; the question is implementation fidelity

## Related issues
- **Issue #3** (Intra-Pattern Spacing) — sibling, same Slice 2 infrastructure
- **Issue #5** (Intra-Pattern Angle Distribution) — blocked by #3
- Issue #3/#5 context lives in the PRD: [[forward-density]] carries the shared design notes

## Output placement
Sibling of `density` / `trajectory` / `traps` / `topography` in the reading JSON.

## Links
- Design: [[forward-density]] · Module: [[reading-analysis]] · PRD: `raw/prd-reading-analysis.md`
