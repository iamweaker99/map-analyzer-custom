---
type: research
status: designed
updated: 2026-08-08
sources:
  - Temp/framework-reading-analysis.md
  - Temp/handoff-reading-analysis_1.md
  - Temp/reading_analysis_architecture_draft_6.md
  - Temp/chaos-score-normalization-discussion_2.md
  - wiki/raw/prd-reading-analysis.md
  - wiki/wiki/module/reading-analysis.md
  - backend/src/analysis/reading/sequence_motor.rs (code)
  - backend/src/analysis/reading/mod.rs (code)
---
# Research: Reading Analysis — Design Story

The compiled design history of reading analysis: from a flat-list metric module to a pattern-aware design body, and which parts of that design the shipped pipeline actually implements. Entry point: [[reading-analysis]] (module, code state).

## Source chronology

| # | Source | Date | Content |
|---|---|---|---|
| 1 | handoff-reading-analysis_1 (Temp) | 2026-07-16 | Angle prototype built (`proto_angle.rs`), 15° bucket decision, framework doc written |
| 2 | framework-reading-analysis (Temp) | draft, ~07-16 | Three-layer pattern-aware framework (intra / inter / BPM) |
| 3 | prd-reading-analysis (raw) | 2026-07-16 | Scoped PRD: intra-pattern 2/3/4 + forward density |
| 4 | reading_analysis_architecture_draft_6 (Temp) | undated | Five-layer architecture (node → pattern → sequence → map → evaluators) |
| 5 | chaos-score-normalization-discussion_2 (Temp) | undated | Chaos-score normalization for intra-pattern spacing; Option B chosen |

