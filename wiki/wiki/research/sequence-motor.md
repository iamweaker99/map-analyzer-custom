---
type: research
status: prototyped
updated: 2026-08-08
sources: [Temp/000-handoff-sequence-motor-descriptors.md, Temp/handoff-sequence-motor.md, Temp/sequence_motor_plan_research_notes_7.md, Temp/prototype_sequence_motor_descriptors_8.md, Temp/temporal-vs-rhythm-illustrated.md, wiki/wiki/module/reading-analysis.md, backend/src/analysis/reading/sequence_motor.rs, backend/src/analysis/finger_control/rhythm_segmentation.rs]
---
# Research: Sequence Motor Descriptors (MPA / MM / SC)

Three orthogonal, sequence-length-independent motor descriptors computed per-pattern (bursts/streams with ≥2 notes) inside the reading pipeline: **MPA** (Motor Plan Adjustment — geometric instability), **MM** (Movement Magnitude — RMS spacing), **SC** (Spacing Consistency — coefficient of variation). Header-marked PROTOTYPE but shipped in `reading::analyze()` output (per reading-analysis.md:23). They deliberately produce **no** single difficulty score.

## Origin and design evolution

1. **Research notes (2026-07-18, sequence_motor_plan_research_notes_7.md)** — started from visualizing spacing demand over time. Two philosophy shifts:
   - *Describe, don't classify*: bursts/streams/deathstreams are human labels; the primary analysis object is a **note sequence**. Descriptors must not depend on the label.
   - *Sequence-length independent*: must work for 4, 8, 30 notes; score must not inflate with note count; comparable across lengths.
   - Reinterpreted "spacing transition demand" as **Motor Plan Adjustment** — "how much the player's movement plan must change while executing a sequence" — which maps to player intuition (constant `2→2→2` ≈ none; `1→3→1` ≈ large).
   - **Identified contradiction**: `1→3→1` and `2→6→2` have identical normalized shape (identical normalized LTD score) yet players find the second harder (larger cursor travel). One metric was representing two concepts.
   - **Proposed two-metric model**: Metric 1 = Motor Plan Adjustment (scale-independent, candidate: existing LTD), Metric 2 = Movement Scale (candidates: mean spacing, max spacing, total path length, RMS — "to be evaluated experimentally"). Rationale: display both independently so the player understands *why* a pattern feels harder (high adjustment + high scale vs high adjustment + low scale).
   - UI direction: numbers with the evidence (e.g. Transition Score 18.4 + Spacing Profile `1.1D → 3.4D → 1.2D`), not long text.

