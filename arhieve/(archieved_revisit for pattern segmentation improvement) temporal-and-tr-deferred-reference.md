# Temporal Discontinuity and T−R: Deferred Reference

> **Status:** Archived after deciding Rhythm discontinuity as the primary
> segmentation signal. Retained for future improvement sessions.

---

## 1. What They Are

Both signals are computed per adjacent note triple (A→B→C), emitted at the
middle note B.

**Temporal Discontinuity (T):**
```
T = log₂(Δt₂ / Δt₁)

Δt₁ = t(B) − t(A)     [ms]
Δt₂ = t(C) − t(B)     [ms]
```

A pure measure of "how does the inter-note interval change" — no BPM
information involved.

**BPM Differential (T−R):**
```
T − R = log₂(beat_len₂ / beat_len₁)
```

Where `beat_len₁` is the active beat_len for pair A→B and `beat_len₂` is
the active beat_len for pair B→C (from the timing point at each pair's
second note).

This simplifies to a pure BPM-change indicator: zero unless a timing point
boundary falls between notes B and C of the triple.

---

## 2. Relationship to Rhythm Discontinuity (R)

```
R = log₂(snap₂ / snap₁)
  = log₂((Δt₂/beat_len₂) / (Δt₁/beat_len₁))
  = log₂(Δt₂/Δt₁) + log₂(beat_len₁/beat_len₂)
  = T − (T − R)
  = T − log₂(beat_len₂/beat_len₁)
```

| Condition | T | R | T−R |
|-----------|---|---|-----|
| Same TP, same snap | 0 | 0 | 0 |
| Same TP, snap changes | ≠0 | ≠0 | 0 |
| BPM change between pairs | **0** | **≠0** | **≠0** |
| BPM change at boundary note | ≠0 | ≠0 | ~0 |

The key divergence: T−R ≠ 0 **only** when consecutive pairs use different
timing points. This happens when a BPM change falls between notes B and C
of the triple.

---

## 3. Potential Future Uses

### 3.1 Boundary Type Classification

The T−R signal can classify what KIND of boundary the rhythm discontinuity
detected:

```
if |R| > threshold and |T−R| ≈ 0:
    → "pure snap-change boundary" (pattern truly changed)

if |R| > threshold and |T−R| > bpm_threshold:
    → "BPM-section boundary with snap change"

if |R| ≈ 0 and |T−R| > bpm_threshold:
    → "BPM change only — the pattern continued at a different tempo"
```

This classification could feed into difficulty estimation:
- A boundary with BPM-change + snap-change is harder than either alone
- A BPM-only boundary (no snap change) might not be a boundary at all

### 3.2 Timing Point Validation

T−R can be used to validate timing point placement:

```
For each timing point boundary:
    Compute T−R across it
    If T−R ≈ expected_bpm_ratio → timing is correct
    If T−R deviates → possible timing point misalignment
```

### 3.3 Hybrid Segmentation (If R Needs a Fallback)

If timing points are unreliable or missing:

```
if timing_points available:
    use R for primary segmentation
else:
    fall back to T (works without BPM data)
```

T is the more portable signal (no dependencies). R is the more precise one
(when timing points are correct).

### 3.4 Decomposition for Downstream Models

The three-signal decomposition (T, R, T−R) gives independent components:

- **R** → what the player feels as rhythm change
- **T−R** → what the BPM infrastructure does
- **T** → the confounded sum (rarely useful alone, but the residual check)

This is analogous to separating "what changed" from "why it changed."

---

## 4. Implementation Reference

**Temporal module:** `backend/src/analysis/reading/discontinuity_temporal.rs`

```rust
pub fn compute_signal(times_ms: &[f64]) -> Vec<DiscontinuityPoint>
// extras: [dt1, dt2, ratio]
// value: log2_ratio
```

**T−R in practice:** At any triple, it's simply:
```rust
let t_minus_r = temporal_pt.value - rhythm_pt.value;
// Equivalently: log2(rhythm_pt.extras[2] / temporal_pt.extras[0])
// Where extras[2] is beat_len_1 and extras[0] is dt1... 
// Actually: T−R = log2(beat_len_used_by_pair2 / beat_len_used_by_pair1)
```

The cleanest way to compute T−R in isolation:
```rust
fn bpm_discontinuity(timing_points: &[TimingPoint], times: &[f64], idx: usize) -> f64 {
    // For triple at index idx (middle note = times[idx+1])
    let beat_len_1 = timing_point_at(timing_points, times[idx+1])
        .map_or(500.0, |tp| tp.beat_len);
    let beat_len_2 = timing_point_at(timing_points, times[idx+2])
        .map_or(500.0, |tp| tp.beat_len);
    (beat_len_2 / beat_len_1).log2()
}
```

---

## 5. When to Revisit

Consider revisiting this reference when:

1. **Multi-BPM maps** need special handling — T−R can isolate BPM transitions
2. **Inherited timing points** (green lines) affect pattern grouping
3. **Timing point validation** becomes a feature
4. **Difficulty model** needs separation of "rhythm complexity" from "tempo complexity"
5. **Temporal-only analysis** is needed for maps without timing points

---

*Archived 2026-07-21. See Temp/temporal-vs-rhythm-illustrated.md for the
worked examples that led to the Rhythm-primary decision.*
