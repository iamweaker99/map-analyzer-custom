/// Rhythm-Based Pattern Segmentation
///
/// Segments beatmap hit objects into patterns using three signals:
///
///   1. **Gap threshold**: consecutive notes > ½ beat apart are separate patterns
///   2. **Rhythm discontinuity (R)**: |log₂(snap₂/snap₁)| > threshold → pattern
///      boundary at the middle note. R normalises for BPM changes by using
///      per-timing-point snap values.
///   3. **Asymmetric type-boundary rules** (2026-08-09/10, wiki
///      [[rhythm-segmentation]]; user-verified against the in-game editor):
///      slider→circle change always splits; circle→slider splits iff the
///      following slider run has ≥2 sliders (a lone near slider engulfs into
///      the preceding circle group); spinner/hold adjacency always splits;
///      pure slider runs ≥2 classify as `SliderChain`. The R/T discontinuity
///      signal is skipped at slider→circle transition windows (its boundary
///      would land one note inside the circle run — the type rule already
///      bounds the run at the transition).
///
/// This replaces the old `patterns::extract_patterns` approach (Path A,
/// retired per [[segmentation-unification]]) that used a fixed gap threshold
/// with global BPM and was type-blind.
///
/// Portability: depends only on `Beatmap` and `snap_filter` — no circular
/// dependency with `reading`.

use rosu_pp::Beatmap;
use rosu_pp::model::control_point::TimingPoint;
use rosu_pp::model::hit_object::HitObject;
use serde::Serialize;
use std::ops::Range;

use super::snap_filter;

/// Rhythm discontinuity threshold. |R| above this → pattern boundary.
///
/// 0.35 (2026-08-11, [[experiment-protocol]] #1): lowered 0.5 → 0.35 to catch
/// 1/4→1/3 (R = 0.415) while staying above jitter noise ~0.29.
/// Previously 0.5 ≈ a snap ratio change of 1.4×.
const R_THRESHOLD: f64 = 0.35;

/// Temporal discontinuity threshold. |T| above this → pattern boundary.
///
/// Same numeric value as R_THRESHOLD for direct comparison (T and R are
/// equivalent on single-BPM maps and diverge only at BPM transitions).
const T_THRESHOLD: f64 = 0.35;

/// Engulf proximity = 2× the circle diameter — one "note's worth" of space.
/// (2026-08-10: 25px was too strict — trailing slider heads measured 38.9–
/// 116px in vs ≥185px out.)
const ENGULF_DIAMETERS: f64 = 2.0;

// ── Pattern Types ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PatternType {
    Jump,
    Slider,
    Burst(u32),      // 2 to 6 notes
    Stream,          // 7+ notes
    SliderChain,     // pure slider run ≥2 (exp rule, 2026-08-09)
}

impl PatternType {
    pub fn as_str(&self) -> String {
        match self {
            Self::Jump => "Jump".to_string(),
            Self::Slider => "Slider".to_string(),
            Self::Burst(n) => format!("{}n Burst", n),
            Self::Stream => "Stream".to_string(),
            Self::SliderChain => "SliderChain".to_string(),
        }
    }

    pub fn note_count(&self) -> u32 {
        match self {
            Self::Jump | Self::Slider => 1,
            Self::Burst(n) => *n,
            // Long-run approximation, same convention as Stream (7+ notes).
            Self::Stream | Self::SliderChain => 7,
        }
    }

    pub fn is_odd(&self) -> bool {
        match self {
            Self::Burst(n) => n % 2 != 0,
            _ => false,
        }
    }

    pub fn is_even(&self) -> bool {
        match self {
            Self::Burst(n) => n % 2 == 0,
            _ => false,
        }
    }
}

pub struct Pattern {
    pub p_type: PatternType,
    pub time: f64,
    pub snap: String,
}

// ── Type-Boundary Rules (exp, ported from prototype 2026-08-11) ─────────────

/// Object kind for the type-boundary rules (anything not circle/slider = Other).
#[derive(Clone, Copy, PartialEq, Debug)]
enum ObjKind {
    Circle,
    Slider,
    Other,
}

fn obj_kind(o: &HitObject) -> ObjKind {
    if o.is_circle() {
        ObjKind::Circle
    } else if o.is_slider() {
        ObjKind::Slider
    } else {
        ObjKind::Other
    }
}

