---
type: module
status: stable
updated: 2026-08-08
---
# Module: stream-profile (frontend)

Purpose: renders the Streams analysis card in the results grid (Row 2: Jump | Stream | Slider). Pure presentational component — it only renders; the parent fetches and hands it the already-parsed analysis JSON section.

## Files
- `frontend/src/components/analysis_engine/StreamProfile.tsx` — the profile card
- `frontend/src/components/analysis_engine/types.ts` — `StreamAnalysis` interface (lines 29-36)
- `frontend/src/components/analysis_engine/utils.ts` — `getStreamSpacingTag` (lines 9-15)
- `frontend/src/components/analysis_engine/StatBar.tsx` — shared label + progress-bar row
- `frontend/src/components/analysis.tsx` — parent: finds the section and renders the card (lines 96-98, 300-317)
- Backend producer: `backend/src/analysis/streams.rs` (the `analyze()` fn emitting this JSON section)

## Data consumed (StreamAnalysis, types.ts:29-36)
Snake_case JSON from backend, mirrored 1:1 in the TS interface:
- `overall_confidence`, `total_stream_patterns`, `circle_diameter`
- Spacing counts: `s_stacked_count`, `s_overlapping_count`, `s_spaced_count`, `s_extreme_count`
- Spacing densities: `s_stack_dens`, `s_over_dens`, `s_space_dens`, `s_extr_dens`
- `avg_stream_spacing`
- Variance counts: `v_steady_count`, `v_variable_count`, `v_dynamic_count`
- Length counts: `bursts`, `short_streams`, `medium_streams`, `long_streams`, `death_streams`
- `max_stream_length`, `stream_density`, `bpm_consistency`

## What the card renders
1. **Header line** — `Type: {getStreamSpacingTag(avg, d)} ({avg.toFixed(1)} px)` where `avg = avg_stream_spacing` and `d = circle_diameter || 73`. Tag thresholds (utils.ts:9-15): `0 → "N/A"`; `< 0.5*d → Stacked`; `< 1.0*d → Overlapping`; `< 2.0*d → Spaced`; else `Extreme (Jump-Stream)`.
2. **Distance Profile (Density by Notes)** — 4 StatBars with the backend densities as direct percentages (`s_*_dens * 100`): Stacked (<0.5x D, green), Overlapping (0.5-1x D, blue), Spaced (1-2x D, orange), Extreme (2-2.5x D, red).
3. **Variance Profile** — 3 StatBars computed as value/total over `total_stream_patterns`: Steady (green), Variable (blue), Dynamic (red).
4. **Length Profile** — 5 StatBars as value/total over a locally computed `totalLength` = sum of `bursts + short_streams + medium_streams + long_streams + death_streams`: Bursts (3-4), Short (5-12), Medium (13-24), Long (25-48), Deathstream (49+).
5. **Footer rows** — `Max stream: {max_stream_length} notes` and `BPM Consistency: {bpm_consistency * 100}%`.

StatBar semantics (StatBar.tsx): a `percentage` prop is used directly; otherwise `value/total*100`. Bar is a 1px Tailwind color strip; count + percentage shown in a mono label.

## Backend semantics (what the numbers mean, streams.rs)
- Stream membership (streams.rs:18): `time_gap <= (60000/bpm/4)*1.5` (1.5x a 1/4 beat at map BPM) AND `0 < distance <= 2.5*d`.
- Consecutive stream gaps are buffered; a pattern is flushed when the chain breaks (streams.rs:21-41).
- Length bins (streams.rs:25-27): 3-4 → burst; 5-12 → short; 13-24 → medium; 25-48 → long; 49+ → deathstream. `max_stream_length` = longest pattern note count.
- Spacing counts (`s_*_count`) and variance counts are **pattern-level**: computed from the mean / coefficient of variation of a pattern's gaps, only for patterns of 5+ notes (streams.rs:27-32).
- Spacing densities (`s_*_dens`) are **gap-level**: each gap counted per distance bin, divided by `total_obj` (streams.rs:33-36).
- `overall_confidence` = total stream gaps / `total_obj` — fraction of map notes participating in streams (streams.rs:52).
- `avg_stream_spacing` = mean of all stream gaps (streams.rs:53).
- `bpm_consistency` = `(1 - cv(time_gaps)).max(0)` (streams.rs:44-49).

## Quirks
- UI label says "Extreme (2-2.5x D)" but backend bins `dist >= 2.0*d` with no upper bound (streams.rs:35) — the 2.5x cap only applies to stream membership (streams.rs:18). Label is a misnomer.
- **Bursts (3-4 notes) are excluded from Distance Profile and Variance Profile** — pattern-level counts and gap-level densities only accumulate for 5+ note patterns (streams.rs:26-37). Bursts appear only in the Length Profile.
- Density percentages don't sum to 100 across the four bars: denominators are `total_obj` (map-wide), not pattern or gap counts.
- `totalObjects` prop is declared (analysis.tsx:312) but never used inside StreamProfile — the component computes its own `totalLength`.
- `overall_confidence` and `stream_density` exist in the type but are never rendered by the card.
- Fallbacks: `circle_diameter || 73`, counts default to 0 via `||` (StreamProfile.tsx:12-21).

## Integration points
- Parent (analysis.tsx:96-98) selects the section with `find(a => a.analysis_type === "stream")`, casts `.analysis as StreamAnalysis`, and renders inside a blue-accented Card titled "Streams" with a `h-72` ScrollArea (analysis.tsx:300-317).
- Card data contract = the exact JSON keys emitted by `streams.rs` (streams.rs:51-60); renaming either side breaks rendering silently (missing keys fall back to defaults).
- One of the six cards of the analysis-type result grid; sibling cards are jump/slider/finger-control/aim-control/reading profiles.

## Related
- [[streams]] — backend module producing this data
- [[Analysis-Type]] — the "stream" analysis type this profile visualizes
- [[Data-Philosophy]] — density/count conventions behind these metrics

_Sources: frontend code and backend/src/analysis/streams.rs (read 2026-08-07)_
