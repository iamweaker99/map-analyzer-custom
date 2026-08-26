# map-analyzer-custom OKF Project Brain

> Generated. Do not edit directly.

## Curated routes

- [Analysis systems](routes/analysis.md) — Entry points for current backend analysis, API, and frontend analysis surfaces.
- [Research and evidence](routes/research.md) — Entry points for anchored raw evidence and unresolved research questions.

## Active systems

- system.backend-analysis — Backend analysis system (Rust backend analysis modules that derive beatmap characteristics and expose analysis data to the application.)
- system.backend-http-api — Backend HTTP API (Axum HTTP surface that serves health, beatmap details, and beatmap analysis responses.)
- system.frontend-analysis-ui — Frontend analysis UI (Frontend application surfaces that present beatmap analysis results and category profiles.)

## Current decisions

- decision.forward-density-window — Forward-density window contract (Accepted project decision for the forward-density window specified by the closed primary issue.)

## Active concepts

- None.

## Active entities

- None.

## Active research

- None.

## Active references

- reference.backend-analysis-source — Backend analysis source tree at bootstrap commit (Repository-local snapshot of the backend analysis modules.)
- reference.backend-api-source — Backend API source at bootstrap commit (Repository-local snapshots of the backend HTTP routes and API handlers.)
- reference.forward-density-handoff — Forward-density prototype handoff (Repository-local raw handoff describing a separate forward-looking density prototype.)
- reference.frontend-source — Frontend source tree at bootstrap commit (Repository-local snapshot of the frontend application and analysis presentation code.)
- reference.github-issue-3 — GitHub issue 3 — intra-pattern spacing (Primary GitHub issue specifying the intra-pattern spacing workstream.)
- reference.github-issue-4 — GitHub issue 4 — forward-looking note density (Closed primary GitHub issue specifying the 1000ms forward-density contract.)
- reference.github-issue-5 — GitHub issue 5 — intra-pattern angle distribution (Primary GitHub issue specifying the short-pattern angle distribution workstream.)
- reference.github-issue-6 — GitHub issue 6 — duration-based stream categorization (Primary GitHub issue specifying duration-based stream categories and final-buffer handling.)
- reference.project-readme — Project README at bootstrap commit (Repository-local snapshot of the project README used for high-level project identity and scope.)
- reference.reading-analysis-prd — Reading analysis PRD raw document (Repository-local raw PRD describing pattern-aware reading metrics and a proposed forward-density window.)
- reference.sequence-motor-prototype — Sequence-motor prototype source (Repository-local prototype source for spacing-transition and sequence-motor descriptors.)
- reference.sequence-motor-sample — Sequence-motor prototype sample (Repository-local JSON sample containing prototype sections, tiers, geometry notes, and descriptor values.)
- reference.spacing-transition-handoff — Spacing transition demand handoff (Repository-local raw handoff describing prototype findings and a proposed spacing-transition metric.)

## Historical / deprecated

- None.

## Draft knowledge

- research.forward-density-window-conflict — Forward-density window reconciliation (Bootstrap research record for reconciling two raw project proposals with different forward-density windows.)
- research.sequence-motor-prototype-limitations — Sequence-motor prototype limitations (Bootstrap research record for what the sequence-motor prototype measures and leaves unmeasured.)
- research.spacing-transition-demand — Spacing transition demand prototype (Bootstrap research record for a prototype comparing local spacing-transition descriptors.)

## Needs review

- See generated/review.md.
