# Handoff: Spacing Transition Demand Implementation

## State

**Status:** Design phase complete, prototype tested, ready for implementation.
**Branch:** `Reading_Analysis_Iteration`
**Date:** 2026-07-17

---

## What's been decided

### Design spec

The governing specification is `Temp/spacing_transition_demand_research_notes.md`. Core philosophy:

- **Describe, don't classify** — continuous mathematical descriptors, no pattern geometry labels
- **No arbitrary weights** — every operation must emerge from the mathematical object
- **Local** — metrics depend on local transitions between consecutive spacings
- **Continuous** — small geometric changes produce small score changes
- **Extendable** — same philosophy must apply to angle sequences later

### Candidate families

Both families are implemented and compared in the prototype (see below). Both satisfy all five design principles.

| Family | Formula | Interpretation |
|--------|---------|---------------|
| **A (TV2)** | Σ\|Δ²s\| | Total absolute second-order variation (L¹) |
| **B (LTD)** | (1/k)·Σ(Δ²s)² | Mean squared bending energy (L²), normalised by length |

The prototype comparison at `Temp/spacing-transition-demand-candidate-analysis.md` **recommends Family B as primary**, with the following rationale:

1. **Normalisation** — LTD is a mean (normalised by pattern length), TV2 is a sum (scales with length). LTD separates "how demanding each transition" from "how many transitions."
2. **Emphasis** — L² penalises large deviations quadratically, matching the reality that an extreme spacing reversal is disproportionately harder, not just linearly harder.
3. **Comparability** — LTD scores are directly comparable across different pattern lengths.

### Normalisation

- Spacings are divided by **circle diameter (D)** before computing Δ²
- Stacked threshold: **< 0.5 × D** (consistent with existing stream analysis in `streams.rs`)

### Scope

- Target patterns: **Burst(2), Burst(3), Burst(4)** only
- Burst(2): only 1 spacing → no Δ² possible → **both scores are null** (raw feature vector only)
- Burst(3): 2 spacings → no Δ² possible → **both scores are null** (raw feature vector only)
- In practice, top-N rankings will consist entirely of 4n Burst patterns since those are the only ones scorable

### Pattern snap inclusion

- **1/2 snap patterns ARE included** (finding from initial testing: excluding them left almost no patterns on a 220 BPM deathcore map)

---

## What's been built

### Code changes

