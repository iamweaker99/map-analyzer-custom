# Index — map-analyzer-custom Wiki

_Updated 2026-08-11. History: [[log]]._

## Hubs (start here when wiki grows past ~100 pages)
- [[reading-hub]] — reading domain: module, research, decisions, issues

## Overview
- [[overview]] — what the project is, stack, pipeline, status map

## Domain entities
- [[Beatmap]] — a playable osu! difficulty; belongs to a set; not "bitmap"
- [[Analysis-Type]] — the six analysis dimensions; classification vs. detail card vs. stats

## Concepts
- [[Data-Philosophy]] — raw data, no interpretation; how it shapes every metric
- [[experiment-protocol]] — every experiment: prediction | kill-criteria | outcome | verdict; `[explore]` runs carry no criteria

## Codebase modules — backend
- [[aim-control]] — spatial/angle buckets, z-band velocity, ACCV complexity; analysis_type "aimcontrol"
- [[jumps]] — jump classification (≤1 beat gap, >1.5× quarter OR >2.5d), chain bins, bpm_consistency
- [[sliders]] — slider length profile, buzz/static, control-point artistic profile
- [[streams]] — stream classification, burst vs short/med/long/death, spacing + velocity CV
- [[reading-analysis]] — pipeline visuals→density→trajectory→traps→strain→sequence_motor; JSON sections
- [[finger-control]] — snap distribution, patterns, rhythm segmentation (R/T), transitions, timeline

## Codebase modules — frontend cards
- [[aim-control-profile]] — Aim Control card: StatBars + ACCV dashboard + strain curve
- [[finger-control-profile]] — Finger Control card: polished, recharts, snap→color map
- [[jump-profile]] — Jumps card: distance buckets, chain bins, spacing tag
- [[reading-profile]] — Reading card: strain topography, visual clutter, trajectory chaos, traps
- [[slider-profile]] — Sliders card: length/buzz/artistic stat bars + slider tag
- [[stream-profile]] — Streams card: distance/variance/length profiles + spacing tag

## Research (compiled from Temp/ handoffs)
- [[forward-density]] — window 3000→1000ms evolution; prototype results (YOASOBI + AngelMaker); agreed API
- [[angle-distribution]] — 15°/12-bin intra-pattern angle (gh#5 scope); categories dropped from scope; prototype files vanished
- [[sequence-motor]] — MPA/MM/SC origin; per-pattern vs sliding-window; T vs R segmentation
- [[reading-analysis-design]] — framework + architecture draft + chaos-score resolution; what shipped vs PRD
- [[spacing-demand]] — TV2/LTD distilled; superseded by [[sequence-motor]]; keep-1/2-snap; frontend undecided
- [[rhythm-segmentation]] — R vs T discontinuity; three-variable prototype; shipped in reading pipeline; patterns.rs parallel path; `--exp` type-boundary rules (2026-08-10); pivot rule + R_THRESHOLD 0.35 (2026-08-11)

## Decisions
- [[keep-12-snap]] — include 1/2 snap patterns in spacing demand (AngelMaker test, 22→2 patterns)
- [[run-start-engulf-known-limits]] — 8 engulf mismatches accepted as known limitations; proximity-override rejected (8 fixes vs 6 verified breaks); ≥185px dataset retracted
- [[suppression-refinement-abandoned]] — s→c suppression refinement abandoned (what it fixes = what it breaks)
- [[segmentation-unification]] — finger control analysis unified on the updated pattern segmentation (Path A retires; lands with the port)

## Issues
- [[issue-3-intra-pattern-spacing]] — gh#3 OPEN: per-pattern spacing (Burst 2/3/4); prefactor exists in WIP
- [[issue-5-angle-distribution]] — gh#5 OPEN: angle distribution (Burst 3/4); blocked by #3

## Archived
- [[issue-4-forward-density]] — gh#4 CLOSED + archived 2026-08-08; design notes on [[forward-density]]

## Sessions (conversation state, most recent first)
- [[2026-08-11-handoff]] — rerun verdict split: Feral 7/7 + 6/6 fixed, YOASOBI 106 rows break (pivot rule ≈ all; threshold ≈ none); "9 stacks gap-rule" refuted; clue-based kill-criteria adopted; pivot rule decision OPEN
- [[2026-08-10-handoff]] — cross-check rounds 1–2 → resolution: 8 mismatches accepted as known limitations (override rejected: 8 fixes vs 6 verified breaks); re-measure + NC bits done; decisions documented; resume = recap-only
- [[2026-08-09-handoff]] — pattern segmentation: slider-chain misclassification found; asymmetric type rule + SliderChain decided; new-combo decision open
- [[2026-08-08-handoff]] — Phases 0–1 + Phase 2 area 1 done (30 pages); next: area 2 finger-control + decision backlog
- [[2026-08-07-handoff]] — Phase 0 done: wiki live, skills + resume wiring in place (resolved)
- [[2026-08-06-handoff]] — LLM wiki adoption discussion: all decisions confirmed, 5-phase plan approved (resolved)

## Known gaps
- Frontend display for [[spacing-demand]] — undecided (per-pattern vs aggregate, top-N, primary metric, placement, empty state) → `[gap]` in log
- 2 parallel pattern-classification paths ([[finger-control]] vs reading pipeline) — divergence untested; situation documented on [[rhythm-segmentation]] → `[gap]` in log
