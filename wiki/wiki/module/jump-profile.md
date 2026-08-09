---
type: module
status: stable
updated: 2026-08-08
---
# Module: jump-profile (frontend)

Purpose: renders the Jump analysis card in the 3×2 results grid (Row 2, "Jumps" card) of the beatmap analysis page.

## Files
- `frontend/src/components/analysis_engine/JumpProfile.tsx` — the profile component (pure render)
- `frontend/src/components/analysis_engine/types.ts` — `JumpAnalysis` interface (lines 21–27) and `BeatmapAnalysisResult` union (lines 7–19)
- `frontend/src/components/analysis_engine/utils.ts` — `getSpacingTag` (lines 1–7)
- `frontend/src/components/analysis_engine/StatBar.tsx` — shared bar primitive used for all stats
- `frontend/src/components/analysis.tsx` — parent page, wires the card (lines 280–298)

## Data flow
1. `analysis.tsx:76` fetches everything at once: `getBeatmapAnalysis(+beatmapId, "all")` → `BeatmapAnalysisResult[]`
2. `analysis.tsx:93–94` picks the entry with `analysis_type === "jump"` and casts `.analysis as JumpAnalysis`
3. `JumpProfile` receives the single prop `{ analysis: JumpAnalysis }` — no fetch, no state
4. Rendered inside a Card with pink top border (`border-t-pink-500/50`), title "Jumps", inside a `ScrollArea h-72` (analysis.tsx:283–296)

## Data shape (JumpAnalysis, types.ts:21–27)
All snake_case, matching the backend JSON 1:1:
- `circle_diameter` (px), `avg_spacing` (px), `bpm_consistency` (0–1), `overall_confidence` (0–1), `jump_density` (0–1)
- Distance buckets: `narrow_count`/`narrow_dens`, `moderate_count`/`moderate_dens`, `wide_count`/`wide_dens`, `extreme_count`/`extreme_dens`
- Chains: `short_jumps` (3–5 notes), `medium_jumps` (6–11), `long_jumps` (12+), `max_jump_length`

## What the card renders
1. **Header row** — `Spacing: {getSpacingTag(avg_spacing, circle_diameter)} (px)`; tag thresholds: <2.0×D Narrow, <3.5×D Moderate, <5.0×D Wide, else "Cross-Screen (Extreme)" (utils.ts:1–7)
2. **Distance Profile (Excluding Streams)** — 4 StatBars (JumpProfile.tsx:20–48):
   - Narrow (<2.0×D) green | Moderate (2–3.5×D) blue | Wide (3.5–5×D) orange | Extreme (5.0×+ D) red
   - Each shows count as value plus `*_dens` × 100 as a direct percentage
3. **Jump Chain Profile** — 3 StatBars (JumpProfile.tsx:50–72): Short (3–5) green, Medium (6–11) blue, Long (12+) red; percentage computed inside StatBar as `value / total` where `total = short + medium + long` (JumpProfile.tsx:9–12)
4. **Footer** — `Max jump chain: {max_jump_length} notes` and `BPM Consistency: {bpm_consistency × 100}%`

## StatBar semantics (StatBar.tsx:13–54)
- `percentage` prop (direct 0–100) takes priority; otherwise `value/total × 100` (0 when total is 0)
- With `value` passed it renders `value (pct%)` plus a 1px-tall colored bar

## Quirks
- `<li>` elements sit directly under a plain `<div>` (JumpProfile.tsx:16, 74, 78) — invalid HTML nesting, renders fine
- `max_jump_length` has **no fallback** (line 76) — renders "undefined notes" if the backend omits it; all other fields use `|| 0`
- `circle_diameter` hardcodes a 73px fallback (JumpProfile.tsx:6) — spacing tag math silently runs on the fallback if the field is missing
- Backend sets both `jump_density` and `overall_confidence` to `j_cnt / total_obj` (jumps.rs:57, 66) — the two fields are always identical
- Backend `*_dens` are normalized by `total_obj` (all objects on the map), not by jump count (jumps.rs:60–61)
- Chain semantics: backend counts `note_count = consecutive_jumps + 1` (jumps.rs:18–27), so "Short (3–5)" means 2–4 consecutive jump movements
- Backend distance thresholds (2.0/3.5/5.0×D, jumps.rs:37–40) duplicate `getSpacingTag` thresholds — same values in two places
- JSX labels escape `<` as `&lt;` (JumpProfile.tsx:25) so "Narrow (<2.0x D)" renders correctly
- Same unified card style as the other five profiles (StatBar/grid); no recharts charts — a few stray list-item (`<li>`) markup lines remain (see above in this section)

## Integration points
- Consumes the `"jump"` variant of the `analysis` union in `BeatmapAnalysisResult` (types.ts:9, 12–18); adding a new analysis type requires extending both unions
- Backend producer: `backend/src/analysis/jumps.rs` `analyze()`; a movement is a jump when `time_gap <= 60000/bpm` AND (`time_gap > 1.5 × 16th-note` OR `distance > 2.5×D`) (jumps.rs:6–7, 30)
- Jump results appear **twice** on the page: in this card, and in the Classification card (`classificationTypes = [jump, stream, slider]`, analysis.tsx:112) sorted by `overall_confidence` with a presence bar (analysis.tsx:118–119, 430–446)

## Related
- [[jumps]] — backend module producing the JSON this card renders
- [[Analysis-Type]] — the jump analysis type and its union
- [[Data-Philosophy]]
- [[overview]]

_Sources: frontend/src/components/analysis_engine/{JumpProfile,types,utils,StatBar}.tsx, frontend/src/components/analysis.tsx, backend/src/analysis/jumps.rs (read 2026-08-07)_
