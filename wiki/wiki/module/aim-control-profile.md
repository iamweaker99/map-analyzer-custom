---
type: module
status: stable
updated: 2026-08-08
---
# Module: aim-control-profile (frontend)

Purpose: renders the Aim Control analysis card — spatial/angle/alignment distributions, ACCV complexity metrics, and the sustained strain timeline — in the results grid.

## Files
- `frontend/src/components/analysis_engine/AimControlProfile.tsx` — the profile component
- `frontend/src/components/analysis_engine/types.ts` — `AimControlResult` interface (lines 107–159); `analysis_type` union includes `"aimcontrol"` (line 9)
- `frontend/src/components/analysis_engine/StatBar.tsx` — shared percentage bar used by all distribution sections
- `frontend/src/components/analysis.tsx` — host page: fetches, picks the section, mounts the profile

## Pipeline / data flow
1. `analysis.tsx handleSubmit` calls `getBeatmapAnalysis(+beatmapId, "all")` (analysis.tsx:76).
2. Backend (`backend/src/api/get/beatmap.rs:272`) emits each analysis as `{ analysis_type, analysis }`; aim section built by `backend/src/analysis/aim_control/mod.rs::analyze()` (`analysis_type: "aimcontrol"`).
3. Frontend finds the section via `analysisResult.find((a) => a.analysis_type === "aimcontrol")` (analysis.tsx:105–107) and casts `.analysis` to `AimControlResult`.
4. Rendered inside a cyan-bordered Card (`border-t-cyan-500/50`) in Row 3 (Finger Control | Aim Control | Reading) of the 3×2 grid, inside an `h-72` ScrollArea (analysis.tsx:358–374).

## What the card renders
- **Version guard** (AimControlProfile.tsx:36–42): if `data.spatial.spacing_distribution` is missing, shows a red "Backend Version Mismatch" box telling the user to `cargo clean && cargo run`. This is the contract gate for the Stage 3 data shape.
- **Stat cards** (2×4 grid, lines 56–73): Avg Spacing (`spatial.avg_spacing_d`, unit `D`, 2 decimals), Avg Velocity (`kinematics.avg_velocity`, `px/ms`), Dir Flips (`vectors.directional_flips`, orange), Peak Strain (`endurance.peak_strain`, red, 0 decimals).
- **Spacing Profile** — StatBars over `spatial.spacing_distribution`, count + % of total (line 50): `stacked` gray, `micro` blue, `flow` emerald, `standard` orange, `large` red.
- **Angles (Pathing)** — StatBars over `spatial.angle_distribution`: `linear` emerald, `wide` blue, `acute` orange, `snap_backs` red (line 51).
- **Alignment (Pattern Logic)** — `vectors.directional_chirps` shown as a raw count row, plus StatBars over `vectors.alignment`: `parallel` emerald, `orthogonal` orange, `anti_symmetric` red (line 52).
- **ACCV Complexity Dashboard** (lines 114–146, comment "NEW: ACCV"): rendered only `if (data.accv)` — Peak (95%) `peak_complexity` red, Sustained (50%) `sustained_complexity` blue, Spatial Var `peak_spatial_cv`, Temporal Var `peak_temporal_cv`, Kinetic Var `peak_kinetic_var` (2 decimals).
- **Sustained Aim Strain** (lines 148–178): recharts `LineChart` of `endurance.strain_curve`, mapped to `{ timeMs: point.time, strain: parseFloat(point.strain.toFixed(2)) }` (line 44–48). X axis ticks formatted MM:SS via local `formatTime` (takes `any`, comment: bypassing strict TS); custom `CustomTooltip` because the formatter prop was buggy (line 20); red `#f87171` line, `margin right: 20` added to fix cut-offs (line 162). "Time Under Tension" header shows `endurance.time_under_tension_ms` formatted.

## Contracts / coupling to backend JSON keys
- Keys are snake_case and must match `aim_control/mod.rs` output exactly (lines 70–115): `spatial`, `kinematics`, `vectors`, `endurance`, `accv`.
- Backend spacing thresholds (mod.rs:44–51), in circle diameters: `stacked` ≤0.5d, `micro` ≤1.25d, `flow` ≤2.5d, `standard` ≤4.5d, else `large`.
- Backend angle thresholds (mod.rs:53–59): `linear` ≤45°, `wide` ≤90°, `acute` ≤135°, else `snap_backs`.
- Backend `analyze()` returns `{"error": "Not enough objects for aim analysis"}` for empty spatial vectors (mod.rs:24–26) — the frontend type does not model the error shape; the version-mismatch guard is the only fallback.
- Analysis-type string is `"aimcontrol"` (no underscore), same in both directions (beatmap.rs:272, analysis.tsx:105).

## Quirks
- **Unused data**: `kinematics.velocity_distribution` (5 bands) and `kinematics.velocity_std_dev` exist in `AimControlResult` and are produced by the backend (mod.rs:89–98) but the profile never renders them. `spatial.avg_angle`, `spatial.total_movements`, `kinematics.avg_velocity`'s std dev are also unrendered.
- **Duplicated types**: `AimControlResult` is defined in `analysis_engine/types.ts` and imported by `analysis.tsx` (analysis.tsx:23) — same duplication pattern as Finger Control (see [[finger-control-profile]]).
- **No `?.` on ACCV fields**: `data.accv &&` guards the block, but the five `.toFixed(2)` calls (lines 126–142) would throw if a partial `accv` object ever arrived.
- **Rounding**: strain curve values are rounded to 2 decimals via `parseFloat(...toFixed(2))` — chart is a rounded approximation of backend EMA data.
- **`any` typing**: `formatTime(ms: any)` and `CustomTooltip({...}: any)` — deliberate bypasses of strict TS for recharts axis/tooltip props.
- Mixed cast style: `spacing_distribution` totals summed with `(a: any, b: any)` reducers (lines 50–52).

## Related
- [[aim-control]] — backend module producing the data this profile renders
- [[Analysis-Type]] — the `analysis_type` union / section model
- [[Data-Philosophy]] — how backend JSON shapes are consumed by the UI
- [[finger-control-profile]] — sibling profile with the same grid/card pattern
- [[overview]]

_Sources: frontend code (read 2026-08-07), backend code (read 2026-08-07)_