/// Consecutive sliders starting at index `i`.
fn slider_run_from(kinds: &[ObjKind], i: usize) -> usize {
    let mut n = 0;
    while i + n < kinds.len() && kinds[i + n] == ObjKind::Slider {
        n += 1;
    }
    n
}

/// Euclidean distance between objects i and i+1.
fn obj_distance(a: &HitObject, b: &HitObject) -> f64 {
    let dx = (b.pos.x - a.pos.x) as f64;
    let dy = (b.pos.y - a.pos.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// Skip the discontinuity signal (R/T) when the window (k-1, k, k+1) straddles
/// a slider→circle type transition — there R's boundary would land one note
/// inside the circle run (974/792 cases), while the type rule already bounds
/// the run at the transition itself. Asymmetric: circle→slider windows keep
/// the signal (that's what isolates the transition slider from a following
/// chain, e.g. 964).
fn skip_discontinuity(kinds: &[ObjKind], k: usize) -> bool {
    (kinds[k - 1] == ObjKind::Slider && kinds[k] == ObjKind::Circle)
        || (kinds[k] == ObjKind::Slider && kinds[k + 1] == ObjKind::Circle)
}

/// Apply the asymmetric type-boundary rules on top of gap/discontinuity
/// boundaries. `dists[i]` = distance between objects i and i+1; `engulf_px` =
/// proximity within which a lone slider merges into the preceding circle group.
fn apply_type_rules(kinds: &[ObjKind], dists: &[f64], engulf_px: f64, is_boundary: &mut [bool]) {
    for i in 0..kinds.len().saturating_sub(1) {
        if kinds[i] == ObjKind::Other || kinds[i + 1] == ObjKind::Other {
            is_boundary[i + 1] = true; // never merge across a spinner/hold
            continue;
        }
        match (kinds[i], kinds[i + 1]) {
            (ObjKind::Slider, ObjKind::Circle) => is_boundary[i + 1] = true,
            (ObjKind::Circle, ObjKind::Slider) => {
                if slider_run_from(kinds, i + 1) >= 2 || dists[i] > engulf_px {
                    is_boundary[i + 1] = true;
                }
                // else: lone near slider engulfs into the circle group
            }
            _ => {} // same-kind runs stay continuous
        }
    }
}

/// Apply the type rules with map-derived inputs (2× circle diameter engulf).
fn apply_exp_type_rules(map: &Beatmap, kinds: &[ObjKind], is_boundary: &mut [bool]) {
    let n = kinds.len();
    let dists: Vec<f64> = (0..n.saturating_sub(1))
        .map(|i| obj_distance(&map.hit_objects[i], &map.hit_objects[i + 1]))
        .collect();
    // Engulf = 2 × circle diameter (CS-dependent): diameter = 108.8 − 8.96·CS
    let engulf_px = (54.4 - 4.48 * map.cs as f64) * 2.0 * ENGULF_DIAMETERS;
    apply_type_rules(kinds, &dists, engulf_px, is_boundary);
}

/// Classify a pattern by composition (not count only — fixes "slider runs
/// ≥7 → Stream"). The engulfed slider head counts as the nth note of the
/// Burst(n); no mixed label.
fn classify(kinds: &[ObjKind], range: &Range<usize>) -> PatternType {
    let all_sliders = kinds[range.clone()].iter().all(|k| *k == ObjKind::Slider);
    if range.len() >= 2 && all_sliders {
        PatternType::SliderChain
    } else if range.len() >= 7 {
        PatternType::Stream
    } else if range.len() >= 2 {
        PatternType::Burst(range.len() as u32)
    } else if kinds[range.start] == ObjKind::Slider {
        PatternType::Slider
    } else {
        PatternType::Jump
    }
}

// ── Shared Grouping Helper ───────────────────────────────────────────────────

/// Shared: group notes into patterns using a precomputed boundary array.
///
/// All boundary detection (gap + discontinuity signal + type rules) must be
/// done before calling this. It handles pattern-type classification
/// (composition-aware, incl. `SliderChain`) and snap-label assignment.
fn group_into_patterns(
    objects: &[HitObject],
    times: &[f64],
    timings: &[TimingPoint],
    kinds: &[ObjKind],
    is_boundary: &[bool],
) -> Vec<(Pattern, Range<usize>)> {
    let n = objects.len();
    let mut patterns = Vec::new();
    let mut start = 0;

    for i in 1..=n {
        if i == n || is_boundary[i] {
            let range = start..i;
            let len = range.len();

            if len > 0 {
                let p_type = classify(kinds, &range);

                // Snap label
                let snap = if len >= 2 {
                    let avg_delta = times[start..i]
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .sum::<f64>()
                        / (len - 1) as f64;
                    let beat_len = timings
                        .iter()
                        .rev()
                        .find(|tp| tp.time <= times[start])
                        .map_or(500.0, |tp| tp.beat_len);
                    snap_filter::identify_snap(avg_delta, beat_len)
                        .unwrap_or_else(|| "Unstable".to_string())
                } else if start + 1 < n {
                    let delta = times[start + 1] - times[start];
                    let beat_len = timings
                        .iter()
                        .rev()
                        .find(|tp| tp.time <= times[start + 1])
                        .map_or(500.0, |tp| tp.beat_len);
                    snap_filter::identify_snap(delta, beat_len)
                        .unwrap_or_else(|| "Unstable".to_string())
                } else {
                    "End".to_string()
                };

                patterns.push((
                    Pattern {
                        p_type,
                        time: objects[start].start_time,
                        snap,
                    },
                    range,
                ));
            }

            start = i;
        }
    }

    patterns
}

// ── R-Based Segmentation ────────────────────────────────────────────────────

/// Extract pattern indices using gap-threshold + rhythm-discontinuity (R) +
/// asymmetric type-boundary rules.
///
/// R = |log₂(snap₂ / snap₁)| — normalises for BPM changes via per-timing-point
/// snap values. The primary segmentation signal for the reading pipeline.
pub fn extract_pattern_indices(map: &Beatmap) -> Vec<(Pattern, Range<usize>)> {
    let objects = &map.hit_objects;
    let timings = &map.timing_points;
    let n = objects.len();

    if n == 0 {
        return Vec::new();
    }

    let times: Vec<f64> = objects.iter().map(|o| o.start_time).collect();
    let kinds: Vec<ObjKind> = objects.iter().map(obj_kind).collect();
    let snaps = snap_filter::note_pair_snaps(&times, timings);

    // ── Boundary Detection ─────────────────────────────────────────────────
    let mut is_boundary = vec![false; n];
    is_boundary[0] = true;

    // Gap threshold: ½ beat of the active timing point at the second note
    for i in 0..n.saturating_sub(1) {
        let gap = times[i + 1] - times[i];
        let beat_len = snap_filter::timing_point_at(timings, times[i + 1])
            .map_or(500.0, |tp| tp.beat_len);
        let gap_threshold = beat_len / 2.0 + 10.0;
        if gap > gap_threshold {
            is_boundary[i + 1] = true;
        }
    }

    // Rhythm discontinuity: R = |log₂(snap₂ / snap₁)| at middle note k —
    // skipped at slider→circle transition windows (2026-08-10: R boundary
    // lands one note inside the circle run there)
    if snaps.len() >= 2 {
        for k in 1..snaps.len() {
            if skip_discontinuity(&kinds, k) {
                continue;
            }
            let snap1 = snaps[k - 1].1;
            let snap2 = snaps[k].1;
            let r = if snap1 > 0.0 { (snap2 / snap1).log2().abs() } else { 0.0 };
            let r = if r.is_finite() { r } else { 0.0 };
            if r > R_THRESHOLD {
                let boundary_idx = k + 1;
                if boundary_idx < n {
                    is_boundary[boundary_idx] = true;
                }
            }
        }
    }

    // Asymmetric type-boundary rules
    apply_exp_type_rules(map, &kinds, &mut is_boundary);

    group_into_patterns(objects, &times, timings, &kinds, &is_boundary)
}

// ── T-Based Segmentation ────────────────────────────────────────────────────

/// Extract pattern indices using gap-threshold + temporal-discontinuity (T) +
/// asymmetric type-boundary rules.
///
/// T = |log₂(Δt₂ / Δt₁)| — a pure time-ratio discontinuity computed from
/// raw note timestamps, without any BPM/timing-point information.
///
/// Useful for comparison against R-based segmentation:
/// - On single-BPM maps T and R produce identical results.
/// - On multi-BPM maps T introduces false boundaries at BPM transitions
///   (where the timing point changes but the snapped rhythm hasn't).
pub fn extract_pattern_indices_temporal(map: &Beatmap) -> Vec<(Pattern, Range<usize>)> {
    let objects = &map.hit_objects;
    let timings = &map.timing_points;
    let n = objects.len();

    if n == 0 {
        return Vec::new();
    }

    let times: Vec<f64> = objects.iter().map(|o| o.start_time).collect();
    let kinds: Vec<ObjKind> = objects.iter().map(obj_kind).collect();

    // ── Boundary Detection ─────────────────────────────────────────────────
    let mut is_boundary = vec![false; n];
    is_boundary[0] = true;

    // Gap threshold (same as R-based)
    for i in 0..n.saturating_sub(1) {
        let gap = times[i + 1] - times[i];
        let beat_len = snap_filter::timing_point_at(timings, times[i + 1])
            .map_or(500.0, |tp| tp.beat_len);
        let gap_threshold = beat_len / 2.0 + 10.0;
        if gap > gap_threshold {
            is_boundary[i + 1] = true;
        }
    }

    // Temporal discontinuity: T = |log₂(Δt₂ / Δt₁)| at middle note k —
    // skipped at slider→circle transition windows (see `skip_discontinuity`)
    for k in 1..n.saturating_sub(1) {
        if skip_discontinuity(&kinds, k) {
            continue;
        }
        let dt1 = times[k] - times[k - 1];
        let dt2 = times[k + 1] - times[k];
        let t = if dt1 > 0.0 { (dt2 / dt1).log2().abs() } else { 0.0 };
        let t = if t.is_finite() { t } else { 0.0 };
        if t > T_THRESHOLD {
            let boundary_idx = k + 1;
            if boundary_idx < n {
                is_boundary[boundary_idx] = true;
            }
        }
    }

    // Asymmetric type-boundary rules
    apply_exp_type_rules(map, &kinds, &mut is_boundary);

    group_into_patterns(objects, &times, timings, &kinds, &is_boundary)
}

// ── Tests ────────────────────────────────────────────────────────────────────
// Ported from prototype_sequence_motor.rs (--exp rules, user-verified) on
// 2026-08-11. Run with: cargo test --bin backend  (package tests)

#[cfg(test)]
mod tests {
    use super::*;

    /// 'c' → circle, 's' → slider; any other char → Other.
    fn kinds_from(desc: &[char]) -> Vec<ObjKind> {
        desc.iter()
            .map(|c| match c {
                'c' => ObjKind::Circle,
                's' => ObjKind::Slider,
                _ => ObjKind::Other,
            })
            .collect()
    }

    /// Segment with only the type rules active (no gap/discontinuity signal),
    /// then group + classify. `dists[i]` = distance between objects i and i+1.
    fn segment(desc: &[char], dists: &[f64], engulf_px: f64) -> Vec<(String, Range<usize>)> {
        let kinds = kinds_from(desc);
        let mut is_boundary = vec![false; kinds.len()];
        is_boundary[0] = true;
        apply_type_rules(&kinds, dists, engulf_px, &mut is_boundary);
        group_and_classify(&kinds, &is_boundary)
    }

    /// Group by boundaries and classify (test helper — no snap labels needed).
    fn group_and_classify(kinds: &[ObjKind], is_boundary: &[bool]) -> Vec<(String, Range<usize>)> {
        let mut out = Vec::new();
        let mut start = 0;
        for i in 1..=kinds.len() {
            if i == kinds.len() || is_boundary[i] {
                let range = start..i;
                if !range.is_empty() {
                    out.push((classify(kinds, &range).as_str(), range));
                }
                start = i;
            }
        }
        out
    }

    #[test]
    fn pure_slider_chain_is_slider_chain() {
        // 8 consecutive sliders — must NOT be "Stream" (the original bug)
        let pats = segment(&['s'; 8], &[10.0; 7], 146.0);
        assert_eq!(pats, vec![("SliderChain".to_string(), 0..8)]);
    }

    #[test]
    fn lone_slider_engulfs_into_circle_group() {
        // c,c,c,s,c,c with the slider within proximity → [c,c,c,s] + [c,c]
        let pats = segment(&['c', 'c', 'c', 's', 'c', 'c'], &[10.0; 5], 146.0);
        assert_eq!(
            pats,
            vec![("4n Burst".to_string(), 0..4), ("2n Burst".to_string(), 4..6)]
        );
    }

    #[test]
    fn trailing_slider_engulfs_within_two_diameters() {
        // 2026-08-10: stream trailing sliders measure 38.9–116px — a 100px head
        // engulfs at 2× diameter (146px at CS4) but would not at the old 25px
        let pats = segment(&['c', 'c', 'c', 's', 'c', 'c'], &[10.0, 10.0, 100.0, 10.0, 10.0], 146.0);
        assert_eq!(
            pats,
            vec![("4n Burst".to_string(), 0..4), ("2n Burst".to_string(), 4..6)]
        );
    }

    #[test]
    fn far_slider_does_not_engulf() {
        // same shape, slider beyond proximity → its own "Slider" pattern
        let pats = segment(&['c', 'c', 'c', 's', 'c', 'c'], &[10.0, 10.0, 40.0, 10.0, 10.0], 25.0);
        assert_eq!(
            pats,
            vec![
                ("3n Burst".to_string(), 0..3),
                ("Slider".to_string(), 3..4),
                ("2n Burst".to_string(), 4..6),
            ]
        );
    }

    #[test]
    fn alternation_yields_doubles() {
        // c,s,c,s,c,s → three [c,s] doubles (no annihilation)
        let pats = segment(&['c', 's', 'c', 's', 'c', 's'], &[10.0; 5], 146.0);
        assert_eq!(
            pats,
            vec![
                ("2n Burst".to_string(), 0..2),
                ("2n Burst".to_string(), 2..4),
                ("2n Burst".to_string(), 4..6),
            ]
        );
    }

    #[test]
    fn slider_to_circle_always_splits() {
        // the 9s+1c case: 10 sliders + 8 circles split at the type change
        let desc: Vec<char> = vec!['s'; 10].into_iter().chain(vec!['c'; 8]).collect();
        let pats = segment(&desc, &[10.0; 17], 146.0);
        assert_eq!(
            pats,
            vec![
                ("SliderChain".to_string(), 0..10),
                ("Stream".to_string(), 10..18),
            ]
        );
    }

    #[test]
    fn slider_run_of_two_splits() {
        // c,c,s,s,c → [2n Burst][SliderChain][Jump]
        let pats = segment(&['c', 'c', 's', 's', 'c'], &[10.0; 4], 146.0);
        assert_eq!(
            pats,
            vec![
                ("2n Burst".to_string(), 0..2),
                ("SliderChain".to_string(), 2..4),
                ("Jump".to_string(), 4..5),
            ]
        );
    }

    // ── skip_discontinuity (R/T suppressed at slider→circle windows) ────────

    #[test]
    fn discontinuity_skipped_at_slider_to_circle_first_pair() {
        // s→c at the window's first pair — R would land a boundary one note
        // inside the circle run (the 974 / 792 cases)
        let kinds = kinds_from(&['s', 'c', 'c']);
        assert!(skip_discontinuity(&kinds, 1));
    }

    #[test]
    fn discontinuity_skipped_at_slider_to_circle_second_pair() {
        // s→c at the window's second pair — same one-note-in fragmentation
        let kinds = kinds_from(&['s', 's', 'c']);
        assert!(skip_discontinuity(&kinds, 1));
    }

    #[test]
    fn discontinuity_kept_at_circle_to_slider_window() {
        // c→s keeps the signal — isolates the transition slider (issue D: 964)
        let kinds = kinds_from(&['c', 'c', 's']);
        assert!(!skip_discontinuity(&kinds, 1));
    }

    #[test]
    fn discontinuity_kept_for_pure_circles() {
        // no type change in the window — R still fires (02:08:539 stays split)
        let kinds = kinds_from(&['c', 'c', 'c']);
        assert!(!skip_discontinuity(&kinds, 1));
    }
}
