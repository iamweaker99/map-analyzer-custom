---
type: module
status: stable
updated: 2026-08-08
---
# Module: sliders

Purpose: quantify slider usage — share of sliders, body-length profile, buzz (repeated) sliders, and control-point complexity.

## Files
- `backend/src/analysis/sliders.rs` — the analysis
- Depends on `get_diameter()` from `backend/src/analysis/mod.rs:16-18`

## Pipeline / data flow
1. Iterates `map.hit_objects` directly on the full `osu_map_analyzer::rosu_map::Beatmap` (sliders.rs:14) — unlike [[jumps]]/[[streams]] it does NOT consume the `Movement` list, because slider body geometry only exists in hit-object space.
2. For each `HitObjectKind::Slider`: `body_len = s.path.expected_dist()` (sliders.rs:17), `points = s.path.control_points().len()` (sliders.rs:32), `repeat_count`.
3. Three independent profiles → flat JSON ([[Data-Philosophy]]).

## Profiles & thresholds (d = circle diameter = 108.8 - 8.96*cs)
| Profile | Bins |
|---|---|
| Length (relative to density) | short `< 1.5d` · med `< 3.0d` · long `< 4.5d` · extreme `≥ 4.5d` (sliders.rs:20-23) |
| Buzz (repeat_count > 0) | static if `body_len < 5.0` (raw px) else buzz (sliders.rs:26-29) |
| Artistic (control points) | simple `≤ 2` · curved `≤ 4` · complex `≤ 10` · art `> 10` (sliders.rs:32-36) |

## Output (serde_json Value, sliders.rs:44-58)
- `overall_confidence` = slider count / total_obj (identical to `slider_ratio`)
- `avg_velocity` = Σ(body_len / 100) / slider_count — normalized mean path length
- `l_short/med/long/ext_count` + `*_dens` (count / total_obj)
- `b_buzz` / `b_static` counts + dens (dens / slider count)
- `a_simple/curved/complex/artistic_count` + `*_dens` (dens / slider count)

## Quirks
- `avg_velocity` is a misnomer: it is mean body length in units of 100 px, not a velocity.
- Denominators are inconsistent: length densities divide by total_obj; buzz/artistic densities divide by slider count (sliders.rs:52-57).
- The buzz "static" threshold `body_len < 5.0` mixes units — raw pixels against the diameter-relative length profile; a static buzz slider is one whose entire body is under 5 px.
- `overall_confidence` and `slider_ratio` duplicate the same count.
- `expected_dist()` failing (`unwrap_or(0.0)`) silently counts the slider as zero-length.

## Fits [[Analysis-Type]]
Slider is one of the six analysis dimensions; classification-layer analysis (quantity-based, lossy by design — [[Analysis-Type]]). Rendered by the frontend as the **slider profile** card ([[slider-profile]]).

## Related
[[Analysis-Type]] · [[Data-Philosophy]] · [[streams]] · [[jumps]] · [[slider-profile]]

_Sources: code (read 2026-08-07)_
