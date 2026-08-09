# PRD: Pattern-Aware Intra-Pattern Reading Analysis

**Status:** Draft  
**Date:** 2026-07-16  
**Origin:** Cross-check with osu! official reading skill PR (ppy/osu#33196)  
**Prototype phase:** Completed — angle distribution (15° buckets, stats, 1000ms forward density window)  
**This PRD:** Focused on intra-pattern analysis for 2/3/4 note patterns + forward-looking density.  
**Next PRD (planned):** Inter-pattern/transition analysis.

---

## Problem Statement

The current reading analysis treats all notes as a flat sequence. It computes visual density, trajectory entropy, reading traps, and cognitive strain across sliding windows — but it has no awareness of *pattern structure*: how notes group into doubles, triples, and 4-note patterns.

This means:
- A map with many sharp-angle triples looks the same as a map with gentle-flow triples at the trajectory-entropy level, because entropy measures *change in angles*, not the angles themselves.
- A map with dense 4-note patterns and a map with sparse isolated notes get similar density readings, because density counts *currently visible* notes without distinguishing pattern structure.
- The *rhythmic context* of patterns (1/2 snap vs. 1/4 snap) is invisible to the reading analysis, even though it's already detected by the finger_control module.

The osu! official reading PR (ppy/osu#33196) confirmed that per-object angle data and forward-looking note density are meaningful signals for reading analysis. We need to bring pattern-aware geometry metrics to the reading module — without crossing into difficulty assignment or weightage.

---

## Solution

Add two new families of raw metrics to the reading analysis, both reporting flat (non-nested, unweighted) values:

### 1. Intra-Pattern Angle & Spacing

Leverage the existing pattern detection from `finger_control` to segment the beatmap into patterns (Jump, Burst, Stream, Slider). Then compute angle and spacing metrics *inside* each detected pattern — limited to 2/3/4 note patterns for scope discipline.

- **Doubles (2 notes):** Report spacing only (no interior angle exists for a note pair).
- **Triples (3 notes):** Report the interior angle at the middle node + spacing between consecutive pairs.
- **4-note patterns:** Report interior angles at both internal nodes + spacing between consecutive pairs.

All angles use the dot-product formula prototyped and validated in the angle distribution prototype (`acos` of normalized dot product, degrees 0-180). 15° buckets were chosen as the default granularity (12 bins: 0-15°, 15-30°, ..., 165-180°) — confirmed informative across test maps.

Angle and spacing are reported alongside each other because they are correlated (not causative) and must be assessed together.

### 2. Forward-Looking Note Density

For each hit object, count how many notes fall within the next 1000ms window. Prototyping confirmed 1000ms as the sweet spot — it captures "what's coming up soon" cognitive load without excessive noise. This is a separate raw metric from the existing visual density (which counts currently-visible notes at the hit moment), and both co-exist in the output.

### Data Philosophy

All new metrics follow the project's core principle: **raw data, no interpretation.** No difficulty weightage is assigned. No arbitrary thresholds classify patterns as "hard" or "easy." The consumer (user or frontend) interprets the data — the analysis only reveals what exists in the beatmap.

---

## User Stories

1. As a user, I want to see the distribution of interior angles for triples and 4-note patterns in a beatmap, so that I can understand the geometric reading complexity independent of trajectory entropy.
2. As a user, I want to see the spacing values for doubles, triples, and 4-note patterns, so that I can assess spatial density per pattern type.
3. As a user, I want to see a forward-looking note density (1000ms window) for each object, so that I can understand the visual anticipation load throughout the map.
4. As a user, I want angle and spacing data reported as flat, non-nested values (no difficulty weightage, no interpretation), consistent with the project's data philosophy.
5. As a user, I want the rhythmic snap (1/2, 1/4, 1/6, etc.) included alongside each pattern's data, so that I can distinguish between rhythmically different patterns with similar geometry.
6. As a user, I want the raw per-object angle values available for timeline plotting, so that I can see where sharp turns cluster in the map.
7. As a user, I want summary statistics (mean, median, stddev) alongside the angle distribution, so that I can quickly characterize a map's overall angle profile.
8. As a user, I want the forward density metric computed independently of the existing visual density metric, so that I can compare what each reveals about the same map.

---

## Implementation Decisions

### Pattern Detection
- Reuse the existing pattern detection from `finger_control::patterns::extract_patterns()`. This function groups consecutive circles by temporal proximity (≤ half-beat + 10ms) and labels each pattern with its type (Jump, Burst(n), Stream, Slider) and rhythmic snap.
- For this PRD's scope, filter detected patterns to **Burst(2), Burst(3), and Burst(4)** only. Longer patterns (Burst 5-6n, Stream 7+n) are deferred to a future iteration.
- The `snap` field from pattern detection carries through — a 1/2 triple and a 1/4 triple are distinguishable in the output.

### Angle Computation
- Use the prototyped dot-product formula: interior angle = `acos(dot(AB, BC) / (|AB|·|BC|))` for each consecutive triple, reported in degrees (0-180).
- Default bucket granularity: **15° bins** (0-15, 15-30, ..., 165-180) — 12 buckets total. This was selected after testing 15° vs 10° on real beatmaps; 10° added noise without revealing new structure.
- Companion summary statistics: **mean, median, stddev** — these capture distribution shape that bucket counts alone cannot express (e.g., mean=56°/median=24° reveals rare but extreme sharp turns).
- Reported per-object as a time series AND as an aggregate distribution.

### Spacing Computation
- Use existing Euclidean distance between consecutive note positions (same formula as `Movement::distance` in `analysis/mod.rs`).
- Reported per-pattern as: mean spacing, stddev of spacing (consistency), min, max.
- For doubles: spacing is the only metric (no interior angle).
- Spacing data is emitted alongside angle data because they are correlated and must be assessed together.

### Forward-Looking Density
- Window size: **1000ms** (confirmed by prototyping as optimal).
- For each object's `start_time`, count notes whose `start_time` falls in `[start_time, start_time + 1000ms]`.
- Raw count only — no exponential decay, no opacity weighting, no time nerf factor.
- Reported per-object as a time series AND as an aggregate distribution (e.g., % of notes with 0-3, 4-6, 7-10, 11+ upcoming notes).

### Output Format
- All new metrics are added as **flat fields** in the existing reading JSON — no nested per-pattern-type sections.
- The forward density is a sibling of the existing density section, not a replacement.
- Per-object angle values are included alongside aggregate distributions so the frontend can render a timeline if desired.

### Inter-Pattern Boundary for Angles
- For a 4-note pattern with nodes (A-B-C-D), angles are computed for triples (A-B-C) and (B-C-D) — strictly inside the pattern.
- The angle does NOT take the last note of the previous pattern or the first note of the next pattern as context. Transition notes are treated as outliers (they typically occur at less demanding musical sections).
- *Note for future:* The inter-pattern angle approach was discussed but not decided — two approaches were considered (taking the previous pattern's last note OR the next pattern's first note as angle context at boundaries). This is deferred to the next PRD.

---

## Testing Decisions

### What makes a good test
- Tests should validate that the computed values are **mathematically correct** (angle formula, spacing formula, count accuracy).
- Tests should compare results across **known different map types** (tech maps, flow maps, jump maps) to confirm the metrics meaningfully distinguish them.
- Tests should **not** assert difficulty or weightage — this is a measurement system, not a rating system.

### Test categories

| Category | What to test | Comparison target |
|---|---|---|
| **Angle correctness** | Dot-product formula on known geometries (straight line = 180°, right angle = 90°, reversal = 0°) | Expected values |
| **Pattern segmentation** | That `extract_patterns()` correctly identifies 2/3/4 note patterns from real beatmaps | Manual verification on selected maps |
| **Angle distribution** | That the 15° bucket distribution differs meaningfully between tech and flow maps | AngelMaker vs YOASOBI (prototype test maps) |
| **Forward density** | That 1000ms window produces different counts than the existing visual density | Existing `density.rs` output on the same maps |
| **Spacing consistency** | That stddev of spacing per pattern type is higher on erratic maps than on consistent ones | Known consistent vs erratic maps |

### Prior art
- The existing jumps analysis (`analysis/jumps.rs`) reports spacing distributions with buckets (narrow/moderate/wide/extreme) — this PRD follows the same pattern but applies it per-pattern-type.
- The existing stream analysis (`analysis/streams.rs`) reports spacing profiles on consecutive circles — the spacing-per-pattern logic here mirrors that approach.
- The prototyped angle computation (`proto_angle.rs`) already has unit tests for straight line, 30° zigzag, 90° square, 135° sharp, and 180° reversal — these serve as the test foundation.

---

## Out of Scope

This PRD explicitly does NOT cover:

| Item | Rationale |
|---|---|
| **5+ note patterns** | Scope discipline — extend only after 2/3/4 patterns proven useful |
| **Inter-pattern / transition analysis** | Deferred to next PRD (transition notes treated as outliers here) |
| **BPM modulation of spacing** | Not an objective metric — subjective perception of mismatch |
| **Frontend UI updates** | Separate effort, after backend is stable |
| **Timeline window granularity** | Current 5-second windows noted as coarse; fix deferred |
| **Existing derived metrics cleanup** | Known tension (entropy, effective_objects, strain, is_spaghetti) — to be addressed separately |
| **Hidden mod / Hard Rock analysis** | NM only — no mod-specific data |
| **TUI binaries or side tools** | Prototype tooling is abandoned; only findings are carried forward |
| **Difficulty weightage or PP calculation** | Against project philosophy — raw data only |

---

## Further Notes

### Prototype findings carried forward
The angle distribution prototype (built during the exploration phase) produced these validated findings:
- **15° buckets** are the right granularity — not too coarse (hiding structure), not too fine (adding noise).
- **Mean/median/stddev** are essential companion stats — the mean-median gap reveals whether sharp turns are rare-but-extreme vs evenly distributed.
- **Raw angles ≠ trajectory entropy** — they measure orthogonal things (turn sharpness vs. pattern predictability). Both are needed in the reading analysis.

### Inter-pattern angle — two approaches documented for future
When the next PRD tackles inter-pattern analysis, two approaches were identified for angles at pattern boundaries:
1. Take the **last note of the previous pattern** as angle context.
2. Take the **first note of the next pattern** as angle context.
Both need prototyping to determine which better captures reading-relevant geometry at boundaries.

### Relationship to existing metrics
- **Forward density (new)** co-exists with **visual density (existing)** — they measure different windows (future 1000ms vs. currently-visible).
- **Intra-pattern angles (new)** co-exists with **trajectory entropy (existing)** — they measure different things (turn sharpness vs. pattern predictability).
- **Intra-pattern spacing (new)** complements **jump spacing (existing)** — it adds per-pattern-type breakdown.

### Snapping limitations
The existing pattern detection (`finger_control::patterns.rs`) uses a simple temporal gap threshold (half-beat + 10ms). This can misclassify patterns on maps with variable BPM or unusual rhythms — the `snap` field may show "Unstable" for such cases. This is an accepted limitation for the current scope.
