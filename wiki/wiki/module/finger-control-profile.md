---
type: module
status: stable
updated: 2026-08-08
---
# Module: finger-control-profile (frontend)

Purpose: renders the Finger Control card in Row 3 (Finger Control | Aim Control | Reading) of the results grid. Pure renderer — the parent fetches; the profile receives one prop.

## Files
- `frontend/src/components/analysis_engine/FingerControlProfile.tsx` — the profile component
- `frontend/src/components/analysis_engine/types.ts` — single source of truth for `FingerControlAnalysis` and all sibling analysis types
- `frontend/src/components/analysis.tsx` — fetch + dispatch; imports all analysis types from `./analysis_engine/types` (analysis.tsx:16-25)

## Data flow
1. `analysis.tsx` fetches all types at once: `getBeatmapAnalysis(+beatmapId, "all")` (analysis.tsx:76)
2. Picks the finger control result with `analysis_type === "fingercontrol"` via `.find` (analysis.tsx:102-103)
3. Casts `analysis as FingerControlAnalysis` and passes it as the single prop (analysis.tsx:347-352)
4. Rendered inside a `Card` with purple `border-t-2 border-t-purple-500/50` and a `ScrollArea h-72` (analysis.tsx:340-356)

## Key facts (from FingerControlProfile.tsx)
- Single prop: `FingerControlAnalysis` (camelCase `beatmapMd5`/`snapDistribution`/`burstHistogram`/`offGridDetails`/`offGridBuckets`/`transitionMatrix`/`timeline`, plus snake_case `overall_confidence`; types.ts:96-105)
- Five rendered sections:
  1. Numbered Burst Profile — 5 tiles for burst sizes 2–6 from `burstHistogram`
  2. Rhythmic Signature — horizontal stacked bar + legend from `snapDistribution`
  3. Rhythmic Instability by Map Section — sticky-header scrollable table of `offGridBuckets` (10 decile buckets, labeled "Section N (X%-Y%)"); nonzero rows get yellow highlight + inline bar
  4. Morphology & Transitions — 4 stat tiles from `transitionMatrix.categoryCounts` (odd-to-odd / even-to-even / odd-to-even / rhythmic resets), then scrollable percentage tables via `renderTransitionTable`: top snap transitions, bpmOrdinary/Minor/Major, top pattern transitions, `deltaGroups[0..3]`
  5. Technical Density Curves (SMA) — three recharts `LineChart`s sharing `syncId="fingerControl"`; all lines `isAnimationActive={false}` with `connectNulls={true}`; X axis numeric time formatted MM:SS via `formatTime` (lines 28-32)
- Snap-to-color map, fixed: 1/1 slate, 1/2 blue, 1/4 red, 1/3 yellow, 1/6 purple, 1/8 pink, 1/12 orange; unknown labels fall back to `bg-gray-400` (lines 10-18)
- Chart reset key: `analysis.beatmapMd5 || "default-key"` (line 25) applied as a React `key` on the timeline section wrapper (line 195) — forces the graph section to rebuild from scratch when a new map is analyzed

## State
- **Polished** — shadcn `Card`/`CardContent` + dark-themed recharts; Jump/Stream/Slider share the same card style but keep a few stray list-item (`<li>`) markup lines (JumpProfile.tsx:16, StreamProfile.tsx:25, SliderProfile.tsx:8) and no recharts charts
- Type duplication between `analysis.tsx` and `analysis_engine/types.ts` is **resolved**: `analysis.tsx` no longer defines local interfaces and imports everything from `./analysis_engine/types`; `types.ts` is the single source

## Quirks
- Mixed casing inside one interface: camelCase (`beatmapMd5`, `snapDistribution`, `offGridBuckets`, `timeline`) next to snake_case (`overall_confidence`) in `FingerControlAnalysis`
- `transitionMatrix.categoryCounts.rhythmicResets` (a number) and `transitionMatrix.rhythmicResets` (a `TransitionOccurrence[]` table) reuse the same name for different things
- `offGridDetails: OffGridNote[]` is declared in the type (types.ts:102) but never read by the profile — only `offGridBuckets` is rendered
- Leftover dev scaffolding comments in the component: "--- ADD THIS LINE HERE ---" and "Add this block back!" (lines 23-24, 27)
- Stale type union: `getBeatmapAnalysis`'s prop type is `"stream" | "jump" | "slider" | "all"` (analysis.tsx:36) while `BeatmapAnalysisResult.analysis_type` includes `"fingercontrol" | "aimcontrol" | "reading"` (types.ts:9). Call site only ever uses `"all"`, so it works today

## Related
- [[Analysis-Type]] — how analysis types are dispatched to the backend
- [[finger-control]] — backend module producing the data this profile renders
- [[Data-Philosophy]] — data conventions this renderer consumes
- [[overview]]

_Sources: frontend code (read 2026-08-07)_