| File | Change |
|---|---|
| `backend/src/analysis/finger_control/patterns.rs` | **Added** `extract_pattern_indices()` — returns `Vec<(Pattern, Range<usize>)>` with note index ranges per pattern (the prefactor step from issue #3). Existing `extract_patterns()` unchanged. |
| `backend/src/bin/prototype_spacing_demand.rs` | **New** — test binary loading two .osu beatmaps, computing TV2 and LTD on every Burst(2/3/4) pattern, printing top 10 per method per map. Compiles and runs. |

### Test artefacts

| File | Purpose |
|---|---|
| `Temp/spacing_transition_demand_prototype.py` | Python prototype with mathematical derivation, test patterns, design principle verification, L¹ vs L² analysis |
| `Temp/spacing-transition-demand-candidate-analysis.md` | Full analysis document with formulas, empirical comparison, recommendation |
| `Temp/chaos-score-normalization-discussion.md` | Earlier discussion on normalisation options (historical reference) |

### Test maps used

1. `D:\osu files\AngelMaker - A Dark Omen (Kyu96) [Demonic Colossus].osu` — CS 3.8, BPM 220
2. `D:\osu files\YOASOBI - Yoru ni Kakeru (CoLouRed GlaZeE) [Collab Extra].osu` — CS 4.0, BPM 130

### Test findings

- AngelMaker (220 BPM deathcore): 22 Burst(2/3/4) patterns, almost all at 1/2 snap. Top TV2 ~1.27, top LTD ~1.62. Very consistent spacing overall.
- Yoru ni Kakeru (130 BPM J-pop): 14 patterns, wider score range. Top TV2 ~5.59, top LTD ~31.25. More spacing variation — 1/3 triplet bursts at [00:36], [00:48], [00:58], [03:53] are the most demanding.

---

## What needs to be done

### Backend implementation

1. **Integrate into `reading::analyze()`** — The prototype lives in a standalone binary. The real implementation needs to be wired into `reading::analyze()` so its output appears in the reading analysis JSON.

2. **Data shape to emit** — Per-pattern results must be aggregated into the reading analysis output. Suggested shape (values as arrays of per-pattern records):

```json
{
  "spacing_demand": {
    "patterns": [
      {
        "time_ms": 155000.0,
        "snap": "1/4",
        "n_notes": 4,
        "spacings_d": [1.84, 3.11, 2.80],
        "tv2": 1.581,
        "ltd_energy": 2.499,
        "ltd_rms": 1.581
      }
    ],
    "top_tv2": [...],   // top N by TV2, pre-sorted
    "top_ltd": [...],    // top N by LTD, pre-sorted
    "summary": {
      "mean_ltd": 0.0,
      "max_ltd": 0.0,
      "pattern_count": 0
    }
  }
}
```

3. **Which module** — Create a new file `backend/src/analysis/reading/intra_pattern.rs` (as originally called out in issue #3). It should:
   - Call `patterns::extract_pattern_indices()` on the beatmap
   - Filter to Burst(2/3/4)
   - Filter out Burst(2) and Burst(3) (no Δ² possible)
   - For each Burst(4): extract spacings, compute TV2 and LTD, record time and snap
   - Return structured data for `reading::analyze()` to embed in its JSON

4. **Copy the prototype logic** — the spacing extraction and metric computation in `prototype_spacing_demand.rs` is ready to be moved into the production module. The formulas are verified against synthetic and real data.

### Frontend integration

**⚠️ PENDING — NOT YET DISCUSSED**

The frontend web UI display needs to be designed. Open questions:

- Should the frontend show **per-pattern** results (a table/list of individual patterns) or **aggregate** statistics (mean LTD, distribution buckets)?
- If showing individual patterns, how many? Top 5/10/20?
- Should TV2 or LTD be the primary displayed metric?
- Where in the ReadingProfile UI should this appear? A new card? Inside an existing section?
- Should the spacing values (× D) be shown alongside the scores?
- How to handle the case where no patterns are scorable (all Burst 2/3, or no bursts at all)?
- Mobile responsiveness for a potentially long list of patterns?

**Recommended approach (not approved):** A dedicated card in ReadingProfile showing a table of top patterns by LTD, with columns for Time, Snap, Pattern length (4n), max spacing, and LTD score. Collapsible to show top 5 by default with an option to expand.

### Testing strategy

- **Unit tests** — Copy the synthetic patterns from `proto_angle.rs` tests as a model. Test known patterns with known scores:
  - Consistent [1,1,1] → TV2 = 0, LTD = 0
  - Arithmetic [1,2,3] → TV2 = 0, LTD = 0
  - Single reversal [1,3,1] → TV2 = 4, LTD = 16
  - Ensure Burst(2) and Burst(3) return None scores
- **Integration** — The existing prototype binary can serve as a manual integration test
- **Edge cases** — Zero spacings (stacked notes), patterns with sliders swallowed into bursts

---

## Files to touch (implementation)

| Priority | File | What |
|---|---|---|
| **P0** | `backend/src/analysis/reading/intra_pattern.rs` | **Create** — main spacing demand module |
| **P0** | `backend/src/analysis/reading/mod.rs` | **Edit** — call intra_pattern module, embed results in JSON |
| **P0** | `backend/src/lib.rs` | **Edit** — ensure new module is exported (if needed) |
| **P1** | `frontend/src/types.ts` | **Edit** — add `IntraPatternSpacing` / `SpacingDemandResult` types |
| **P1** | `frontend/src/components/ReadingProfile.tsx` | **Edit** — add UI section (DESIGN PENDING) |
| **P2** | `temp/*.py`, `temp/*.md` | Clean up or archive after implementation stabilises |

---

## Risk & notes

- **Burst(2) and Burst(3) produce no Δ² terms** — these patterns will never appear in top-N rankings. The frontend should make this clear (e.g., "only 4-note bursts are scorable").
- **Sliders swallowed into bursts** — `extract_pattern_indices()` inherits the slider-swallowing logic from `extract_patterns()`. Slider head positions are used as note positions, which is correct for spacing computation.
- **Patterns with mixed CS** — Not applicable for standard maps (single CS per map), but worth noting for future multi-CS modes.
- **Backward compatibility** — Adding new fields to `reading::analyze()` JSON output is additive; existing frontend code that doesn't know about the new fields should ignore them gracefully.
