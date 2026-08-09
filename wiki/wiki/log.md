# Log — append-only history

## [2026-08-06] demo bootstrap | ingest
- **Ingested:** raw/CONTEXT.md · raw/prd-reading-analysis.md · raw/reading-handoff-01-forward-density.md · raw/spacing-demand-handoff_5.md
- **Created:** index, overview, entities ([[Beatmap]], [[Analysis-Type]]), concept ([[Data-Philosophy]]), modules ([[reading-analysis]], [[finger-control]]), research ([[forward-density]], [[spacing-demand]]), decision ([[keep-12-snap]]), issue [[issue-4-forward-density]]
- **Contradiction resolved:** forward-density window 3000ms (handoff, pre-prototype) **superseded by** 1000ms (PRD, post-prototype finding)
- `[gap]` TV2/LTD definitions not yet distilled from spacing handoff
- `[gap]` spacing-demand frontend decisions (per-pattern vs aggregate, top-N, primary metric, placement, empty state)
- `[gap]` Jump/Stream/Slider analysis modules not yet ingested
- **Lint:** no broken links; index current; 2 unresolved gaps logged

## [2026-08-06] finger-control routing drill | ingest
- **Routing miss found:** "finger control section" → wiki had backend module page but no frontend page. Route dead-ended at [[finger-control]].
- **Ingested:** frontend `FingerControlProfile.tsx` + `types.ts` → new [[finger-control-profile]] (polished, recharts, snap→color map, beatmapMd5 reset key, types duplication known issue)
- **Index updated; log appended.**
- **Routing note:** ambiguous query "section" now resolves — backend page + frontend page both reachable from index.

## [2026-08-06] session save demo | ingest
- Created session page type + [[2026-08-06-handoff]] — conversation state, open threads, next step
- Index updated (Sessions section); log appended
- **Design note:** session pages are the "save for tomorrow" artifact — see SCHEMA ground rules (session type + save ritual + review gate)

## [2026-08-06] scaling drill | structure
- Created hub page type + [[reading-hub]] — two-stage routing (hub lines → pages) for when flat index passes ~100 pages
- Index updated (Hubs section); log appended
- **Design notes from community research:** page cap ~400 lines; in-place updates (no v2 pages); closed issues → archive/; search layer only after ~300 pages

## [2026-08-06] adoption decision | status
- **User decided: implement LLM wiki permanently** (demo approved; scaling plan approved)
- SCHEMA evolved to v0.2: added hub/session page types, archive rule, page caps, in-place updates, forget≠delete, save ritual, scaling table
- Handoff [[2026-08-06-handoff]] updated: decision points listed for user review
- Status: discussion phase — awaiting user answers before execution

## [2026-08-07] discussion closed | status
- **All decisions confirmed** (A1–D2). Execution plan (5 phases) approved by user.
- Resume protocol agreed: `resume:<date>` token pasted by user at session start; CLAUDE.md wiring (~3 lines); unrelated sessions load no wiki context.
- obsidian-vault skill → delete in Phase 3. Temp/wiki-demo → delete after Phase 1 confirmed.
- Next: user pastes `resume:2026-08-07-wiki-bootstrap` in a new session → Phase 0 (scaffold).

## [2026-08-07] Phase 0 scaffold | bootstrap
- **Wiki is live at repo root:** `wiki/SCHEMA.md` (v0.2), `wiki/raw/` (4 curated sources), `wiki/wiki/` (19 pages ported from demo).
- **Skills created:** `.claude/skills/wiki/` — `/wiki-ingest`, `/wiki-query`, `/wiki-lint`, `/resume` (gitignore exception added: `!.claude/skills/wiki/`).
- **CLAUDE.md wired:** sections 5–6 — wiki-first rule + `resume:YYYY-MM-DD` → `wiki/wiki/session/<date>-handoff.md` protocol.
- **Lint lessons (schema co-evolution):** `overview.md` documented as root page; hub folder→type mapping fixed (hubs/ → `type: hub`); lint now classifies `[gap]`-logged links as pending, not broken.
- `[gap]` `[[issue-3-intra-pattern-spacing]]` + `[[issue-5-angle-distribution]]` — hub forward links; pages land in Phase 1 (issue pointer model, B2).
- **Lint:** green (2 pending, logged). Resume roundtrip: token `resume:2026-08-07-wiki-bootstrap` → `wiki/wiki/session/2026-08-07-handoff.md`.
- Demo `Temp/wiki-demo/` kept until Phase 1 confirmed (D2).