2. **Prototype spec (prototype_sequence_motor_descriptors_8.md)** — expanded to **three** metrics (SC added; the third went unnamed in the notes' two-metric model): MPA = mean(abs(Δ²)) of spacings; MM = sqrt(mean(s²)); SC = std(s)/mean(s). Explicitly **not** endurance metrics, not a difficulty score. Visualization recommended a **sliding window** (8 or 16 notes, step 1) producing three synchronized time-series + global stats (mean, max, p95), rendered as stacked line charts with shared time axis. "Avoid combining the three metrics into a single score during this prototype phase."

3. **Grilling handoff (000-handoff-sequence-motor-descriptors.md)** — locked the computation unit: **each pattern is one atomic window, no sliding sub-window** (supersedes the spec's sliding window); min ≥2 notes (jumps/sliders excluded); MPA = 0.0 for 2–3 note patterns (needs ≥4 for second differences); spacing normalized by circle diameter `108.8 − 8.96·CS`; time format `"MM:SS"`; output = per-pattern records + summary (mean, max, p95); pattern source `finger_control::patterns::extract_pattern_indices(map)`; wire into reading; binary `prototype_sequence_motor`; no synthetic test patterns — real `.osu` beatmaps only.

4. **Implementation handoff (handoff-sequence-motor.md)** — done: 6 unit tests pass, verified on 2 beatmaps in `D:\osu files\`, wired into `reading::analyze()` as a `"sequence_motor"` JSON section. Next step flagged: dataviz stacked-line visualization (timeline entries have `time, mpa, mm, sc`).

5. **Segmentation (temporal-vs-rhythm-illustrated.md + shipped rhythm_segmentation.rs)** — pattern boundaries use two signals: gap > ½ beat, and rhythm discontinuity **R** = |log₂(snap₂/snap₁)| > 0.5 (snap via per-timing-point beat_len, from the timing point at the **second** note's time, snapper rule per temporal-vs-rhythm-illustrated.md:7-8). The temporal variant **T** = |log₂(Δt₂/Δt₁)| is pure time-ratio with no BPM info. Illustrated findings:
   - T and R agree on single-BPM maps (same-snap sections, clean 1/4→1/2 boundaries).
   - Only R detects a BPM change sitting **between** two pairs: T reads flat 0.00 while R spikes +0.585 (Example 3); T−R quantifies the BPM ratio alone and acts as a BPM-change detector.
   - Recommendation (temporal-vs-rhythm-illustrated.md:139): **use R as the primary signal** — it does everything T does plus catches the BPM-mid-triple case; T is only a fallback when timing points are unavailable.

## What shipped vs what stayed in the prototype

| Item | Prototype spec / handoff | Shipped code (verified 2026-08-08) |
|---|---|---|
| Computation unit | Spec: sliding window 8/16 notes, step 1 | Per-pattern atomic windows (handoff decision) |
| Metrics | MPA, MM, SC | Same three (sequence_motor.rs:57-121) |
| Min notes / zero rules | ≥2; MPA 0.0 if <4 | ≥2 in timeline; MPA 0.0 if <4; MM 0.0 if <2; SC 0.0 if <3 |
| Normalization | diameter `108.8 − 8.96·CS` | Same (mod.rs:12-14; sequence_motor.rs:10-11) |
| Pattern source | `finger_control::patterns::extract_pattern_indices` | `finger_control::rhythm_segmentation::extract_pattern_indices` (R variant, mod.rs:35) |
| Output shape | timeline + mean/max/p95 | Same (`sequence_motor` section, mod.rs:201) |
| Time format | "MM:SS" | `analyze()` rounds to MM:SS (mod.rs:47-52); prototype bin prints MM:SS.mmm |
| Visualization | Stacked line charts (spec) | Not shipped — flagged as next step (dataviz) |
| Test data | "No synthetic patterns — real beatmaps only" | 6 unit tests on synthetic node geometries (sequence_motor.rs:211-308) |

## Contradictions (handoff vs code)

- **Stale pattern-source path**: handoff 000 and sequence_motor.rs:172 docstring cite `finger_control::patterns::extract_pattern_indices`; the live path is `finger_control::rhythm_segmentation` (mod.rs:35). The old `patterns::extract_pattern_indices` used only a fixed gap threshold with global BPM; rhythm_segmentation replaces it, adding R-based detection of rhythm changes inside continuous streams (e.g. 1/4→1/6) — per rhythm_segmentation.rs:10-12.
- **Two vs three metrics**: research notes proposed a two-metric model (MPA + Movement Scale); the prototype spec added SC as a third orthogonal axis before implementation.
- **Sliding window vs per-pattern**: the spec's sliding-window deliverable was deliberately superseded by the grilling decision (per-pattern atomic windows).
- **"No synthetic test patterns"**: the handoff decision says real beatmaps only, but the shipped unit tests construct synthetic nodes. Minor; the decision reads as about evaluation data rather than unit-test fixtures.

## Numbers / thresholds

- Diameter normalization: `108.8 − 8.96·CS` (radius `54.4 − 4.48·CS`).
- Pattern boundary: gap > ½ beat; |R| > 0.5 (≈1.4× snap ratio; catches 1/4→1/2, 1/4→1/6, 1/8→1/4; **misses 1/4→1/3 ≈ 0.415** — "tune if needed", rhythm_segmentation.rs:25-30). T threshold same value (0.5), equivalent on single-BPM maps.
- Pattern classification: ≥7 notes = Stream, ≥2 = Burst(n), slider = Slider, else Jump (rhythm_segmentation.rs:61-69).

## Open questions / next steps

- Stacked-line visualization (three synchronized time-series, shared X axis) not yet built; timeline JSON is ready (handoff-sequence-motor.md:37).
- Whether descriptors should later merge into a higher-level execution model or stay independent (open in sequence_motor_plan_research_notes_7.md).
- R threshold 0.5 misses 1/4→1/3 snap transitions; tuning considered.
- Frontend presentation of the section undecided; see [[reading-hub]] and the frontend-overhaul state.

Related: [[reading-analysis]] (module), [[forward-density]] (planned sibling metric), [[reading-hub]] (landing).