Two sources are undated; the order above is inferred from content (chaos-disc references Burst(2/3/4), the PRD's scope; draft-6 references LTD/TV2, concepts from [[spacing-demand]]).

## Stream 1 — Angle distribution prototype (handoff-1, PRD)

- Prototype `compute_angles()`: interior angle per middle note of consecutive triples, `acos(dot(AB,BC)/(|AB|·|BC|))`, degrees 0-180; `compute_distribution()` buckets into configurable ranges with mean/median/stddev/min/max.
- **15° buckets = 12 bins (0-15 … 165-180)** chosen over 10° — 10° added 6 buckets without revealing new structure (handoff-1, PRD). Companion stats mean/median/stddev kept because they capture shape buckets miss: AngelMaker mean 56° / median 24° = mostly small angles with rare extreme spikes (handoff-1).
- **Raw angles ≠ trajectory entropy — both needed**: angles measure turn sharpness (a square is ~90° regardless of order); entropy measures predictability of the turning pattern. Orthogonal and complementary (handoff-1, PRD).
- Timeline visualization deferred: 5 s windows judged too coarse; user suggested ~1 s (handoff-1). Also deferred in PRD ("Timeline window granularity", out of scope).

## Stream 2 — Three-layer pattern-aware framework (framework)

Skeleton it builds on: `finger_control/patterns.rs` (Jump / Slider / Burst 2-6n / Stream 7+n via temporal proximity gap ≤ half-beat + 10 ms) and `finger_control/transitions.rs` (existing between-pattern pairs analysis) (framework).

- **Layer 1 — Intra-pattern**: angle + spacing inside each detected pattern; per-pattern-type aggregation `angles_by_pattern[type] = dist_15deg`, `spacing_by_pattern[type] = {mean, std_dev, min, max}`.
- **Layer 2 — Inter-pattern/transition**: boundary between adjacent patterns (last note of A → first note of B): boundary angle via cross-boundary triple, spacing change vs each pattern's avg, pattern delta + snap change (already in transitions.rs). Claimed mechanism for catching **anti-symmetry without case-by-case matching**: "high angle + high spacing change at a transition = reading difficulty spike" (framework).
- **Layer 3 — BPM-modulated spacing**: same px spacing feels harder at low BPM. Formula undecided — proposed `adjusted_spacing = raw_spacing · (bpm / reference_bpm)` or `spacing_anomaly = spacing_px / expected_spacing_for_bpm`; "least developed — needs discussion" (framework).

## Stream 3 — Five-layer architecture (draft-6)

Philosophy: **describe before evaluating**; separate measurements from interpretations; every layer answers exactly one question; complementary mechanisms instead of a universal metric. Layers: Node descriptors (local geometry: spacing, movement angle, velocity, timing, direction change) → Pattern descriptors (LTD, TV2, spacing stats/variance, angle stats, rhythm consistency) → Sequence descriptors (cluster size, recovery time, sliding-window demand, sustained exposure) → Map descriptors (peak, average, distribution, percentiles) → **Evaluators** (Top-N hardest, practice recommendation, skill profile) which consume descriptors without modifying them. No layer recomputes another layer's geometry. Draft-6 states current work focuses on the **Pattern Descriptor** layer.

## Stream 4 — PRD: scope discipline (prd-reading-analysis, 2026-07-16)

PRD narrows the framework for the first shipped slice:
- **Pattern scope: Burst(2), Burst(3), Burst(4) only.** 5+ note patterns, inter-pattern/transition analysis, BPM modulation, frontend UI, derived-metric cleanup, mods — all explicitly **Out of Scope** (PRD). Doubles report spacing only; triples report the single interior angle; 4-note patterns report both interior angles.
- **Supersedes framework Layer 3**: BPM modulation "not an objective metric — subjective perception of mismatch" (PRD) vs framework's "needs design" layer. Contradiction resolved by PRD (later, authoritative).
- **Supersedes framework Layer 2**: boundary angles NOT computed — transition notes treated as outliers; two boundary-context approaches (previous pattern's last note vs next pattern's first note) documented for a future PRD, neither decided (PRD).
- **Forward-looking density (1000 ms)**: count notes in `[start_time, start_time+1000]`; raw count only, no decay/weighting; sibling of existing visual density, not replacement (PRD). See [[forward-density]] (cross-wiki: an earlier handoff used 3000 ms to match ppy/osu#33196; PRD's 1000 ms supersedes — the design-agreed-not-implemented state is recorded there).
- Snap (1/2, 1/4, …) carries through from pattern detection; known limitation: half-beat + 10 ms gap can misclassify on variable-BPM maps, snap may read "Unstable" (PRD).
- Output: flat fields in reading JSON, no nested per-pattern-type sections; per-object angle values alongside aggregates for timeline plotting (PRD).

## Stream 5 — Chaos score normalization (chaos-disc)

Design for intra-pattern spacing difficulty of Burst(2/3/4): a single "chaos" score capturing the nonlinear interaction of frequency × magnitude of spacing changes, **no tunable weights**. Second-derivative approach: `chaos = sum(|ΔΔ of spacing|) / denominator` — 0 for monotonic trends, magnified on reversals (chaos-disc).

Normalization options tested on synthetic patterns (spacings in circle-diameter units D, `D = 108.8 - 8.96·cs` px):

| Option | Formula | Verdict |
|---|---|---|
| A: ÷ mean spacing | `sum\|ΔΔ\| / mean` | **Rejected** — flip problem: near-stacked jitter P4 [0.1,0.8,0.1] scores 4.2, wide wobble P5 [3,5,3] only 1.09; tiny denominators inflate |
| B: ÷ circle diameter D | `sum\|ΔΔ\| / D` | **Chosen** — P6 [1,6,1] dominates at 10.0, P5 at 4.0, P4 de-emphasized at 1.4, monotonic P3 = 0 regardless of width; CS-independent |
| C: ÷ (mean + D) | hybrid | Clamps extremes but collapses differentiation (P5 1.0 vs P7 1.6) |

Edge cases: Burst(2) has one spacing → **chaos = null**, only raw features (mean_spacing, stddev 0, num_direction_changes 0). Stacked notes (< 0.5·D) still included in computation (stacked→wide→stacked is itself reading difficulty); a 50% overlap threshold only for interpretation labels. Secondary diagnostic (weight-free): `relative_jitter = stddev(Δspacing)/mean_spacing`. Always emit a raw feature vector: `mean_spacing, spacing_stddev, num_direction_changes, max_spacing_delta, cumulative_jitter` (all in D units) for downstream classification (chaos-disc). Future direction: apply the same frequency+magnitude treatment to **angle distribution** — spacing freq/mag + angle freq/mag as the complete picture (chaos-disc).

## Mapping to the shipped pipeline (code-verified 2026-08-08)

Shipped flat-list pipeline: `visuals → density → trajectory → traps → strain` (module page). **Pattern-aware piece shipped**: `sequence_motor` — per-pattern MPA / MM / SC over pattern ranges from `finger_control::rhythm_segmentation::extract_pattern_indices` (R-variant, mod.rs:35-44), header-marked PROTOTYPE but present in `analyze()` output (mod.rs:6,44,201; module page).

- **MPA (Motor Plan Adjustment)** = mean absolute second difference of spacings, spacings pre-normalized by circle diameter (sequence_motor.rs:83-94) — this is the chaos-disc second-derivative idea in **mean** form with Option B's D normalization. The chaos-disc raw feature vector (num_direction_changes, max_spacing_delta, cumulative_jitter) is **not shipped**.
- **MM** = RMS spacing in diameters; **SC** = coefficient of variation of spacing (sequence_motor.rs:96-112) — covers the PRD's "mean spacing + consistency" per-pattern intent, but as a per-pattern timeline, not the PRD's per-pattern-type aggregated distributions.
- Minimum-note gates: MPA 0.0 if < 4 notes, MM 0.0 if < 2, SC 0.0 if < 3 (sequence_motor.rs, module page).

**Design claims the code does NOT implement**: 15° angle distribution (proto_angle.rs prototype never wired into `analyze()`; no angle section in mod.rs output — [[issue-5-angle-distribution]] open); forward density ([[forward-density]], design-agreed-not-implemented; issue #4 archived); intra-pattern spacing per pattern type with snap ([[issue-3-intra-pattern-spacing]] open); Layer 2 inter-pattern/transition analysis incl. anti-symmetry (deferred by PRD); Layer 3 BPM modulation (out of scope).

## Open questions (framework/handoff-1)

1. Jumps (2 notes): report direction vector (atan2) or spacing only? Unresolved.
2. Stream length granularity: 7-note vs 20-note streams may differ — track length as a dimension?
3. Combined score vs separate fields: angle + entropy + spacing merged or kept separate?
4. Pattern detection accuracy: misclassified patterns (stream split into two bursts) propagate into pattern-level metrics — improve detector first?

## Related

[[reading-analysis]] · [[sequence-motor]] · [[angle-distribution]] · [[forward-density]] · [[spacing-demand]] · [[finger-control]] · [[issue-3-intra-pattern-spacing]] · [[issue-5-angle-distribution]] · [[issue-4-forward-density]] · [[Data-Philosophy]] · [[reading-hub]]
