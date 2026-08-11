use rosu_pp::Beatmap;
use rosu_pp::model::control_point::TimingPoint;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OffGridNote {
    pub time: f64,
    pub delta: f64,
}

/// Snap definitions: (fraction_of_beat, label)
const SNAPS: &[(f64, &str)] = &[
    (1.0, "1/1"),
    (0.5, "1/2"),
    (0.3333, "1/3"),
    (0.25, "1/4"),
    (0.1666, "1/6"),
    (0.125, "1/8"),
];

/// Adaptive tolerance fraction (10% of beat_len × fraction)
const TOLERANCE: f64 = 0.10;

// ── Timing Point Lookup ─────────────────────────────────────────────────────

/// Find the active [`TimingPoint`] at a given timestamp (binary search).
pub fn timing_point_at(points: &[TimingPoint], time: f64) -> Option<&TimingPoint> {
    let i: Result<usize, usize> = points
        .binary_search_by(|probe| probe.time.total_cmp(&time));
    let i = i.unwrap_or_else(|i| i.saturating_sub(1));
    points.get(i)
}

// ── Snap Identification ─────────────────────────────────────────────────────

/// Identify the snap label for a time delta given a beat_len, using
/// adaptive ±10% tolerance.
pub fn identify_snap(delta: f64, beat_len: f64) -> Option<String> {
    for (fraction, label) in SNAPS {
        let target = beat_len * fraction;
        let tolerance = target * TOLERANCE;
        if (delta - target).abs() <= tolerance {
            return Some(label.to_string());
        }
    }
    None
}

// ── Per-Pair Snap Export (for rhythm segmentation) ─────────────────────────

/// For every adjacent note pair, return the snap fraction (delta / beat_len)
/// and the active beat_len, using per-timing-point BPM lookup.
///
/// Returns `Vec<(time_of_second_note, snap_value, beat_len)>`.
/// The snap_value is a continuous fraction (not quantized to labels).
pub fn note_pair_snaps(
    times: &[f64],
    timing_points: &[TimingPoint],
) -> Vec<(f64, f64, f64)> {
    if times.len() < 2 {
        return Vec::new();
    }

    let mut results = Vec::with_capacity(times.len() - 1);
    for window in times.windows(2) {
        let delta = window[1] - window[0];
        let tp = timing_point_at(timing_points, window[1])
            .copied()
            .unwrap_or(TimingPoint::new(0.0, 500.0)); // fallback 120 BPM
        let snap_value = delta / tp.beat_len;
        results.push((window[1], snap_value, tp.beat_len));
    }

    results
}

// ── Foundation Analysis (finger_control pipeline) ──────────────────────────

pub fn analyze_foundation(map: &Beatmap) -> (HashMap<String, u32>, HashMap<u32, u32>, Vec<OffGridNote>, [u32; 10]) {
    let mut snap_counts = HashMap::new();
    let burst_histogram = HashMap::new();
    let mut off_grid_notes = Vec::new();
    let mut buckets = [0u32; 10];

    if map.hit_objects.is_empty() {
        return (snap_counts, burst_histogram, off_grid_notes, buckets);
    }

    let start_time = map.hit_objects.first().unwrap().start_time;
    let end_time = map.hit_objects.last().unwrap().start_time;
    let total_duration = (end_time - start_time).max(1.0);

    // Snap & Off-grid Logic (uses per-timing-point BPM)
    for window in map.hit_objects.windows(2) {
        let delta = window[1].start_time - window[0].start_time;
        let tp = timing_point_at(&map.timing_points, window[1].start_time);
        let beat_len = tp.map_or(500.0, |t| t.beat_len);

        if let Some(label) = identify_snap(delta, beat_len) {
            *snap_counts.entry(label).or_insert(0) += 1;
        } else {
            let note_time = window[1].start_time;
            off_grid_notes.push(OffGridNote { time: note_time, delta });

            // "Snap" to one of the 10 sections
            let relative_pos = (note_time - start_time) / total_duration;
            let bucket_idx = (relative_pos * 10.0).floor() as usize;
            if bucket_idx < 10 {
                buckets[bucket_idx] += 1;
            } else {
                buckets[9] += 1;
            }
        }
    }

    (snap_counts, burst_histogram, off_grid_notes, buckets)
}
