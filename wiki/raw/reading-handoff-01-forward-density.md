# Handoff: Forward-Looking Note Density — Reading Analysis Prototype

## Summary

Prototype a forward-looking note density metric for beatmap reading analysis, inspired by the official osu! reading skill PR (ppy/osu#33196). This is a **new standalone metric** to be compared against the existing visual density in the reading analysis module.

## Context

The project is [map-analyzer-custom](https://github.com/iamweaker99/map-analyzer-custom) — it reveals raw beatmap data that contributes to skills without assigning difficulty weightage. The reading analysis module lives in `backend/src/analysis/reading/`.

The existing density calculation (`density.rs`) counts how many notes are **currently visible** on screen at each object's start time (based on fade_in_time from AR). This is a "snapshot at hit moment" approach.

The official osu! reading PR introduces both past-visible and **future-visible** objects within a 3000ms sliding window. The future density looks at notes that will appear soon after the current note is hit — which captures "what's coming up" cognitive load that the current density misses.

## What to Build

A new module or function that computes, for each hit object in a beatmap:

- **`forward_note_count`**: Number of notes whose `start_time` falls within the range `[current_note.start_time, current_note.start_time + 3000ms]`
- Per-object values → aggregate into a distribution (e.g., what % of notes have 0-3, 4-6, 7-10, 11+ future notes nearby)

### Key Design Decisions (Already Made)
- **Window size**: 3000ms (matches the official PR for comparability)
- **No weighting/decay**: Just a raw count — no exponential falloff or influence factors
- **NM only**: No mods, no Hidden/Fade-in considerations
- **Separate metric**: Not replacing the existing visual density, kept as a parallel measurement

### Integration Points

- Input: `&[VisualNode]` from `visuals.rs` (has `start_time`, `end_time`, `fade_in_time`, `x`, `y`)
- New output: Include in the JSON returned by `mod.rs::analyze()`
- Frontend: `ReadingProfile.tsx` + `types.ts` will need type updates
- Discord bot: `reading.rs` embed + `types.rs` for deserialization

## Suggested Skills

- `/prototype` — Use this skill to build and test the forward-looking density calculation
- `/code-review` — After prototyping, review the implementation for edge cases (empty maps, single objects, very dense sections)

## References

- Official osu! PR: https://github.com/ppy/osu/pull/33196
  - Key file: `ReadingEvaluator.cs` — `retrieveCurrentVisibleObjectDensity` method
  - Uses 3000ms (`reading_window_size`) sliding window, opacity weighting, and time nerf factor (we skip the weighting)
- Existing code: `backend/src/analysis/reading/density.rs` — current visual density for comparison
- Existing code: `backend/src/analysis/reading/visuals.rs` — VisualNode struct and extraction

## Open Questions for Prototyping

1. How does the forward-looking density curve differ from the existing visual density curve on the same maps?
2. On maps with dense stream sections vs. sparse jump sections, which metric better captures the "clutter" feeling?
3. Should the window size be fixed 3000ms or should multiple windows be reported?
4. After seeing results: should these two density metrics be merged or kept separate?

## Success Criteria

The prototype should be able to output per-object forward note counts for any beatmap, and we should be able to visually compare the distribution against the existing density data (e.g., overlay both curves on the same timeline).
