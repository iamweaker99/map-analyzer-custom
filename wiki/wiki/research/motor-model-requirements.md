---
type: research
status: idea
updated: 2026-08-12
sources: [Prototyping/yoasobi_collab_extra_motor_patterns (success criteria here).xlsx, Prototyping/51_test_run_sample.json, wiki/wiki/research/sequence-motor.md, 2026-08-12 discussion (user, quoted in log)]
---
# Research: Motor Model Requirements (R1–R9) + Assumptions & Hypotheses

> **2026-08-13 — list demotion:** the A/H/AA/HH lists on this page are **reference-only**. The active hypothesis-clearing basis is [[motor-metrics-oiah]]; requirements R1–R9 below stay **authoritative**. Use the demoted lists for gap-hunting after the OIAH clears (milestone on that page).

The motor measurement framework for the reading analysis, derived from the user's **51 tagged hard sections** ([[difficulty-philosophy]] gives the axioms). Current status: requirements discussed with the user (2026-08-12); hypotheses ([[motor-metrics-oiah]]) proposed — awaiting validation before any feature implementation (role-split mode: main = research agent, builder sub-agents code later, critic kills hypotheses pre-build).

## Ground truth

- `Prototyping/51_test_run_sample.json` — 51 sections (S001–S051), 85 pattern rows, 3 tiers: **purple (22 rows / 14 sections) > orange (33 / 17) > aqua (30 / 20)**; each section has a mapper note (verbatim "why & how it is hard").
- Sections span multiple patterns (e.g. S031 = Jump + Stream + 4 Jumps) or live in one. The 549-row Excel is the source; the sample file is the trial input from now on.
- Note vocabulary clusters by tier: "abrupt" 5 purple / 3 orange / 1 aqua; "self-overlap" 5 purple / 4 orange / 0 aqua; row 542 annotated "the hardest"; 36-note stream 294 (minor sections, smooth joins) is orange → **join severity matters more than section count**.

## Why the current model cannot capture these

MPA / MM / SC ([[sequence-motor]]) are **spacing-only per-pattern aggregates**: MPA = mean(|Δ²spacing|) (0 if <4 notes), MM = RMS spacing (0 if <2), SC = coefficient of variation (0 if <3). Consequences:

- **Geometry-blind by construction** — no angles, directions, or positions: a uniform-spacing zig-zag ≡ a uniform-spacing straight line.
- **MPA is literally a mean** — a sharp local spike is diluted by the pattern length by design.
- **SC is a drift metric** — cannot separate gradual drift ("spacing gradually increase") from abrupt discontinuity ("half stacked → merely touched then back").
- The "hardest" section (S042, 542) scores mid-range (MPA .358 / MM .952) while untagged rows reach 4.7 — no obvious separation.

## Requirements (discussed, with user verdicts 2026-08-12)

| # | Requirement | User verdict |
|---|---|---|
| R1 | **Turn-angle sequence** at note resolution (not just its aggregate) — the sequence is where context and momentum live; a sudden change against that context is what makes it difficult | ✓ agreed; optimization details deferred — design direction first |
| R2 | **Geometric section segmentation + join severity**: segment into minor sections (direction consistency), measure the join (angle + spacing discontinuity at the boundary, snap-normalized). Join severity > number of sections | ✓ agreed; **join severity is important — frequent at higher star ratings**; geometry via low-level CAD-style descriptors, never a geometry database |
| R3 | **Self-overlap / crossing detection**: non-adjacent pair distances, polyline crossing, boundary-anchored (notes: "self-overlap when hitting the slider head") | ✓ agreed, with traps: the perceived-polyline time window is tricky; refer to official osu! reading-skill research (AR/approach circles; **no mods**); design trap — edge vs center crossing, how much it crosses |
| R4 | **Spacing dynamics**: sequence dᵢ, first/second differences, stack threshold, stacked↔spread transitions; **chaos-of-change statistics** (monotonic increase vs increase-then-decrease) | ✓ agreed — search statistical qualities that measure *how chaotic the value change is*; also **spreading direction**: wiggle+spacing-increment (diverges into two directions) is harder than curve+spacing-increment (one direction) |
| R5 | **Direction-oscillation structure**: turn-sign sequence distinguishes line / curve / wiggle / zig-zag / V / spiral; chirality flips (cw↔ccw) | ✓ agreed — the descriptors that replace hard-coded geometry; derive how difficulty is caused or correlated with them (causation/correlation study — now or when physics quantities enter) |
| R7 | **Window/subset locality**: difficulty can concentrate in a subset of a section (S042: "the first two spaced diagonal/spiral is one of the hardest pattern") or distribute evenly (S021, 36 notes); per-section averages dilute concentrated difficulty; short sections (2–4 notes) are internally degenerate (MPA needs ≥4) → lean on boundary/join features | ✓ explained + agreed (see below) |
| R8 | **Snap-window normalization**: join/reset quantities expressed against the time budget of the snap (1/4, 1/2 → ms via BPM); "more motor adjustment under same hit window" | ✓ agreed |
| R9 | **Vocabulary-mapping output**: every analysis expressible in the notes' language (spiral with self-overlap, abrupt cw→ccw join, exponential spacing increase) so the model can be judged note-by-note | ✓ agreed |