## [2026-08-08] Phase 1 core ingest | ingest
- **11 subagents, 14 pages:** created `aim-control`, `jumps`, `sliders`, `streams`, `aim-control-profile`, `jump-profile`, `reading-profile`, `slider-profile`, `stream-profile`, `issue-3-intra-pattern-spacing`, `issue-5-angle-distribution`; updated in place `finger-control`, `reading-analysis`, `finger-control-profile`. Index updated; pending links `[[issue-3-intra-pattern-spacing]]` + `[[issue-5-angle-distribution]]` now resolve.
- **Cross-module contradictions logged on pages:** `overall_confidence` defined inconsistently across jumps/sliders/streams (streams excludes bursts from the denominator); duplicate keys `jump_density`/`slider_ratio` == `overall_confidence`; "aimcontrol" vs `aim_control` naming mismatch; two parallel pattern-classification implementations (patterns.rs vs rhythm_segmentation.rs, untested divergence); `complexity.rs`/`morphology.rs` orphaned (unregistered in mod.rs).
- `[gap]` ACCV + reading thresholds hardcoded, no calibration evidence in repo; O(n²) density; alignment band gaps; `momentum_retention` dead field; slider tail forced to (256,192).
- **gh#4 state finding: CLOSED on GitHub (checked 2026-08-08)** — wiki page kept as-is per user decision; index + hub lines now reflect actual state; user decides archive per SCHEMA rule 8.
- **Lint:** green after index merge. Frontmatter dates normalized to 2026-08-08.

