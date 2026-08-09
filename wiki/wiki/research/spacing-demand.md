---
type: research
status: prototyped
updated: 2026-08-08
sources: [raw/spacing-demand-handoff_5.md, Temp/spacing_transition_demand_research_notes_3.md, Temp/spacing-transition-demand-candidate-analysis_4.md, Temp/spacing_transition_demand_prototype.py, backend/src/bin/prototype_spacing_demand.rs, backend/src/analysis/finger_control/rhythm_segmentation.rs, memory: spacing-demand-keep-12-snap]
---
# Research: Spacing Transition Demand

How spacing changes between consecutive notes within short burst patterns affect reading difficulty. Governing philosophy: **describe, don't classify** — continuous mathematical descriptors over the spacing sequence, no pattern-geometry labels, no hand-tuned weights (spacing_transition_demand_research_notes_3.md:23-55). Explicitly NOT randomness/entropy/surprise: predictable alternation can still be mechanically demanding (research notes:92-110). "Search for the mathematical object first, not a weighting formula" (research notes:243-250).

This lineage was later **reinterpreted and superseded by [[sequence-motor]]** (MPA/MM/SC), which is the version that shipped in `reading::analyze()`. TV2/LTD as such never reached production — see "Lineage" below.

_Citation legend: handoff = wiki/raw/spacing-demand-handoff_5.md · research-notes = Temp/spacing_transition_demand_research_notes_3.md · candidate-analysis = Temp/spacing-transition-demand-candidate-analysis_4.md · prototype.py = Temp/spacing_transition_demand_prototype.py · prototype-binary = backend/src/bin/prototype_spacing_demand.rs_

## Metrics

Primitive: second-order difference of the spacing sequence s = [s₁..sₘ], m = n_notes−1: **Δ²sᵢ = sᵢ₊₂ − 2·sᵢ₊₁ + sᵢ** — zero for any arithmetic progression; each term spans 3 consecutive spacings (4 notes) (candidate-analysis:19-30). Spacings = Euclidean distance between consecutive note positions ÷ circle diameter D (prototype-binary:40-59).

| Metric | Formula | Property | Status |
|---|---|---|---|
| **TV2** — Second-order Total Variation | Σ\|Δ²sᵢ\| (L¹, sum) | Scales with pattern length; not comparable across lengths | Design-settled, prototype only |
| **LTD** — Local Trend Deviation (bending energy) | (1/k)·Σ(Δ²sᵢ)², k = m−2 (L², mean) | Length-normalised; quadratic penalty on large reversals; recommended primary | Design-settled, prototype only |
| **LTD_rms** | √LTD | Display value; units restored to circle diameters ("10.0 = one 10D reversal per transition") | Design-settled, prototype only |

(candidate-analysis:36-78 · prototype.py:38-123 · handoff:27-31)

- **Null rules**: m < 3 spacings (Burst(2)/Burst(3)) → both scores null; top-N lists are de facto all 4-note bursts (single Δ² term) (handoff:45-48; prototype-binary:68-85).
- **Only parameters**: divide by D; stacked boundary < 0.5×D, aligned with production `streams.rs:29,35` (handoff:40-41). **No window or exponent constants** — contrast ACCV's window=4, ^1.8 (aim_control/accv.rs:32,109-111), which belongs to [[aim-control]], not spacing demand.
- **Emitted fields (proposed)**: ltd_energy, ltd_rms, tv2, n_transitions (=k), mean_spacing, deltas2_raw (candidate-analysis:207-216). Suggested JSON shape (per-pattern records + `top_tv2`/`top_ltd` + summary): handoff:93-115.
- Snap labels come from `identify_snap()` (±10% adaptive tolerance, snap_filter.rs:40-49) carried on the `Pattern` struct.

## Design principles

