---
type: module
status: stable
updated: 2026-08-08
---

# slider-profile

Frontend card ("Sliders") that renders the backend slider-analysis JSON as three
profile groups of stat bars: length (relative to map density), buzz, and
artistic shape (both relative to slider count).

## Purpose

Display the slider-skill profile of a beatmap: what fraction of objects are
sliders, how long they are vs. the circle diameter, how many are buzz
(repeated) sliders, and how complex their control-point geometry is. One of the
six cards on the analysis results page — see [[Analysis-Type]] for the card
set and [[Data-Philosophy]] for the dens/count metric conventions.

## Files

- `frontend/src/components/analysis_engine/SliderProfile.tsx` — the card body.
- `frontend/src/components/analysis_engine/types.ts` — `SliderAnalysis` interface (lines 38-45) and `BeatmapAnalysisResult` union (lines 7-19).
- `frontend/src/components/analysis_engine/utils.ts` — `getSliderTag()` (lines 17-21).
- `frontend/src/components/analysis_engine/StatBar.tsx` — shared bar used by all profile cards.
- `frontend/src/components/analysis.tsx` — integration: find + mount the card.
- `frontend/src/app/page.tsx` — server action that fetches the backend JSON.
- `backend/src/analysis/sliders.rs` — producer of the JSON section.

## Pipeline / data flow

1. `handleSubmit` in `analysis.tsx:59-91` calls `getBeatmapAnalysis(beatmapId, "all")` (analysis.tsx:76).
2. That server action (`page.tsx:25-45`) fetches `GET {BACKEND_URL}/api/beatmaps/{id}/analyze/all` and returns `BeatmapAnalysisResult[]`.
3. `sliderResult = analysisResult?.find(a => a.analysis_type === "slider")` (analysis.tsx:99-101); render is gated on it (analysis.tsx:319).
4. The card casts `sliderResult.analysis as SliderAnalysis` (analysis.tsx:326-331) and renders it inside a "Sliders" `Card` (green top border), `ScrollArea h-72`, in Row 2 grid Jumps | Streams | Sliders.
5. Backend producer `sliders.rs::analyze(map, cs, total_obj)` iterates `HitObjectKind::Slider` objects and emits one flat JSON object with all `SliderAnalysis` fields (sliders.rs:44-58).

## Data shape consumed (SliderAnalysis)

All from `types.ts:38-45`; the backend emits exactly these keys (sliders.rs:44-58):

- `overall_confidence`, `slider_ratio` — both = slider count / total objects.
- `avg_velocity` — mean slider body length / 100 (see quirks).
- Length profile (count + dens per bucket, thresholds in circle diameters `d = get_diameter(cs)`): `l_short` (<1.5d), `l_med` (1.5-3.0d), `l_long` (3.0-4.5d), `l_ext` (>=4.5d) (sliders.rs:19-23).
- Buzz profile: `b_buzz_count/dens`, `b_static_count/dens` — only repeated sliders (`repeat_count > 0`); body_len < 5.0 → static, else buzz (sliders.rs:26-29).
- Artistic profile by control-point count: `a_simple` (<=2), `a_curved` (<=4), `a_complex` (<=10), `a_artistic` (>10) (sliders.rs:32-36).
- Densities: length dens are `/ total_obj` (all objects); buzz and artistic dens are `/ sl_count` (sliders only) — matching the card's "Rel. to Map" vs "Rel. to Sliders" headers (sliders.rs:48-57).

## What the card renders

- Header line: `Style: {getSliderTag(slider_ratio)} (Avg SV: {avg_velocity.toFixed(2)})` (SliderProfile.tsx:8-11). `getSliderTag` (utils.ts:17-21): ratio < 0.30 → "Mechanical Tech", < 0.60 → "Technical", else "Slider Tech".
- Three titled groups of `StatBar`s (SliderProfile.tsx:13-89):
  - "Length Profile (Rel. to Map)" — 4 bars: Short/Medium/Long/Extended.
  - "Buzz Profile (Rel. to Sliders)" — 2 bars: Buzz Sliders / Static Buzz.
  - "Artistic Profile (Rel. to Sliders)" — 4 bars: Simple (Linear) / Curved / Complex / Artistic-Tech.
- Each `StatBar` shows raw count and `(dens * 100)` as percentage (e.g. `percentage={(analysis.l_short_dens || 0) * 100}`, SliderProfile.tsx:20). `StatBar` (StatBar.tsx:22-27) uses the direct `percentage` prop (it takes priority over value/total); the bar width is that percentage (StatBar.tsx:46-51).
- No charts/timelines — purely stat bars; unlike Finger Control or Reading cards.

## Quirks

- `overall_confidence` and `slider_ratio` are literally the same value (`sl_f / total_obj`, sliders.rs:45-46), and the card renders neither — `overall_confidence` is declared in `SliderAnalysis` but never used by `SliderProfile`.
- `avg_velocity` is not velocity: it is average `body_len / 100` ("Normalized SV representation", sliders.rs:38,47) — average slider body length, not scroll velocity.
- Buzz profile counts only repeated sliders; non-repeated sliders fall in neither `b_buzz` nor `b_static` (sliders.rs:26).
- "Simple (Linear)" is decided by control-point count (<=2), not geometric linearity (sliders.rs:32).
- UI label "Extended (>4.5x D)" vs backend `else` branch: body length exactly 4.5d lands in `l_ext` (>=, not >).
- The style header is a bare `<li>` with no `<ul>` wrapper (SliderProfile.tsx:8) — invalid HTML nesting.
- Densities travel as fractions (0-1) and are converted to percent only at render (`* 100`); `|| 0` fallbacks on every field make the card tolerant of missing JSON keys.

## Contracts other code depends on

- The backend JSON key names must match the `SliderAnalysis` interface exactly — the card casts without validation (analysis.tsx:326-331).
- `analysis_type: "slider"` must be present in the `BeatmapAnalysisResult` union and returned by the `/analyze/all` endpoint for the card to mount (types.ts:9, analysis.tsx:99).
- `getSliderTag` boundary values (0.30 / 0.60) are hard-coded in `utils.ts:17-21` — changing backend `slider_ratio` semantics shifts the style label.

## Integration points

- Mounted only via `analysis.tsx` (Row 2, "Sliders" card) — no other consumers.
- Shares `StatBar` with the other profile cards ([[finger-control-profile]], [[aim-control-profile]], [[jump-profile]], [[stream-profile]]).
- Downstream of the `sliders.rs` analyzer; see [[sliders]] for the analysis-type page.

## Related

[[Analysis-Type]] · [[Data-Philosophy]] · [[sliders]]
