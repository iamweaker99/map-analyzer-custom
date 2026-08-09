---
type: module
status: stable
updated: 2026-08-08
---
# Module: aim-control

Purpose: aim difficulty — spatial layout (spacing/angle), movement kinematics (velocity), directional structure (flips/chirps/alignment), endurance strain (EMA), and a windowed complexity score (ACCV). One of the six analysis types alongside [[reading-analysis]], [[finger-control]], jumps, streams, sliders.

## Pipeline (mod.rs `analyze`)
1. `spatial::calculate_spatial_vectors` — consecutive-object movement vectors (tail→head)
2. `kinematics::calculate_kinematics` — per-movement velocity (+ momentum retention, unused)
3. `vectors::calculate_vector_mechanics` — flips, chirps, alignment counts
4. `endurance::calculate_endurance` — exponential-moving-average strain curve
5. `accv::calculate_accv` + `aggregate_accv` — sliding-window complexity
6. Emit JSON: `{spatial, kinematics, vectors, endurance, accv}` (mod.rs:70-115)

Returns `{"error": "Not enough objects for aim analysis"}` when no spatial vectors exist (mod.rs:24-26).

## Outputs & thresholds
**spatial** (mod.rs:34-51) — distances normalized to circle diameters via `get_diameter(cs) = 108.8 − 8.96·cs` (analysis/mod.rs:16-18). Buckets: `stacked ≤ 0.5`, `micro ≤ 1.25`, `flow ≤ 2.5`, `standard ≤ 4.5`, `large > 4.5` (diameters). Angles: `linear ≤ 45°`, `wide ≤ 90°`, `acute ≤ 135°`, `snap_backs > 135°`.

**kinematics** (mod.rs:61-68) — velocity z-bands around mean: `< μ−1.5σ` significantly_slower, `< μ−0.5σ` slower, `±0.5σ` mean, `≤ μ+1.5σ` faster, else significantly_faster.

**vectors** (vectors.rs) — `flip`: consecutive normalized dot < −0.5 (reversal > 120°). `chirp`: cross-product sign change between consecutive vectors (zig-zag). `alignment` compares vectors two apart: dot > 0.8 parallel, < −0.8 anti_symmetric, |dot| ≤ 0.3 orthogonal (the 0.3–0.8 bands are uncounted).

**endurance** (endurance.rs) — EMA strain, half-life 500 ms (`λ = ln2/500`); `mechanical_cost = velocity·(1 + deflection/180)` so a 180° deflection doubles cost. `peak_strain` = max; `time_under_tension_ms` = sum of dt where strain > 0.5·peak. `strain_curve` is one point per movement (can be a large array).

**accv** (accv.rs) — 4-movement sliding window (`window_size = 4`, accv.rs:32), invalidated if any `dt_break > 1000 ms`. `spatial_cv = σ/μ` of distances (0 if μ ≤ 5 px); `temporal_cv = σ/μ` of dt (0 if μ ≤ 10 ms); `kinetic_var = σ` of angles. `geometric_multiplier = 1 + 0.20·(deflections > 90°) + 0.35·chirps`; `base = spatial_cv + 1.5·temporal_cv + (kinetic_var/90)·multiplier`; `magnitude_multiplier = 1 + mean_velocity^1.8` (deliberately separates high-velocity 6*/9* aim, accv.rs:109). `total = base·magnitude`, ×0.7 if window contains a slider. Aggregates: 95th percentile = `peak_complexity`, 50th = `sustained_complexity`, plus 95th percentiles of the three variances.

## Integration
- Registered `pub mod aim_control` at analysis/mod.rs:5.
- Called in `backend/src/api/get/beatmap.rs:263` (`"all"`) and `:277` (`"aimcontrol"`); returns `serde_json::Value` directly (finger_control round-trips through `to_value` with a Null fallback — aim_control has no fallback).
- `analysis_type` string is `"aimcontrol"` (no underscore) — name mismatch vs module.
- Frontend contract: `AimControlResult` in `frontend/src/components/analysis_engine/types.ts:107-159`; rendered by `AimControlProfile.tsx` (flow-aim stat bars, ACCV card, strain card). `accv` is optional in the frontend type.

## Quirks
- `AimVector.end_time` is documented as "True end time (accounts for slider duration)" (vectors.rs:6) but spatial.rs:37-40 sets slider `end_time = start_time` — Phase 1 head-position tracking, comment says refined in Phase 2. So `dt_break == dt` for sliders.
- Spinners: tail position forced to playfield center (256, 192), zero duration (spatial.rs:41-45) — manufactures a fake centerward movement.
- Two different velocities exist: `AimVector.velocity` uses `safe_dt = dt_break` (spatial.rs:27-28), while kinematics recomputes `norm_distance / dt` (kinematics.rs:15). The kinematics version drives all downstream outputs.
- `momentum_retention` is computed but never serialized or consumed (`#[allow(dead_code)]`, kinematics.rs:3,8).
- First movement has `deflection_angle = None` (no predecessor) — filtered out of angle aggregates (mod.rs:35).
- `dt == 0` (simultaneous objects) is unguarded in kinematics — can produce inf/NaN velocity.
- ACCV windows overlap: each movement participates in up to 4 windows.
- `{"error": ...}` response has no branch in the frontend `AimControlResult` type — an error value would fail the cast/render.

## Files
`backend/src/analysis/aim_control/{mod, spatial, kinematics, vectors, endurance, accv}.rs`

_Sources: code (read 2026-08-08); API wiring beatmap.rs; frontend types.ts / AimControlProfile.tsx_