Both families satisfy all five principles from the design spec (describe-don't-classify, no arbitrary weights, locality, continuous, extendable to angle sequences) — verified in prototype code (prototype.py:258-297; candidate-analysis:132-149). Ranking difference: TV2 ranks "alternating big" highest (length effect); LTD ranks "big reversal" and "alternating big" equal (per-transition demand) (candidate-analysis:110-127). Pathological inputs ([0,100,0]) yield finite scores in both (prototype.py:398-416).

## Recommendation (design phase, not shipped)

**LTD primary + LTD_rms display; TV2 kept as reference** (candidate-analysis:189-205; prototype.py:468-476; handoff:32-36). Open at design time: LTD normalisation denominator k vs (k+1) (candidate-analysis:222) — prototype uses k = m−2.

## Key findings

- **Keep 1/2 snap patterns** — snap-label filtering left only **2 of 22** scorable patterns on AngelMaker — A Dark Omen (220 BPM deathcore); most high-BPM maps express spacing variation via 1/2 bursts. Accepted decision → [[keep-12-snap]]. Production wiring must NOT pass the prototype's `exclude_snaps` flag (memory: spacing-demand-keep-12-snap:14) — but the committed prototype binary still hardcodes it true (see Contradictions).
- **Real-map prototype results** (handoff:79-81): AngelMaker — 22 Burst(2/3/4) patterns, almost all 1/2 snap; top TV2 ≈ 1.27, top LTD ≈ 1.62 — very consistent spacing overall. Yoru ni Kakeru (130 BPM) — 14 patterns, wider spread; top TV2 ≈ 5.59, top LTD ≈ 31.25. Cross-check: on both maps TV2² ≈ LTD (1.27²≈1.61; 5.59²≈31.25), so each map's top score is a single-Δ² (4-note) pattern.
- **Lineage: superseded by [[sequence-motor]]** — the sequence-motor research reinterpreted "spacing transition demand" as Motor Plan Adjustment and shipped MPA/MM/SC inside `reading::analyze()` (reading/mod.rs:6,54). MPA = mean(|Δ²|) = **TV2 normalised by length** — the length-normalised L¹ variant this research anticipated. The LTD scale-blindness flaw (same-shape [1,3,1] vs [2,6,2], different difficulty) was resolved by splitting into MPA (shape) + MM (scale) (sequence-motor:16-18).

## Prototype vs production status

| Item | Status |
|---|---|
| TV2/LTD/LTD_rms formulas | Design-settled (candidate analysis + Python prototype, 2026-07-17) |
| Rust prototype binary | `backend/src/bin/prototype_spacing_demand.rs` — exists, uncommitted, compiles/runs (handoff:63) |
| Prefactor `extract_pattern_indices()` | Uncommitted WIP in `rhythm_segmentation.rs:124` (see Contradictions) |
| Production module `intra_pattern.rs` | NOT created; no spacing_demand section in reading JSON (grep backend, 2026-08-08) |
| Actual shipment | As MPA/MM/SC via [[sequence-motor]], not as TV2/LTD |

## Contradictions (file:line)

- handoff:62 — `extract_pattern_indices()` "Added" to `patterns.rs`; actual code has it in `rhythm_segmentation.rs:124` (also flagged in [[issue-3-intra-pattern-spacing]]:19).
- prototype-binary:262 — `analyse_map(&map, true)` hardcodes snap exclusion (`is_excluded_snap` = 1/1, 1/2, :34-36) vs handoff:52 "1/2 snap patterns ARE included" and [[keep-12-snap]]; the handoff's 22-pattern AngelMaker count (:80) is only reproducible with 1/2 included.
- prototype.py:458-459 — synthesis claims a 6D reversal is "4×" (TV2) / "9×" (LTD) more demanding vs prototype.py:361-362 and candidate-analysis:173-175: 5× and 25×. Docstring only; the code's own table (:347) is correct.
- handoff:81 — "1/3 triplet bursts … are the most demanding" vs handoff:46-48 — Burst(3) has no Δ² and null scores; the cited top scores (5.59/31.25) are single-Δ² values, i.e. 4-note patterns. "Triplet" terminology ambiguous.
- candidate-analysis:134 — "All six design principles" vs the design spec's five (research-notes:23-55).
- candidate-analysis:101 — "Medium reversal [1,3,1]" vs prototype.py:159 — "Medium swing [1,4,1]"; handoff:148 unit-test examples use [1,3,1].
- handoff:46-47 — Burst(2/3) "both scores are null (raw feature vector only)" vs prototype-binary:133-137 — unscorable patterns are skipped entirely (`continue`), never emitted with null scores.
- sequence-motor:17 — [1,3,1]/[2,6,2] have "identical normalized LTD score"; under the D-normalised LTD as implemented (prototype.py:91-115) they score 4 vs 16 — "identical" holds only under per-pattern shape normalisation, which the prototype never applied.

## Open questions / [gap]s

- **[gap] Frontend display undecided — blocks shipping** (handoff:129-141): per-pattern results (table/list) vs aggregate statistics? If per-pattern, top-N (5/10/20)? Primary metric TV2 vs LTD? Where in [[reading-profile]] — new card or inside an existing section? Empty state when no scorable patterns (all-Burst-2/3 maps)? Also from handoff: show spacings (×D) alongside scores? Mobile responsiveness for a long list? Recommended-but-unapproved: dedicated card, table of top patterns by LTD (Time, Snap, length, max spacing, LTD), collapsible, top-5 default (handoff:141).
- **[gap] LTD normalisation denominator** — k vs (k+1) never confirmed; prototype uses k = m−2 (candidate-analysis:222).
- Open: angle-sequence integration — identical formula on angle sequences → 2D (spacing + angle) LTD vector (candidate-analysis:224; research-notes:52-54).
- Open: real-map validation against perceived difficulty across mappers/styles (candidate-analysis:226).
- Open: identity of the "1/3 triplet bursts" that score highest (see Contradictions) — affects the "only 4-note bursts are scorable" messaging (handoff:170).

## Related

[[keep-12-snap]] · [[sequence-motor]] · [[issue-3-intra-pattern-spacing]] · [[issue-5-angle-distribution]] · [[reading-analysis]] · [[finger-control]] · [[reading-profile]] · [[forward-density]] · [[Data-Philosophy]] · [[log]]
