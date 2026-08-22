---
type: research
status: implemented
updated: 2026-08-22
sources: [Temp/handoff-three-variable-prototype_10.md, Temp/handoff-rhythm-discontinuity-integration_11.md, Temp/temporal-vs-rhythm-illustrated.md, Prototyping/rhythm_rejection - Signal feat. Such (Mameyudoufu Remix) (NekoShabeta) [Disturbance].csv, Prototyping/temporal_rejection - Signal feat. Such (Mameyudoufu Remix) (NekoShabeta) [Disturbance].csv, backend/src/analysis/finger_control/rhythm_segmentation.rs, backend/src/analysis/finger_control/snap_filter.rs, backend/src/analysis/finger_control/mod.rs, backend/src/analysis/finger_control/patterns.rs, backend/src/analysis/reading/mod.rs]
---
# Research: Rhythm Segmentation (Motor-Continuity Pattern Boundaries)

Rhythm segmentation splits a beatmap's hit objects into **patterns** for the reading pipeline using two signals: a ½-beat **gap threshold** and **rhythm discontinuity R** = |log₂(snap₂/snap₁)|. It comes from a three-variable prototype (Temporal / Velocity / Rhythm discontinuity) in which **Rhythm won** as the primary segmentation signal; the BPM snap infrastructure it required (per-timing-point lookup, ±10% adaptive tolerance) is now shared by all of `finger_control`'s snap analysis. R beats Temporal because it normalizes BPM changes out and still fires where a BPM change sits between two note pairs (T is blind there).

## The three-variable prototype

`Temp/handoff-three-variable-prototype_10.md` (supersedes the four-variable handoff `_9.md`) locked in exactly **three variables** for motor-continuity segmentation (handoff_10:12):