## [2026-08-08] gh#4 archived | status
- **User decision: archive.** [[issue-4-forward-density]] moved to `wiki/wiki/archive/` per SCHEMA rule 8 (gh#4 CLOSED on GitHub 2026-08-08). Page frontmatter → `status: closed`, archive note added.
- Index Archived section + reading-hub Archived list updated; active issue surface = gh#3 + gh#5 only.

## [2026-08-08] Phase 2 area 1: reading distillation | ingest
- **4 subagents, 4 pages:** created `angle-distribution`, `sequence-motor`, `reading-analysis-design`; updated `forward-density` in place (prototype results: YOASOBI + AngelMaker at 1000/1500/3000ms; 1000ms confirmed; merge-vs-separate resolved).
- **Contradictions resolved/recorded:** framework Layer 2/3 superseded by PRD scope (transitions = outliers; BPM layer out); MPA = chaos-disc second-derivative idea in mean form; PRD "e.g." buckets vs prototype actual buckets noted.
- **Key gaps:** proto_angle.rs + prototype_reading_angle.rs (cited 2026-07-16) absent from tree — recover from git or re-derive before gh#5; handoff-03 category layer (linear/curved/orthogonal/anti-symmetry) dropped from gh#5 scope with no decision doc; T/R segmentation design has no dedicated handoff; sequence_motor ships as PROTOTYPE with no matching handoff.
- `[gap]` deferred files honored: `(deferred) angle-direction-research-notes`, `(deferred_combine…) velocity-spatial-discontinuity-framework` — existence logged, content not ingested.
- **Lint:** green after index merge.

## [2026-08-08] session save | status
- Created [[2026-08-08-handoff]] — Phases 0–1 + area 1 record, decision backlog (12 items), next step = Phase 2 area 2.
- [[2026-08-07-handoff]] marked resolved → successor pointer. Index sessions updated.
- Demo `Temp/wiki-demo/` deleted (D2, user-confirmed). Nothing committed yet — working tree ready for a single commit.

## [2026-08-08] Phase 2 area 2: finger-control distillation | ingest
- **2 subagents, 2 pages:** created `rhythm-segmentation`; refreshed `spacing-demand` in place — TV2/LTD definitions distilled from raw handoff, closing the `[gap]` logged 2026-08-06 (index.md gap line removed).
- **spacing-demand refresh:** full TV2/LTD/LTD_rms formulas + null rules + D-normalisation (the only parameters — no ACCV-style constants, those belong to [[aim-control]]); real-map numbers (AngelMaker 22 patterns / TV2 ~1.27 / LTD ~1.62; Yoru ni Kakeru 14 / ~5.59 / ~31.25); lineage recorded — superseded by [[sequence-motor]] (MPA = TV2/k; LTD scale-blindness → MPA+MM split). 8 contradictions logged with file:line, incl. committed prototype binary hardcoding `exclude_snaps=true` against [[keep-12-snap]].
- **rhythm-segmentation (new):** three-variable prototype (Temporal/Velocity/Rhythm; Rhythm won — normalizes BPM changes out); R vs T definitions + worked examples; 1565-pair prototype results (T≡R on single-BPM, 341 boundaries at threshold 0.5, near-empty band 0.05–0.5); prototype-vs-shipped status table; **patterns.rs vs rhythm_segmentation.rs parallel-path situation documented with file:line evidence** (index.md gap now points at this page).
- `[gap]`s carried: spacing-demand frontend display undecided (blocks shipping); LTD denominator k vs (k+1); R_THRESHOLD tuning vs in-game editor; ±10% tolerance off-grid comparison never documented; path-A/B divergence untested; velocity+direction intra-pattern segmentation session; 3-signal dataviz.
- **Lint:** green after index merge (see lint entry below).

## [2026-08-08] cleanup part 2: research status taxonomy | status
- **SCHEMA v0.2 → v0.3:** research `status` vocabulary documented — stages `idea → designed → prototyped → implemented`, optional dispositions `(deferred)` / `(abandoned)`; superseded findings keep stage status + body pointer (no `superseded` value for research).
- **Six research pages normalized:** `forward-density` design-agreed-not-implemented → prototyped; `spacing-demand` backend-prototyped → prototyped; `sequence-motor` shipped-as-prototype → prototyped; `rhythm-segmentation` shipped → implemented; `angle-distribution` + `reading-analysis-design` → designed (status added).
- **Lint surfaces updated to validate the vocabulary:** `wiki-lint` SKILL.md check 4 + `.claude/hooks/wiki-lint.ps1` check (d). Tree verified clean against the new rule.

## [2026-08-08] cleanup part 1: drift fixes | status
- **Group A drift fixed (user-approved):** `overview.md:29`, `jump-profile.md:50`, `finger-control-profile.md:33` rewritten to the unified six-card reality (Jump/Stream/Slider share the card style; stray `<li>` markup lines + no recharts remain); `reading-hub.md:12` `#open` → `#not-implemented` (gh#4 carrier closed).
- **Status taxonomy (Group B #5):** user weighing idea/designed/prototyped/implemented (+ deferred/abandoned dispositions) — SCHEMA documentation pending final decision.
- **Cleanup-session approach (Group B #6):** mechanical sweep + `/grill-with-docs` per contested item — user-approved; session itself deferred.

## [2026-08-08] Phase 3: hygiene | structure
- **`obsidian-vault` skill deleted** (D2, 2026-08-07 decision): symlink `.claude/skills/obsidian-vault`, source dir `.agents/skills/obsidian-vault/`, and its `skills-lock.json` entry removed; lock file JSON-validated; wiki skills untouched.
- **PostToolUse wiki-lint hook installed** (user-approved design): `.claude/settings.json` (matcher `Write|Edit` → `powershell -File .claude/hooks/wiki-lint.ps1`, local + gitignored) + `.claude/hooks/wiki-lint.ps1` (~90 lines, PS 5.1) checking broken wikilinks (log.md exempt, `[gap]`-named basenames pending), index drift both directions, frontmatter type-vs-folder per SCHEMA; prints `WIKI-LINT:` violations, never blocks, silent off-wiki. Tested end-to-end (broken link / drift / frontmatter all detected; clean + non-wiki runs silent). Goes live at next session start (watcher caveat).
- **Auto-memory migrated** (4 files): `frontend-ui-overhaul-state` rewritten to actual state (six cards unified, radar chart removed, stray `<li>` + no recharts in Jump/Stream/Slider; header/footer/dev-banner notes kept; wiki card pages linked); `forward-density-plan` slimmed (gh#4 CLOSED + archived, never implemented; design/API on [[forward-density]]); `spacing-demand-keep-12-snap` slimmed to pointer + apply rule; `spacing-demand-frontend-pending` updated (superseded-by-sequence-motor context). `MEMORY.md` refreshed.
- **Wiki drift flagged for cleanup session (user-deferred):** `overview.md:29`, `jump-profile.md:50`, `finger-control-profile.md:33` — stale "old-style `<li>` list" characterizations vs the unified six-card UI; `reading-hub.md:12` `#open` tag on [[forward-density]] (carrier issue closed).

## [2026-08-08] overview stale claim fixed | status
- **User-approved fix for lint judgment call:** `overview.md` status map "Forward density | Issue #4 OPEN" → "Issue #4 CLOSED + archived 2026-08-08; design notes → [[forward-density]]".
- Remaining related judgment call: `hubs/reading-hub.md:12` tags [[forward-density]] `#open` — ambiguous since carrier issue closed; user to decide.

## [2026-08-08] lint | health
- **Scope:** all 31 pages under `wiki/wiki/` — index, log, overview, 2 entities, 1 concept, 12 modules, 5 research, 1 decision, 2 issues, 1 archived issue, 1 hub, 3 sessions.
- **Checks run:** broken wikilinks (none — all 30 unique link targets resolve, case-exact); orphan pages (none — every page reachable from index); index drift (clean both directions); frontmatter validity (types match folders incl. `hubs/` → `type: hub`; `updated`/`status`/`sources`/`github`+`status`/`area` all present where required); stale claims; page caps (largest = 92 lines, no split candidates).
- **Flagged (judgment call, not auto-fixed):** `overview.md` status map still reads "Forward density | Issue #4 OPEN" while gh#4 is CLOSED + archived 2026-08-08 (index/log/archive all agree) — stale-vs-fresh conflict left for human.
- **Result:** lint green except 1 stale claim; no mechanical fixes applied.
