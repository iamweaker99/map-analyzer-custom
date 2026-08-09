---
type: module
status: stable
updated: 2026-08-08
---
# Module: reading-profile

Purpose: the frontend "Reading" card — renders the `reading` analysis JSON section (produced by [[reading-analysis]]) as four chart/stat blocks: cognitive strain candlestick topography, visual clutter + trajectory chaos distributions with timelines, and relational deception traps.

## Files
- `frontend/src/components/analysis_engine/ReadingProfile.tsx` — the card component itself
- `frontend/src/components/analysis_engine/types.ts` — `ReadingResult`, `KLineData`, `TrajectoryTimelinePoint`, `DensityTimelinePoint` (types.ts:161-214)
- `frontend/src/components/analysis_engine/StatBar.tsx` — percentage bar used by the density/trajectory sections
- `frontend/src/components/analysis.tsx:376-392` — integration point: renders `<ReadingProfile>` inside a Card (amber top border, `h-72` ScrollArea) when the analysis section is present
- Producer (backend): `backend/src/analysis/reading/mod.rs` `analyze()` — JSON shape at mod.rs:173-209

## Data flow
Backend `analyze()` JSON → API returns `{ analysis_type: "reading", analysis: ReadingResult }` → frontend picks it via `analysisResult.find(a => a.analysis_type === "reading")` (analysis.tsx:108) → `data={readingResult.analysis as ReadingResult}` — a bare type cast, **no runtime validation** (component takes `data: any`).

## Consumed fields (each grounded in the backend shape)
- `topography.klines` — candlesticks `{window_start, open, high, low, close, volume}` (KLine struct, strain.rs:14-21; `volume` = number of strain samples in the window). Rendered as a hand-rolled candlestick chart, not recharts.
- `summary.peak_strain` — 95th percentile of kline highs (mod.rs:168-171). Displayed as "Peak Reading Strain (95th)".
- `density.{isolated,chunking,clutter,overload}_pct` — share of windows with effective-object count 0-2 / 3-5 / 6-8 / 9+ (mod.rs:70-78, 178-183).
- `trajectory.{linear,mild_shifts,sharp_kinks,spaghetti}_pct` — entropy-based buckets (<30, <90, spaghetti flag) (mod.rs:81-92).
- `density_timeline` / `trajectory_timeline` — per-5-second-window counts (mod.rs:94-156) → recharts multi-line charts.
- `traps.{count, trap_index, peak_magnitude, notable_traps}` — `trap_index` = traps per 1000 nodes (mod.rs:166); `notable_traps` = top 5 by magnitude (mod.rs:158-164).
- **Not consumed**: `summary.ar_preempt_ms` (typed at types.ts:189, never read) and the whole `sequence_motor` section (backend mod.rs:201-208) — absent from the `ReadingResult` type entirely.

## Chart blocks (top to bottom)
1. **Cognitive Strain Topography** — hand-rolled candlesticks: 5px-wide divs with wick + body, y-axis markers at `maxStrain` / 0.66x / 0.33x / 0, time labels every 12th candle, custom fixed-position tooltip that flips side near the screen edge (ReadingProfile.tsx:22-31).
2. **Visual Clutter** — 4 StatBars plus optional "Visual Clutter Over Time" recharts LineChart (isolated/chunking/clutter/overload counts).
3. **Trajectory Chaos** — 4 StatBars plus optional "Trajectory Breakdown Over Time" LineChart (linear/mild shifts/sharp kinks/spaghetti counts).
4. **Relational Deception (Traps)** — trap index shown as "x / 1k", top-5 notable traps with magnitude badges (red badge when `magnitude > 2.5`), and a total traps count.

## Quirks
- Candle color is **inverted vs. finance convention**: up (close >= open) = red, down = emerald (ReadingProfile.tsx:98, 110).
- Only guard is `!data || !data.topography` → "No reading data." placeholder (line 16); malformed deeper fields would crash.
- Zero-height candle bodies clamped to 2% of chart height (line 96); `maxStrain` floored at 10 to avoid div-by-zero (line 19).
- `.slice(0, 5)` on `notable_traps` (line 204) is redundant — backend already truncates to 5.
- Defensive `|| 0` fallbacks on `peak_strain` and `trap_index` (lines 77, 194).
- Code comment "RESTORED PEAK VALUE HERE" (line 75) — peak strain was removed and re-added at some point.

## Integration points
- Sibling cards (jump/stream/slider/fingercontrol/aimcontrol profiles) are dispatched from the same `analysis_type` find-chain in analysis.tsx:94-108 — adding a new analysis type requires touching both the API union and the find-chain.
- `BeatmapAnalysisResult` (types.ts:7-19) documents the required extension points in comments: add `"reading"` to the `analysis_type` union and `ReadingResult` to the `analysis` union.
- Follows [[Data-Philosophy]]: backend metrics land as sibling JSON sections; the card only renders keys it knows.

## Related
[[reading-analysis]], [[Analysis-Type]], [[Data-Philosophy]], [[reading-hub]]

_Sources: code read 2026-08-08_
