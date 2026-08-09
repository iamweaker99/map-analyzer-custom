---
type: research
status: prototyped
updated: 2026-08-08
sources: [raw/reading-handoff-01-forward-density.md, raw/prd-reading-analysis.md, Temp/prototype-results-forward-density.md, memory: forward-density-plan]
---
# Research: Forward-Looking Note Density

A metric for "what's coming up" cognitive load: for each hit object, count notes whose start time falls within the next window.

## Design evolution (why a wiki beats re-reading the sources)
| Phase | Window | Notes |
|---|---|---|
| Initial handoff | 3000ms | Match official PR (ppy/osu#33196) for comparability |
| Prototype finding | 1000ms | Sweet spot — captures anticipation without noise (15° bucket test family) |
| Issue #4 (agreed) | 1000ms, **inclusive** (`<= 1000`) | Seam decision locked — inclusive, not exclusive |

The handoff and PRD contradict each other on window size. The prototype results file (Temp/prototype-results-forward-density.md, dated 2026-07-16, predecessor = handoff) is the empirical bridge: it tested 1000ms, 1500ms, and 3000ms side-by-side and chose 1000ms; the PRD then supersedes the handoff by locking that decision. Recorded in [[log]].

## Empirical results (prototype run, 2026-07-16)
Throwaway Rust CLI (`backend/src/bin/prototype_forward_density.rs`) computed forward density alongside the existing visual density, printing both distributions side-by-side at 1000ms / 1500ms / 3000ms (per prototype-results-forward-density). Buckets used: Isolated (0-2), Chunking (3-5), Clutter (6-8), Overload (9+).

### YOASOBI - Yoru ni Kakeru [Collab Extra] (130 BPM, AR 9.2, 1097 objects, moderate spacing)
| Category | Visual | 1000ms | 1500ms | 3000ms |
|---|---|---|---|---|
| Isolated (0-2) | 20.6% | 7.6% | 1.5% | 0.8% |
| Chunking (3-5) | 79.4% | 64.4% | 16.2% | 1.2% |
| Clutter (6-8) | 0.0% | 28.0% | 55.4% | 3.1% |
| Overload (9+) | 0.0% | 0.0% | 26.9% | 94.9% |
| Mean | 3.3 | 4.6 | 7.4 | 14.7 |
| Correl. r vs visual | — | 0.770 | 0.643 | 0.530 |

### AngelMaker - A Dark Omen [Demonic Colossus] (220 BPM, AR 9.8, 1824 objects, dense streams)
| Category | Visual | 1000ms | 1500ms | 3000ms |
|---|---|---|---|---|
| Isolated (0-2) | 8.4% | 2.1% | 0.5% | 0.2% |
| Chunking (3-5) | 34.9% | 20.4% | 3.9% | 0.3% |
| Clutter (6-8) | 56.7% | 21.1% | 14.2% | 1.0% |
| Overload (9+) | 0.0% | 56.4% | 81.3% | 98.5% |
| Mean | 5.8 | 9.8 | 15.3 | 30.4 |
| Correl. r vs visual | — | 0.946 | 0.934 | 0.884 |

### Why 1000ms won (reasoning per prototype-results-forward-density)
- **3000ms** collapses everything into overload on both maps (94.9% / 98.5%) — undifferentiating.
- **1500ms** is OK on moderate maps but already 81.3% overload on dense maps — loses spread.
- **1000ms** keeps spread on both: moderate maps show a meaningful shift (visual says 0% clutter, forward says 28% — captures "more coming than expected" cognitive load); dense maps still span chunking/clutter/overload (20.4% / 21.1% / 56.4%). Correlation r = 0.770 / 0.946 confirms "related but different" — exactly what a new parallel metric should be. Also a round, easy-to-document number.

The prototype results do **not** contradict the agreed design — they are its empirical basis: the 1000ms window, the raw-count rule, and the keep-separate decision all trace to this run. One soft mismatch: the PRD's example bucket boundaries (0-3, 4-6, 7-10, 11+) differ from the prototype's actual buckets (0-2, 3-5, 6-8, 9+), which are the ones the integration checklist carries (`isolated_pct`, `chunking_pct`, `clutter_pct`, `overload_pct`).

## Agreed API
```rust
compute_forward_density(nodes: &[VisualNode], window_ms: f64) -> Vec<ForwardDensityPoint>
// ForwardDensityPoint { time: f64, forward_count: usize }
```

## Fixed rules
- Raw count only — no decay, no opacity weighting, no time nerf
- NM only (no mods)
- **Sibling** of existing density section in reading JSON, not a replacement
- Reported per-object as a time series AND as an aggregate distribution (bucket percentages)

## Status
- Prototype completed 2026-07-16 (throwaway CLI); design agreed, **not implemented** in production → [[issue-4-forward-density]]
- Integration checklist from the prototype (per prototype-results-forward-density): new `forward_density` JSON section in `backend/src/analysis/reading/mod.rs`; `ForwardDensityResult` in `frontend/src/components/analysis_engine/types.ts` + chart in `ReadingProfile.tsx` mirroring the "Visual Clutter" section; Discord bot `reading.rs` embed + `types.rs`; delete `prototype_forward_density.rs` once integrated.
- Validation plan: build-first, run on real `.osu` files from `D:\osu files`, regression-lock only after visual match

## Open questions
- Merge with visual density or keep separate? — **Answered by the prototype: keep separate.** The moderate correlation (r = 0.77 on the moderate map) means the two capture different aspects; forward density is a new parallel metric, not a replacement (per prototype-results-forward-density). This supersedes the earlier "deferred until both curves are visible" stance.

## Related
- [[reading-analysis]] · [[reading-hub]] · [[issue-4-forward-density]] · [[Data-Philosophy]] · [[overview]]