- **Temporal Discontinuity (Variable 1)** — *Signed* (faster vs slower matters). Δt, ratio of Δt, and log-ratio of Δt were to be scatter-plotted to pick the most informative representation; representation never decided (handoff_10:17-20). Blocked behind the BPM infra fix, then superseded by Rhythm.
- **Velocity Discontinuity (Variable 2)** — Euclidean distance `(x1,y1)→(x2,y2)`; positions normalized by CS circle diameter (`108.8 − 8.96·CS`, same formula as the rest of the codebase) (handoff_10:22-25). Not implemented; deferred to combine with Direction/Angle for intra-pattern segmentation.
- **Rhythm Discontinuity (Variable 4** — numbered as in the handoff, leftover from the four-variable scheme**) — Ratio-based: `log2(snap2/snap1)` as a continuous signed value; **note-pair-level** granularity (pattern-level aggregation "later") (handoff_10:27-30).
- Direction/angle (the four-variable scheme's Variable 3) — deferred entirely to `Temp/angle-direction-research-notes.md` (handoff_10:13, 42-46).

The prototype's critical path was the **BPM snap fix** (handoff_10:32-36): find the active uninherited (red line) timing point at each object via `rosu_pp::Beatmap::timing_points`; replace the global `map.bpm()` + ±12 ms tolerance with per-timing-point beat_len + **±10% adaptive tolerance** (`beat_len × fraction × 0.10`); export per-adjacent-pair snap; remove the dead burst-histogram computation.

## Deferred velocity + direction framework

The deferred velocity note adds an important design boundary to the prototype history. Velocity discontinuity is the absolute change in CS-normalized cursor speed between adjacent note pairs, not raw movement speed. For each note pair, use Euclidean movement distance divided by elapsed time, normalize distance by circle diameter `D = 108.8 - 8.96·CS`, then compare consecutive normalized speeds as `|Δv|`. The experiments found that this signal is useful for detecting spatial substructure inside an already rhythmically coherent pattern, but it is not reliable as a standalone pattern-boundary detector.

The proposed follow-up is a joint velocity + direction/angle signature: rhythm/temporal signals establish coarse pattern boundaries first; velocity and direction then describe or subdivide intra-pattern structure. Candidate descriptors include speed-change magnitude, direction-change magnitude, and their local co-occurrence. This remains design-only: no production segmentation rule, threshold, JSON output, or frontend contract has been adopted. The angle work is separately deferred to [[angle-distribution]]. Source: `Temp/(deferred_combine with directional discontinuity or angle stuffs) velocity-spatial-discontinuity-framework.md`.

## Discontinuity definitions: R vs T

- **R** = |log₂(snap₂/snap₁)|; each pair's snap is `Δt / beat_len` using the timing point active at the **second** note's timestamp (temporal-vs-rhythm-illustrated.md:7-8; snap_filter.rs:69-73). Continuous fraction, not the quantized label.
- **T** = |log₂(Δt₂/Δt₁)| — pure time ratio, no BPM/timing-point information (rhythm_segmentation.rs:172-175).

Worked examples (temporal-vs-rhythm-illustrated.md):

| Example | T | R | Meaning |
|---|---|---|---|
| 1. Single BPM, 1/4→1/2 | +1.00 | +1.00 | Both detect the boundary |
| 2. BPM change, snap constant | spikes at the transition note | same | — |
| 3. BPM change **between** two pairs | **0.00 flat** | **+0.585** | Only R sees it; T−R = −0.585 = log₂(333/500), a pure BPM-change detector |

Recommendation (temporal-vs-rhythm-illustrated.md:139): **use R as the primary signal**; T is only a fallback when timing points are unavailable. T−R was archived as a BPM-change detector (handoff_11:14; `Temp/temporal-and-tr-deferred-reference.md`).

## Prototype results

Two CSVs in `Prototyping/` (both named "Signal feat. Such (Mameyudoufu Remix) (NekoShabeta) [Disturbance]"), produced by the comparison-study binary `bin/prototype_segmentation.rs` (deleted after serving its purpose, handoff_11:48). 175 BPM throughout (beat_len 342.8571 ms in every row), **1565 adjacent note pairs**.

- **T ≡ R on single-BPM maps**: the two CSVs' `log2_ratio` columns are numerically identical in all 1565 rows (row-by-row diff = 0). Prototype-scale confirmation of rhythm_segmentation.rs:35.
- **|R| distribution** (log2 ratio): 1220 rows in [0, 0.05), 2 in [0.05, 0.1), 0 in [0.1, 0.3), 2 in [0.3, 0.5), 17 in [0.5, 0.6), 198 in [0.6, 1.1), 114 in [1.1, 2.1), 12 ≥ 2.1; max 6.98.
- **Threshold 0.5 → 341 boundaries (21.8%), 1224 non-boundaries (78.2%)**. Only 4 of 1565 rows fall in (0.05, 0.5) — the 0.5 threshold sits in a nearly empty band between "same snap" (≈0) and "snap change" (≥0.5) on this map.
- Snap fractions are continuous with jitter around clean values (0.2479, 0.2508, 0.4987, 0.7496, 1.0004). Note: **3/4 (0.75) is not in the SNAPS label table** (snap_filter.rs:14-21), so those pairs fall to off-grid handling.
- Threshold coverage: catches 1/4→1/2 (2×, R=1.0), 1/4→1/6 (1.5×, R=0.585), 1/8→1/4 (2×); **misses 1/4→1/3 (1.33×, R≈0.415)** (rhythm_segmentation.rs:28-29).

## Integration (handoff_11) and what shipped

Prototype → production integration (`Temp/handoff-rhythm-discontinuity-integration_11.md`): Rhythm chosen as primary; T rejected (false boundaries on multi-BPM maps); Velocity deferred; BPM infra (per-timing-point lookup + ±10% tolerance) validated and merged; `rhythm_segmentation` module created and registered (mod.rs:7); reading pipeline switched to it (reading/mod.rs:35); now-dead `extract_pattern_indices` removed from patterns.rs.

Segmentation algorithm (rhythm_segmentation.rs:124-168): boundary array from (1) gap > `beat_len/2 + 10 ms` at the second note (line 145), (2) R > 0.5 at middle note (lines 153-164); then `group_into_patterns` classifies: ≥7 notes = Stream, ≥2 = Burst(n), singleton slider = Slider, else Jump (rhythm_segmentation.rs:61-69).

### Prototype vs production status

| Item | Prototype spec / handoff | Shipped code (verified 2026-08-08) |
|---|---|---|
| R formula | `log2(snap2/snap1)` signed, pair-level (handoff_10:28-30) | Same, with abs + non-finite guards — rhythm_segmentation.rs:156-157 |
| R_THRESHOLD | 0.5 (handoff_11:36-38) | `const R_THRESHOLD: f64 = 0.5`, "Tune if needed" — rhythm_segmentation.rs:25-30; tuning vs in-game editor still open (handoff_11:73) |
| T_THRESHOLD | comparison-only | 0.5 — rhythm_segmentation.rs:32-36; `extract_pattern_indices_temporal` shipped but used only by prototype_sequence_motor.rs:125 |
| BPM source | per-timing-point lookup (handoff_10:33) | `timing_point_at` binary search — snap_filter.rs:29-34 |
| Snap tolerance | ±10% adaptive, "tune the percentage afterwards" (handoff_10:34, 52) | `TOLERANCE = 0.10` — snap_filter.rs:24, 40-48; the promised ±12 ms off-grid comparison never documented |
| Pair-level snap export | per-adjacent-pair (handoff_10:35) | `note_pair_snaps` — snap_filter.rs:58-77 |
| Gap boundary | ½ beat (handoff_11:35) | `beat_len/2 + 10.0 ms` — rhythm_segmentation.rs:145 |
| Velocity discontinuity | spec'd (handoff_10:22-25) | **Not implemented** — deferred with Direction (handoff_11:13, 75) |
| Temporal as primary | prototype candidate (handoff_10:17-20) | Rejected; R wins (handoff_11:11-13) |
| Dead burst histogram | remove (handoff_10:36-37, 70) | snap_filter.rs:83 returns empty HashMap; **mod.rs:39-45 `bursts.clear()` + repopulate still present** (see Contradictions) |
| 3-signal visualization | 3 synchronized line charts (handoff_10:80, 99) | Not built — `[gap]` |
| Prototype files | — (handoff_11:40-49) | Deleted: `reading/snapper.rs`, `discontinuity_{temporal,velocity,rhythm}.rs`, `proto_angle.rs`, `prototype_segmentation.rs` |

## Two parallel pattern-classification paths

Two live segmentation paths now produce overlapping output from the same beatmaps; their divergence is **untested** (index.md gap "2 parallel pattern-classification paths"). **Decided 2026-08-10: unify finger control analysis on the updated segmentation** — [[segmentation-unification]] (lands with the production port).

- **Path A — `finger_control::patterns::extract_patterns`** (old, still in `finger_control::analyze`): global `map.bpm()` (patterns.rs:56), gap `ms_per_beat/2 + 10` (patterns.rs:57), `identify_snap` with the **global** beat_len (patterns.rs:97, 116), slider-swallowing into bursts within a 25 px proximity threshold (patterns.rs:58, 81-93). Caller: `finger_control::analyze` (mod.rs:34) → transitions matrix (mod.rs:35), timeline (mod.rs:50). Returns `Vec<Pattern>`, no index ranges.
- **Path B — `rhythm_segmentation::extract_pattern_indices`** (new, drives reading): per-timing-point BPM, ±10% tolerance, gap + R boundaries; also `extract_pattern_indices_temporal` (rhythm_segmentation.rs:181). Caller: `reading::analyze` (reading/mod.rs:35) → `sequence_motor::analyze_patterns` (reading/mod.rs:44). Returns `Vec<(Pattern, Range<usize>)>`.
- Both emit the **same `Pattern` type** from the shared enum (patterns.rs:5-10; rhythm_segmentation.rs:22) — structurally interchangeable outputs. rhythm_segmentation.rs:10-12 documents that it "replaces the old `patterns::extract_pattern_indices` approach" (fixed gap + global BPM).
- The old `extract_pattern_indices` was removed from patterns.rs (handoff_11:28); only `extract_patterns` remains there.
- Handoff's own open item: "Replace `finger_control::patterns` entirely … Could be migrated to `rhythm_segmentation` later" (handoff_11:76).
- Prototype binaries all use Path B: prototype_spacing_demand.rs:122; prototype_sequence_motor.rs:125-127.

## Contradictions

1. **"Catches ≥1.4×" is off by a hair**: handoff_11:38 and rhythm_segmentation.rs:27 say threshold 0.5 catches "snap ratio changes ≥1.4×", but log₂(1.4) = 0.485 < 0.5; 0.5 actually catches ratios ≥ ~1.414×. Cosmetic fuzz repeated in both handoff and code comment.
2. **Dead-burst-removal half-done**: handoff_10:70 says remove "the `bursts.clear()` + repopulate pattern" in mod.rs — but mod.rs:39-45 still contains exactly that block (only the snap_filter.rs side was completed: empty HashMap at snap_filter.rs:83; handoff_11:17 claims only the snap_filter part).
3. **Plan vs execution sequencing**: handoff_10:20 says Temporal "not yet implemented", but handoff_11:44 reports a prototyped `reading/discontinuity_temporal.rs` (archived) and the CSVs contain full T signals — handoff_10 is the plan, handoff_11 the execution report; not a live contradiction, flagged for provenance.
4. **Stale docstring**: sequence_motor.rs:172 still cites `finger_control::patterns::extract_pattern_indices` as its pattern source while the live path is rhythm_segmentation (reading/mod.rs:35) — already flagged in [[sequence-motor]].
5. **Minor**: handoff_10:36 cites the dead burst histogram at "snap_filter.rs:49-64"; no such code exists at those lines in the current file (removed; file renumbered).

## Open questions / `[gap]`s

- **R_THRESHOLD tuning**: run against several beatmaps and compare with in-game editor (handoff_11:73). Only one map's signal data survives on disk (the two CSVs).
- **±10% tolerance never tuned**: off-grid-rate comparison vs the old ±12 ms never documented (handoff_10:52) → `[gap]`.
- **Path A vs Path B divergence untested** (index.md gap); **decided 2026-08-10: unify finger control on the updated segmentation** — [[segmentation-unification]]; replacement of `finger_control::patterns` lands with the production port (handoff_11:76).
- **1/4→1/3 snap transitions missed** by R_THRESHOLD 0.5 (rhythm_segmentation.rs:29).
- **Velocity + Direction intra-pattern segmentation** — framework in `Temp/velocity-spatial-discontinuity-framework.md`, needs its own session (handoff_11:75).
- **Slider detection in segmentation** deferred to the angle session; singletons currently classified only by type (handoff_11:74).
- **Three-signal dataviz** (boundary-distribution visualization) never built (handoff_10:80; handoff_11:82).
- **Frontend impact unassessed** — reading JSON now uses rhythm-based segmentation; the `sequence_motor` section output format is unchanged, but "Frontend may need updates if pattern counts changed noticeably" (handoff_11:77) → `[gap]`; see [[finger-control-profile]] / [[reading-profile]].
- **T−R BPM-change detector** archived (handoff_11:14) — no session planned to use it.

## Deferred (2026-08-09): slider-chain motor model — waypoint design

From the YOASOBI editor-verification session: consecutive sliders get classified as Stream because multi-note classification is count-only (rhythm_segmentation.rs:61-69; `is_slider()` only for singletons). User's motor model: a slider chain is NOT point-to-point — the cursor must follow **head→tail along the curve**, then travel **tail→next head**. Slider chains are at least as demanding as circle runs, not less.

- **Waypoint model (design, deferred):** each circle = 1 waypoint (position); each slider = 2 waypoints (head + tail). Spacing = distance between consecutive waypoints (true traversal path). MPA/MM/SC formulas unchanged, computed over the expanded sequence. Tail positions via slider curve (precedent: `sliders.rs:17` `expected_dist`; handoff_10 Option B `SliderPath.curve()`). Prerequisite when returning to sequence-motor work — without it, slider-chain MM/MPA understate demand (head→head < head→tail + tail→head).
- **Open (deferred): SV-based segmentation of slider chains.** User observed a 4-slider run where one slider has body length 171 vs 102 (≈1.7×), read as "1.9× SV" in the editor. Codebase status (verified 2026-08-09): per-object velocity NOT parsed by rosu_pp-4.0.1 (Slider struct has no `velocity`; TimingPoint is `{time, beat_len}` only); **effective speed IS capturable via `expected_dist` / duration** (present on rosu_pp's Slider and on osu_map_analyzer's SliderPath used by sliders.rs:17). Deferred — unnecessary for the classification fix.

## Experimental type-boundary segmentation (2026-08-09/08-10, prototype-only)

`backend/src/bin/prototype_sequence_motor.rs --exp` implements the agreed asymmetric type-boundary rules; production `rhythm_segmentation.rs` stays untouched until editor cross-checked (user directive 2026-08-09).

- **Type rules** (boundary OR'd with gap + R/T, unchanged from production): slider→circle = **always** boundary; circle→slider = boundary iff following slider run ≥2, else a lone near slider **engulfs** into the preceding circle group; spinner/hold adjacency = boundary; pure slider runs ≥2 = **`SliderChain`** (fixes the count-only "slider runs ≥7 → Stream" bug at rhythm_segmentation.rs:61-69).
- **2026-08-10 — R/T suppressed at slider→circle transition windows** (`skip_discontinuity`): R at the first circle of a run after a slider chain compares the 1/2 transition pair vs the run's 1/4 pairs → R=1.0 → boundary lands one note *inside* the circle run (YOASOBI 974/792: the lone "Jump" was the run's first circle). The type rule already bounds the run at the transition; R's second boundary was the bug. Asymmetric: circle→slider windows **keep** R (isolates the transition slider, e.g. 964). Verified: `[792..804]` 12 circles whole, `[974..982]` 8 circles whole, `[964][965..974 SliderChain][974..982]` in the 03:38 section.
- **2026-08-10 — engulf proximity = 2× circle diameter** (was 25px): trailing slider heads measured 38.9–116 px in vs ≥185 px out on YOASOBI (CS4 → ~146 px). 25px sits *inside* one circle radius — too strict; 7 stream counts lost their ending slider. **Dataset retracted 2026-08-10 evening** (transcription artifact) — true head distances: 8-in 0–116.5 px, 5 verified-out 38.9–97.8 px, genuine ≥185 px set = 30 sliders; see [[run-start-engulf-known-limits]].
- **Run-start sliders stay out (2026-08-10 decision, locked evening):** a stream's ending slider that starts a following slider run (≥2) keeps the boundary — the user's verified 02:53 count (804 out, "12 Circles") is itself a run-start case, structurally identical to the 02:37 case. 02:37 stays 18 in exp-mode vs the user's 19 (production R-mode already yields 19 — the port resolves it). The proximity-override alternative was evaluated and rejected — [[run-start-engulf-known-limits]].
- **Known limitation (accepted 2026-08-10):** 02:08:539 — R fires at a *pure* rhythm change (circles 1/4→1/2 at a stream's end, no type transition). Pre-existing in both versions; fixing = revisiting R policy for stream endings (deferred).
- **New-combo = reference column only** (decision 2026-08-10): NC never a boundary signal; raw-.osu parse adds `new_combo`/`mid_combo_breaks` to the `--patterns` dump. All five run-start sliders on YOASOBI carry NC — explains editor combo colors, not counts.
- **Verified:** 11/11 unit tests; YOASOBI 549 patterns + Signal 413 patterns, 0 structural violations (no interior s→c, no non-trailing c→s, SliderChain labels correct); 7 of 8 stream counts match editor counts (02:37 flagged).
- **2026-08-10 (evening) — 8 new c→s engulf mismatches; distance-data correction.** User cross-checked 8 more cases (00:13–00:42, all 1/4 rhythm) expecting the slider to engulf. Verified from .osu raw coordinates: **all 8 heads are distance-clean — 7 are perfect 0px stacks** (slider head exactly at the last circle's position; case 4 = 116.5px, the only non-stack). The per-case distances quoted earlier (79.2–194px) were **transcription errors** — the measuring script reports each row's distance to the NEXT object; the quoted figures were the sliders' outgoing distances (case 8's "194px" = stack→second-slider-head). **The "trailing heads 38.9–116px in vs ≥185px out" dataset behind the 2× diameter decision came from the same pipeline → retracted after re-verification** (see [[run-start-engulf-known-limits]]). 02:08:539's "437px" also wrong (actual 36px — acceptance stands on rhythm, not distance).
- **Cause breakdown of the 8:** 6 = run-start sliders (type rule `run ≥ 2` is the sole blocker — heads sit ON the circles); case 6 = s→c suppression over-fires (kills R at k=152; transition gap 346ms > ½ beat; un-suppressing at genuine gaps fixes case 6 but breaks case 8 — R at k=165, gap 692ms, would split the wanted 3n; **no gap threshold satisfies both**); case 5 = R boundary at k+1=141 vs user's k=140 — no signal fires at 140 → **unreachable**.
- **Override evaluated (evening, simulator validated byte-identical to the exp artifact):** "proximity overrides run-start" (boundary iff head > 145.92 px) fixes 7 of the 8 (case 5 unchanged) + matches 02:37→19, but breaks 6 verified reads — 02:53→13, 02:03→37, 01:26→17, 03:55→13 **plus 2 newly found at 03:38** (964 solo slider absorbed into a 2n burst; 982 swallows the verified 8-circle run 974..982 → 9). Map-wide 549 → 471 patterns. **NC bits confirmed:** 8-in all type 2, 5-out all type 6 (incl. 739) — a perfect *static* split that still fails as perception (user counted NC-carrying 739 IN; verified no-NC 964 OUT). No signal discriminates the sets (distance/NC/rhythm/following-slider all overlap or fail).
- **Resolved (2026-08-10 evening):** user accepted the 8 mismatches as known limitations — override **rejected**, suppression refinement **abandoned**, ≥185 px dataset **retracted**, 2× diameter threshold stands. Decisions: [[run-start-engulf-known-limits]] · [[suppression-refinement-abandoned]]. Full detail on [[2026-08-10-handoff]].
- **Other-map cross-check (2026-08-10 evening, 3 maps, sub-agents):** the 0px-stack→run-start symptom reproduces at scale with TWO engines — Signal [Disturbance] (175 BPM, CS 4.2): 26 stacks, **18 typed run-start, all type-rule (run ≥ 2 clause)**; the 8 engulfed stacks are all run=1. Feral [Veracious] (160 BPM — beat_len 375 ms — CS 3.9): 9 stacks, reported "all typed run-start by the GAP rule", but the gap threshold is **197.5 ms** (beat_len/2 + 10) and heads sit 187/94 ms after the circle — 187 ms does NOT fire the gap rule; **the "all gap-rule" classification is suspect — re-verified 2026-08-11** (see Feral diagnosis below; the earlier "375 BPM / 90 ms" numbers were a beat_len-vs-BPM misreading). Heart Pie Dancehall (425 BPM, CS 4): 1 stack typed singleton (638 ms pause — correct split), no type-rule stacks. Engulf fires correctly for rhythm-adjacent lone sliders everywhere (Signal 8/8 run-1 stacks, Feral 52/52, Heart Pie 53). Excels for the user's walk: `Prototyping/{signal_disturbance,feral_veracious,heart_pie_dancehall}_patterns_exp.xlsx`.

## Feral 1/3–1/6 diagnosis (2026-08-11) — R_THRESHOLD 0.35 adopted + verified; pivot rule abandoned

7 error groups from the user's Feral [Veracious] Excel walk, all at 1/3–1/6 subdivision transitions, diagnosed with raw-map evidence (`Temp/extract_feral_neighborhood.py` = per-pair deltas + snap fractions; `Temp/extract_feral_nc.py` = NC bits; `Prototyping/feral_veracious_patterns_exp.json` = actual exp output).

- **Class A (errors 1–6): boundary-placement convention.** R fires at subdivision changes (2×/1.5× ratios) but places the boundary AFTER the pivot note (k+1). User's convention: the pivot belongs to the FASTER side. **NC bits validate mapper intent 7/7** — pivots 51360, 52860, 55110, 63360, 125985, 129735 all carry new-combo (mapper starts the new run ON the pivot). Slow-down sites (E3's correct row, 1/6→1/3) already conform to k+1. **Fix (pivot rule) ABANDONED 2026-08-11** — no universal signal separates mapper conventions; Class A remains a known limitation (`abandoned/abandoned_pivot-rule.md`).
- **Class B (errors 7a/7b): threshold.** The 24n/28n streams are 1/4+1/3 (not 1/6+1/4); 1.33× change → R = 0.415 < 0.5 → never fires → merged streams + "Unstable" avg-snap label. Fix: R_THRESHOLD → 0.35 (between jitter noise ~0.29 and signal 0.415).
- **Class C:** "Unstable" single-snap-per-pattern labeling dissolves once streams split correctly (no separate fix).
- **Class D:** error 6's engulf failure — accepted imperfection, untouched.
- **Buffer question answered:** the "memory processing buffer" was never implemented (max window = 3 objects; R = single-pair ratio). NOT the missing piece — A and B are. Buffer remains a later robustness upgrade (windowed rate medians for jitter immunity).
- **Decisions (user, locked 2026-08-11, evening update):** **R_THRESHOLD 0.35 adopted + VERIFIED** — canary PASS (YOASOBI 549/549 row-identical), Feral Class B fixed (385 = 379+6, all 1/4↔1/3 splits), user in-game cross-check clean. **Pivot rule ABANDONED** the same day (user: "I give up optimizing on pivot rule") — canary-FAILED on YOASOBI (106 flips incl. 6 pickup-joins), no clean separator (NC-gate dead, combo-boundary gate partial); see `abandoned/abandoned_pivot-rule.md` (repo root). Class A boundary placement = known limitation. Production port (0.35 + exp rules, NO pivot) in progress. Experiment entries: prediction | kill-criteria | outcome | verdict on [[2026-08-11-handoff]].
- **BPM correction (supersedes the 08-10 cross-check bullet):** Feral beat_len = 375 ms = 160 BPM; gap threshold 197.5 ms; notes 125 ms (1/3), 62.5 ms (1/6), 93.75 ms (1/4), 46.9 ms (1/8); green lines SV-only. "9 stacks ALL gap-rule" suspect (187 ms < 197.5 ms) — re-verify.

Related: [[finger-control]] · [[reading-analysis]] · [[sequence-motor]] · [[spacing-demand]] · [[reading-hub]]