*R6 (placement quality / dirty) dropped 2026-08-12 (user).*

**Terminology (2026-08-12):** *joins* (R2/H2) = the boundaries between patterns, or between minor sections within a pattern — a momentum disruption, the difficulty signal. This is distinct from the angle slice's old "transition notes as outliers" (angle PRD): those were *filler notes* — random notes before/after breaks or spinners (typically 1/1–1/2 snap), disconnected from any pattern or jump chain. They were excluded there because they belong to no pattern, not because boundaries were considered easy.

**R7 explanation (as given 2026-08-12):** the difficulty of a section is not necessarily spread evenly through it. S042's note says the *first two* spaced diagonal/spiral sub-patterns are the hardest part; the rest (stacked a bit + flatten wiggle) is easier — a player experiences the peak, but a per-section average reads "moderate". S021 (294) is the opposite: 36 notes of consistently moderate difficulty, where the average is faithful. So the framework needs both the section's **average level** and its **concentration** (max-window / peak). Short sections (2–4 notes) cannot support internal statistics (MPA needs ≥4 second differences) — their difficulty lives at their boundaries, so they lean on join features. This also connects to segmentation's purpose: the geometric segments are candidate **context windows**, and the ideal window size is unknown → trial.

## Two-level model shape (agreed)

- **Level A — section profiles** (per-section aggregates): shape class (R5), spacing trend (R4), section count. Trend/sum lenses fit here.
- **Level B — event register** (per-boundary / per-window events): join severity (R2), overlaps (R3), turn spikes (R1). Peak/count lenses fit here — the current plan is blind to this level.

## Assumptions (reference — axioms, not tested; demoted 2026-08-13)

- **A1** — Difficulty lives in the (trajectory, velocity) pair: same geometry, different speed → different difficulty (race-track axiom, [[difficulty-philosophy]]).
- **A2** — Motor adjustment is the failure model: the player carries momentum; any break forces a cursor reset; difficulty = cost of resetting under the time budget.
- **A3** — Difficulty is a relative/local spike, not a global magnitude.
- **A4** — Direction-agnostic: a metric's difficulty direction (high and/or low) is metric-specific.
- **A5** — No fused single score; multi-perspective output. *(was A8; renumbered 2026-08-12)*

*Removed 2026-08-12 (sample facts / project decisions / testing instructions, not axioms): the 51-section unit of analysis and the 51 notes as ground truth (→ Ground truth above); slider-body exclusion and no-mods (→ [[difficulty-philosophy]]).*

## Hypotheses (reference — proposed, not validated; demoted 2026-08-13 → [[motor-metrics-oiah]])

