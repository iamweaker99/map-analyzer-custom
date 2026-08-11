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

## [2026-08-09] slider-chain classification bug | ingest
- **Found during editor verification (YOASOBI Collab Extra):** multi-note patterns classified by count only (rhythm_segmentation.rs:61-69; `is_slider()` only for singletons) → consecutive sliders labeled "Stream". Verified on 3 map sections (3c+7s / 2s+2c+4s / 9s+1c — all mixed).
- **Decided:** Option 3 (split at circle↔slider type change) + Option 4 (new `SliderChain` variant) — implementation pending consequence review (fragmentation, guard policy, alternation-annihilation risk).
- **Deferred:** waypoint model (slider = head+tail waypoints) + SV-based slider-chain segmentation — attached to [[rhythm-segmentation]] open questions (2026-08-09 section).
- **Verified:** rosu_pp-4.0.1 exposes no per-object velocity; effective speed derivable via `expected_dist`/duration (sliders.rs:17 precedent).

## [2026-08-09] pattern-segmentation discussion | session
- **Created [[2026-08-09-handoff]]** — slider-chain misclassification found via editor verification; asymmetric type-boundary rule + `SliderChain` variant + 25px engulf decided; new-combo-as-signal decision open (recommendation: reference column only); waypoint model + SV segmentation deferred on [[rhythm-segmentation]]
- **Resume token:** `resume:2026-08-09` — next step: confirm new-combo decision, then implement + regen YOASOBI table/Excel

## [2026-08-10] pattern segmentation editor cross-check | implement
- **Implemented in prototype `--exp` (backend/src/bin/prototype_sequence_motor.rs):** R/T suppression at slider→circle transition windows (`skip_discontinuity`) — R=1.0 at the first circle after a slider chain landed a boundary one note *inside* the circle run (YOASOBI 974/792); type rule already bounds the run at the transition. Asymmetric: circle→slider windows keep R (964 isolation preserved).
- **Engulf proximity = 2× circle diameter** (user decision; CS-scaled): 25px was inside one circle radius at CS4; trailing heads measured 38.9–116px in vs ≥185px out → ~146px at CS4.
- **Run-start sliders stay out** (user decision): stream-ending slider that starts a slider run ≥2 keeps the boundary — 02:53's verified 12 (804 out) is the same shape. 02:37:270 stays 18 vs user's 19, flagged for other-map engulf test.
- **02:08:539 accepted as known R limitation** (pure 1/4→1/2 rhythm change at a stream end; pre-existing both versions).
- **Verified:** 11/11 unit tests; YOASOBI 549 + Signal 413 patterns, 0 structural violations; A/B/D fixed; 7/8 stream counts match editor.
- **Wiki:** [[rhythm-segmentation]] experimental section + [[2026-08-10-handoff]] created; log appended.

## [2026-08-10] pattern segmentation editor cross-check | session
- **Created [[2026-08-10-handoff]]** — prototype `--exp` implemented + verified against editor feedback; R-suppression/engulf/run-start decisions recorded; new-combo reference-only confirmed; 7/8 stream counts match
- **Resume token:** `resume:2026-08-10` — next: user walks `exp.new.xlsx` against editor; end-slider engulf cross-check on another map (scheduled reminder 16:53 local); production port after approval

## [2026-08-10] round 2: 8-mismatch analysis + distance-data correction | session
- **User's 8 new c→s engulf mismatches analyzed (discussion only — no rule changes):** 6 = run-start sliders (type rule sole blocker); case 5 = R boundary placement (unreachable by signals); case 6 = s→c suppression over-fire (but un-suppressing breaks case 8 — no consistent threshold).
- **Distance-data correction (verified from the .osu):** all 8 heads are 0px stacks except case 4 (116.5px); round-1 per-case distances (79.2–194px) were outgoing-distance transcription errors (case 8's 194px = stack→2nd-slider-head); 02:08's "437px" = actually 36px. **Engulf-threshold dataset ("38.9–116 in / ≥185 out") needs re-measurement** before trusting 2× diameter.
- **Blast radius:** run-start engulf = +7 fixes (incl. 02:37→19) vs −4 verified breaks (02:53→13, 02:03→37, 01:26→17, 03:55→13) → not implementing. NC-bit discriminator test pending (8-case sliders all type 2/no-NC; verified-out five recorded NC-carrying except 739).
- **Auto-compact disabled globally** (`autoCompactEnabled: false` in user settings.json; user runs /compact manually).
- **New user rule:** `resume:` token → recap ONLY, zero execution until user reviews and explicitly authorizes (CLAUDE.md §6 updated).
- **Wiki:** [[2026-08-10-handoff]] rewritten (round 2), [[rhythm-segmentation]] experimental section updated, index sessions line updated; log appended.
- **Resume token:** `resume:2026-08-10` — next: recap-only; on approval: re-measure trailing-slider heads + check NC bits, then settle the run-start design.

## [2026-08-10] engulf run-start resolved | decision
- **Re-measurement + NC check done (2 sub-agents, validated):** YOASOBI map lives outside the repo (`D:\osu files\…`; repo maps are Hanabira); handoff IDs are 0-based; 8 case heads = 0px stacks (case 4: 116.5px); 5 verified-out heads 38.9–97.8px (all < 146px engulf threshold); genuine ≥185px-head set = 30 sliders — **≥185px "out" dataset retracted** (transcription artifact; 6 of 50 flagged are true stacks); NC bits confirmed (8-in type 2, 5-out type 6 incl. 739).
- **Override evaluated (simulator validated byte-identical to the exp artifact):** proximity-overrides-run-start = fixes 7/8 + matches 02:37→19 vs **6 verified breaks** — 02:53→13, 02:03→37, 01:26→17, 03:55→13 **+ 2 newly found at 03:38** (964, 982); map-wide 549 → 471. No signal discriminates (distance ⊇, NC fails both ways on 739/964, rhythm uniform, following-slider common to both).
- **Decisions (user, evening):** give up optimizing the 8 mismatches — accepted as known limitations; proximity override **rejected**; suppression refinement **abandoned**; NC reference-only confirmed; 2× diameter threshold stands. → [[run-start-engulf-known-limits]] · [[suppression-refinement-abandoned]]
- **Session directives:** sub-agent orchestration for the rest of the session (evaluation at end); resume = recap-only (CLAUDE.md §6).
- **Wiki:** 2 decision pages + [[rhythm-segmentation]] amendments + handoff rewrite + index; lint green.

## [2026-08-10] decisions: unification + 3-map cross-check | decision
- **Path A/B unification decided (user):** finger control analysis uses the current updated pattern segmentation — Path A (`finger_control::patterns`) retires; lands with the production port → [[segmentation-unification]]
- **Kept open (user):** R_THRESHOLD tuning, ±10% snap tolerance, frontend impact of the port, sequence-motor promote/formalize.
- **Excel walk trimmed:** NC-only boundary check dropped (reference-only); walk = pickup joins 511/360/1054.
- **Production port deferred by user** until the prototype is tested on other maps ("will come back after").
- **3-map end-slider engulf cross-check launched (3 parallel sub-agents):** Signal [Disturbance], meganeko - Feral [Veracious], HIMEHINA - Heart Pie Dancehall [3.1415926535] — Excel outputs for the user's editor walk.
- **Wiki:** [[segmentation-unification]] + handoff update; lint green.

## [2026-08-10] 3-map engulf cross-check complete | validate
- **3 maps run through the exp prototype (3 parallel sub-agents):** Signal [Disturbance] 413 patterns — 26 stacks, 18 typed run-start ALL type-rule (run≥2), 8 engulfed (all run=1); Feral [Veracious] 379 patterns — 9 stacks, ALL gap-rule (90ms threshold @ 375 BPM; heads 187/94ms after circle — real pauses); Heart Pie [3.1415926535] 608 patterns — 1 stack typed singleton (638ms pause, correct), no type-rule stacks. Engulf works for rhythm-adjacent lone sliders everywhere (Signal 8/8, Feral 52/52, Heart Pie 53).
- **Cross-map verdict:** YOASOBI-8 was not a fluke — the run≥2 clause reproduces on Signal at scale (18); the same symptom arises from the gap rule with defensible rhythm reasons on Feral/Heart Pie. Excels for the user's editor walk: `Prototyping/{signal_disturbance,feral_veracious,heart_pie_dancehall}_patterns_exp.xlsx`.
- **Wiki:** [[rhythm-segmentation]] cross-check bullet + handoff updated; lint green.

## [2026-08-10] session end | session
- **User is MID-EXCEL-WALK** (3 new Excels + YOASOBI pickup joins 511/360/1054); resume token: `resume:2026-08-10`.
- **Orchestration verdict (user): "doing very well"** — standing directive: sub-agent orchestration for task execution in ALL sessions onward, except wiki updates (CLAUDE.md §7 + memory).
- **Wiki:** handoff finalized; lint green.

## [2026-08-11] Feral 1/3–1/6 diagnosis → pivot rule adopted | decision
- **7 Feral error groups diagnosed with raw-map evidence** (`Temp/extract_feral_neighborhood.py` + `Temp/extract_feral_nc.py` + `Prototyping/feral_veracious_patterns_exp.json`): Class A = boundary placement (R fires but boundary lands after the pivot; NC bits 7/7 at pivots validate mapper intent = pivot starts the new run); Class B = R_THRESHOLD 0.5 misses 1/4→1/3 (0.415) — streams are 1/4+1/3, not 1/6+1/4; Class C ("Unstable" labels) dissolves with correct splits; Class D (engulf, error 6) accepted.
- **BPM correction:** Feral beat_len = 375 ms = **160 BPM** (NOT 375 BPM); gap threshold = 197.5 ms (not 90 ms); "9 stacks ALL gap-rule" needs re-verification (187 ms < 197.5 ms).
- **Buffer question answered:** "memory processing buffer" never implemented (max window 3 objects); NOT the missing piece; later robustness upgrade only.
- **Decisions (user, locked):** adopt pivot rule (speed-up → boundary at k; slow-down → boundary at k+1); R_THRESHOLD → 0.35; fix the prototype and rerun on the 4 maps (YOASOBI, Signal, Feral, Heart Pie) — judge "breaks more things than it fixes".
- **experiment-protocol page created** (research-loop #1, [[experiment-protocol]]); keep/abandon decision deferred to end of NEXT session; items #3/#4 left open.
- **Wiki:** [[2026-08-11-handoff]] created, [[rhythm-segmentation]] updated (Feral diagnosis + BPM fix), index sessions updated.
- **Resume token:** `resume:2026-08-11` — next: recap-only; on approval: prototype fix (pivot rule + 0.35) → 4-map rerun → regression verdict.

## [2026-08-11] pivot rule + R_THRESHOLD 0.35 rerun on 4 maps | validate
- **Prototype fixed + tested** (`prototype_sequence_motor.rs`): `pivot_boundary_idx` helper (speed-up → boundary at k, slow-down → k+1) in R and T loops; threshold 0.5→0.35; 3 new unit tests; 14/14 pass. Rerun via 4 parallel sub-agents → `Prototyping/rerun_{map}.json` (all 4).
- **Feral (target): PASS on walk** — 7/7 Class A sites split at the pivot (nc=True, mid_combo_breaks=0, incl. the 7th site 56985); 6/6 Class B streams split into clean 1/4 + 1/3 (R=0.415); Unstable labels dissolved (52→46); ZERO fragmentation; 24 additional moves all merges (18/24 NC-corroborated, 6 slider-region non-NC flagged: 17.891, 72.735, 74.235, 75.735, 77.235, 108.922); 379→365.
- **YOASOBI (control): FAIL** — walk (all 549 rows user-verified) == pre-fix output; fix flips 106 rows: 39 merges incl. **6 pickup-joins into streams spanning 2–5 combo sections** (pivots 642/655/680/693/706/720, all NC=1), 12 boundary shifts. **Attribution: pivot rule ≈ 100% of the damage; the threshold drop contributed ~0** (zero new fires with damage — a threshold drop can only split, never merge; every changed row traces to a k+1→k move; Signal agent verified "threshold alone created/removed no sites").
- **Signal:** 02:08.539 byte-identical (PASS — slider→circle skip rule protects it); 21 pivot moves all speed-up; 5 fragments (3 new singletons), 24 merges; 413→397.
- **Heart Pie:** 638ms stack + walk leftover (idx 544) byte-identical (PASS); 19 new singletons vs 76 merges; NC corroborates 4/4 sampled pivots; 608→566.
- **Gap-rule re-verify: claim REFUTED** — 0/9 stacks have gaps > 197.5ms (187.0/94.0ms); all 9 boundaries were R-discontinuity (1/4↔1/2, R=0.99) in both runs; stacks 6–9 now held by type rule alone (rows unchanged). "90ms @375 BPM" was a BPM misreading; code always used 197.5ms.
- **NC-gate hypothesis tested: dead** — all 6 YOASOBI join pivots carry NC=1 (same signal as Feral, opposite walk outcome); a combo-boundary gate (reject moves creating patterns with mid_combo_breaks>0) kills 6/6 joins but is partial (≈4 dissolves survive). **No clean mechanical separator found — the rule encodes Feral's mapper intent; YOASOBI's intent is opposite at the same signals.**
- **Wiki:** [[experiment-protocol]] upgraded to clue-based kill-criteria (pre-flight / canary / final); handoff rewritten; lint green.

## [2026-08-11] session end | session
- **Open decision (user, next session):** pivot rule — adopt as-is / threshold-0.35-only (the safe half: fixes Feral Class B, zero YOASOBI damage) / reject / refine under the new clue-based criteria. Kill-criteria of exp #1 fired on YOASOBI (106 flips); user redesigning criteria, experiment #1 stays OPEN.
- **Orchestration:** 4-map fan-out + 2 follow-ups all returned structurally complete reports; main context stayed lean (~430k tokens in agents). Delegation tiers (reading sources + mechanical wiki maintenance → sub-agents; drafting/decisions → main) adopted for this session — evaluation continues.
- **Infra:** wiki skills fixed (were nested `.claude/skills/wiki/` — never registered; flattened, `.gitignore` exceptions updated); CLAUDE.md §7 classification test added (user-approved); memory updated.
- **Wiki:** handoff finalized; lint green. Resume token: `resume:2026-08-11`.

## [2026-08-11] wiki lint (resume run) | validate
- **Mechanical checks green:** 39 files scanned; 0 broken wikilinks (38/38 resolve), 0 orphans, index drift clean both directions, 0 frontmatter violations, 0 pages over cap.
- **Judgment items flagged (7, none fixed — recap-only session):** (1) 08-09 handoff "25px engulf" vs 2× circle diameter — historical snapshot, current pages correct; (2) 08-10 handoff "375 BPM / 90 ms" vs 160 BPM / 197.5 ms correction (already superseded in [[rhythm-segmentation]]); (3) "9 stacks ALL gap-rule" vs refutation at log.md:176; (4) [[sequence-motor]] :56/:63 still documents R > 0.5 ("tune if needed") — stale vs 0.35 adoption, left open per user; (5) reading-hub.md #open tag — fixed 08-08 (`#not-implemented`), 08-08-handoff open-thread records stale; (6) session status chain — 08-08/09/10 marked `resolved` 08-11 (08-06/07 convention), 08-11 stays open (live); (7) 08-11 handoff self-referential predecessor line (rewrite-in-place artifact, harmless).

## [2026-08-11] session end | session
- **User decisions:** threshold-0.35-only accepted (in-game cross-check "no problem"); pivot rule **abandoned** ("I give up optimizing on pivot rule") → documented at `abandoned/abandoned_pivot-rule.md`, pulled from scope; production port + unification approved; research-loop #2–#5 stay open; [[experiment-protocol]] keep/abandon — user still evaluating (continues on the motor adjustment plan discussion, [[sequence-motor]], next session).
- **Production port shipped + committed:** `rhythm_segmentation.rs` = exp rules (asymmetric type boundary, s→c suppression, `SliderChain`, 2× diameter engulf) + R/T_THRESHOLD 0.35; `finger_control::patterns` (Path A) retired per [[segmentation-unification]]; finger_control + reading rewired; prototype pivot code removed (11/11); full `cargo test` green; **canary PASS — production row-identical to the user-verified baselines (YOASOBI 549/549, Feral 385/385, all 7 fields)**. `canary_segmentation.rs` kept as the port's regression harness. No Signal/Heart Pie runs (user: not needed).
- **Wiki:** handoff 08-11 finalized (session wrapped; next: motor adjustment plan); sessions 08-08/09/10 resolved; [[rhythm-segmentation]] updated (0.35 verified, pivot abandoned, Class A = known limitation); lint green (resume run recorded above).
- **Resume token:** next session — `resume:<next-date>`; focus: motor adjustment plan ([[sequence-motor]]) + kill-criteria evaluation.