- **H1 — Geometry blindness of MPA/MM/SC:** the current triple does not separate tagged sections from the 464 untagged ones (spacing-only ⇒ geometry-blind). Counter-evidence (a clean separation) would refute.
- **H2 — Join severity ranks tiers:** join severity (direction + spacing discontinuity at section boundaries, snap-normalized) orders purple > orange > aqua and is the dominant signal in purple sections.
- **H3 — Lens per feature class:** event-family features separate better under peak/count lenses (max, p95, count-over-threshold); drift-family features under trend/sum lenses. Per-feature lens choice, not one lens.
- **H4 — Relative spike:** a feature is hard when it deviates from the local context (momentum built in the preceding run), not when it is absolutely large — operationalizable as deviation from a local trend model (join angle vs the section's own direction; spacing jump vs local spacing variance).
- **H5 — Segmentation context window:** features computed inside geometric segments beat whole-pattern features; window size matters and is to be trialed (structural-fact-2 purpose of segmentation).
- **H6 — Velocity-aware features:** velocity-aware quantities (spacing per ms — speed) separate better than static geometry; same trajectory at different BPMs scores differently.
- **H7 — Crossing severity window:** crossing/self-overlap is a reading difficulty only within a close-proximity, short-time window (AR-constrained perception); edge-vs-center and crossing depth/count matter. Trap: naive crossing counts over-flag untagged patterns. ***(deferred 2026-08-12 with Q1/Q2 — polyline-trajectory + self-overlap detection and the AR/approach-circle basis are parked)***
- **H8 — Spread direction:** two-direction divergence (wiggle + spacing increment) is harder than one-direction (curve + spacing increment) — a spacing × direction interaction.
- **H9 — Chaos of change:** alternation/oscillation statistics (direction flips of Δspacing, angle-sign flips, run lengths) capture R4 difficulty better than net/magnitude statistics. *(gated 2026-08-12 — see deferred statistical-qualities search below)*
- **H10 — Trajectory-run detection from shape descriptors (R5):** geometry detection from low-level descriptors — the turn-sign sequence sᵢ ∈ {+, −, 0}: line (no sign changes), curve (consistent sign), V (one large flip), wiggle (frequent small flips), zig-zag (frequent large flips), spiral (consistent sign + decreasing radius + possible self-overlap); plus chirality flips (anti-clockwise → clockwise, notes 157, 542 — a subset of R1/R2 but detectable as sign flips of curvature) — uncovers the trajectory runs in sequence, enabling accurate physics measurement (velocity/momentum per run). This is how the model speaks the mapper's language. *(replaces the dirty-placement hypothesis — R6 dropped 2026-08-12)*
- **H11 — Concentration:** section difficulty = f(average, concentration) — S042-type (concentrated) vs S021-type (distributed) sections must rank differently.

## New-list items (reference — AA/HH separate lists; demoted 2026-08-13)

From the gh#5 angle-distribution review (issue + [[angle-distribution]] chain + deferred Temp frameworks). Reference-only since 2026-08-13 — superseded as clearing basis by [[motor-metrics-oiah]]; gap-hunt input after that page clears.

**New assumptions (accepted, not tested):**

- **AA1** — Angle and spacing are correlated, not causative: the spacing-only triple (MPA/MM/SC) is a *diluted proxy* for geometry, not fully blind — H1's "geometry-blind by construction" reads stronger than the evidence warrants. *(was C1; from angle PRD / [[Data-Philosophy]])*
- **AA2** — Raw interior angle is scale-blind: a 90° turn across 5px vs 100px is not equal motor difficulty; equal angles at different segment lengths are not equal — angle features need magnitude context ("spatial significance"). *(was N2; from `Temp/(deferred) angle-direction-research-notes.md` flaw 1)*
- **AA3** — Angle magnitude and turn-change (predictability) measure different axes; both are needed — "confirmed no significant overlap" on raw distributions only (AngelMaker mean 56°/median 24° vs YOASOBI 77°/78°), never checked against difficulty tiers. *(was N1; from `Temp/prototype-discussion-01-angle-buckets.md`)*

**New hypotheses (to validate):**

- **HH1** — Velocity discontinuity defines intra-pattern boundaries: Δv between consecutive notes (normalized by circle diameter) segments a single rhythmic pattern into spatial sub-patterns; a joint f(Δv, Δangle) spatio-temporal signature captures intra-pattern structure ("core discovery" of the deferred velocity framework). Fork for H10: geometry descriptors (H10) vs physics Δv (HH1) as run-boundary signals. *(was N3; from `Temp/(deferred_combine...) velocity-spatial-discontinuity-framework.md`)*

## Open questions & deferred work (resolutions 2026-08-12)

- **Q1 — Two time windows** → **deferred.** The reading window (how far ahead the player can plan, AR/approach-circle based) vs the motor window (the snap-based execution budget): which window does each feature use? Polyline-trajectory and AR-based topics are parked — including the official osu! reading-skill research suggestion (the AR/approach-circle basis).
- **Q2 — Perceived-polyline window for crossing** (R3) → **deferred** — same bucket as Q1: self-overlap / crossing detection is parked.
- **Q3 — Control set** → **resolved (implementation deferred).** Baseline = all 464 untagged rows. Revision trigger: once a measurement system exists that separates the color-tier sets — at minimum it must recognize purple-tagged sections as all harder than aqua-tagged ones — Q3 is revisited.
- **Q4 — Physics quantities** (centripetal v²κ, jerk at corners; causation/correlation study per R5) → **deferred** until the H10 / geometry-descriptor work.
- **Statistical-qualities search (R4, chaos-of-change statistics)** → **deferred.** The alternative — statistics computed on physics quantities (once H10 enables measuring them) — may also make things worse; revisit only when all current hypotheses are cleared.

## Research-loop rules (2026-08-12, amended 2026-08-13)

- **2026-08-13 amendment (user):** A/H/AA/HH demoted to reference; active clearing basis = [[motor-metrics-oiah]]. New discoveries while clearing → new OIAH hypotheses (Hyp.8+); gap-hunt leftovers → new reference notes here. Coverage contract: hypotheses must serve R1–R9 ([[motor-metrics-oiah]] § Requirements coverage).
- New assumptions/hypotheses discovered while clearing the current lists go on **separate lists** — new assumptions as **AA1…**, new hypotheses as **HH1…**; never merged into the current A/H lists. *(superseded for the active basis by the 2026-08-13 amendment; retained as the rule for reference-note growth)*
- No feature implementation until the hypotheses are discussed (role-split mode: main = research agent; builder sub-agents code later; critic kills hypotheses pre-build).

Related: [[difficulty-philosophy]] (axioms) · [[sequence-motor]] (current MPA/MM/SC) · [[2026-08-12-handoff]] (session state) · [[experiment-protocol]] (paused kill-criteria loop).
