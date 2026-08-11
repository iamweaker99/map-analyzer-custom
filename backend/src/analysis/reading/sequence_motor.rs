/// PROTOTYPE — Sequence-Based Motor Descriptors
///
/// Three orthogonal, sequence-length-independent descriptors computed
/// per-pattern (burst/stream window):
///
///   - Motor Plan Adjustment (MPA):  Geometric instability via second differences
///   - Movement Magnitude (MM):      RMS spacing (cursor movement size)
///   - Spacing Consistency (SC):     Coefficient of variation of spacing
///
/// Spacing is normalized by circle diameter so geometrically identical
/// patterns on different CS values produce the same scores.
///
/// Portability: public functions have no I/O and depend only on VisualNode.
use super::visuals::VisualNode;

// ── Data Types ──────────────────────────────────────────────────────────────

/// Metrics for a single pattern window.
#[derive(Debug, Clone)]
pub struct PerPatternMetrics {
    /// Start time of the first note in the pattern (ms)
    pub time_ms: f64,
    /// Number of notes in the pattern
    pub note_count: usize,
    /// Motor Plan Adjustment — geometric instability (0.0 if < 4 notes)
    pub mpa: f64,
    /// Movement Magnitude — RMS spacing in diameters (0.0 if < 2 notes)
    pub mm: f64,
    /// Spacing Consistency — CV of spacing, lower = more uniform (0.0 if < 3 notes)
    pub sc: f64,
}

/// Summary statistics for a metric across all patterns on the beatmap.
#[derive(Debug, Clone)]
pub struct MetricSummary {
    pub mean: f64,
    pub max: f64,
    pub p95: f64,
}

/// Full beatmap-level output.
#[derive(Debug, Clone)]
pub struct SequenceMotorOutput {
    /// Per-pattern metrics in time order (only patterns with ≥2 notes)
    pub timeline: Vec<PerPatternMetrics>,
    pub summary_mpa: MetricSummary,
    pub summary_mm: MetricSummary,
    pub summary_sc: MetricSummary,
}

// ── Metric Computation ──────────────────────────────────────────────────────

/// Compute all three metrics for a single pattern window.
///
/// `nodes` should be the visual nodes belonging to one pattern (≥2).
/// `diameter` is the circle diameter for spacing normalization.
pub fn compute_metrics(nodes: &[VisualNode], diameter: f64) -> PerPatternMetrics {
    let n = nodes.len();
    let time_ms = nodes.first().map_or(0.0, |n| n.start_time);

    if n < 2 {
        return PerPatternMetrics {
            time_ms,
            note_count: n,
            mpa: 0.0,
            mm: 0.0,
            sc: 0.0,
        };
    }

    // Spacings: Euclidean distances between consecutive notes, in diameters
    let spacings: Vec<f64> = nodes
        .windows(2)
        .map(|w| {
            let dx = w[1].x - w[0].x;
            let dy = w[1].y - w[0].y;
            (dx * dx + dy * dy).sqrt() / diameter
        })
        .collect();

    let k = spacings.len(); // = n - 1

    // ── Motor Plan Adjustment (second differences) ──
    let mpa = if k >= 3 {
        // First differences
        let diffs1: Vec<f64> = spacings.windows(2).map(|w| w[1] - w[0]).collect();
        // Second differences
        let diffs2: Vec<f64> = diffs1.windows(2).map(|w| w[1] - w[0]).collect();
        // Mean absolute second difference
        let sum_abs: f64 = diffs2.iter().map(|d| d.abs()).sum();
        sum_abs / diffs2.len() as f64
    } else {
        0.0
    };

    // ── Movement Magnitude (RMS spacing) ──
    let sum_sq: f64 = spacings.iter().map(|s| s * s).sum();
    let mm = (sum_sq / k as f64).sqrt();

    // ── Spacing Consistency (CV = std / mean) ──
    let sc = if k >= 2 {
        let mean_s = spacings.iter().sum::<f64>() / k as f64;
        if mean_s > 0.0 {
            let variance = spacings.iter().map(|s| (s - mean_s).powi(2)).sum::<f64>() / k as f64;
            variance.sqrt() / mean_s
        } else {
            0.0 // all spacings are zero
        }
    } else {
        // k == 1 → single spacing → std = 0
        0.0
    };

    PerPatternMetrics {
        time_ms,
        note_count: n,
        mpa,
        mm,
        sc,
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn sorted_copy(values: &[f64]) -> Vec<f64> {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p / 100.0).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Compute summary statistics for a slice of metric values.
fn summarize(values: &[f64]) -> MetricSummary {
    if values.is_empty() {
        return MetricSummary {
            mean: 0.0,
            max: 0.0,
            p95: 0.0,
        };
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let max = values
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);
    let sorted = sorted_copy(values);
    let p95 = percentile(&sorted, 95.0);
    MetricSummary { mean, max, p95 }
}

/// Convenience: collect metric values for one field across all timeline entries.
fn collect_mpa(timeline: &[PerPatternMetrics]) -> Vec<f64> {
    timeline.iter().map(|e| e.mpa).collect()
}
fn collect_mm(timeline: &[PerPatternMetrics]) -> Vec<f64> {
    timeline.iter().map(|e| e.mm).collect()
}
fn collect_sc(timeline: &[PerPatternMetrics]) -> Vec<f64> {
    timeline.iter().map(|e| e.sc).collect()
}

/// Run the full pipeline on a slice of visual nodes and a list of pattern
/// index ranges (from `finger_control::rhythm_segmentation::extract_pattern_indices`).
///
/// Returns only patterns with ≥2 notes in the timeline, plus summary stats
/// across all patterns.
pub fn analyze_patterns(
    nodes: &[VisualNode],
    pattern_ranges: &[(std::ops::Range<usize>, &str)],
    diameter: f64,
) -> SequenceMotorOutput {
    let mut timeline = Vec::new();

    for (range, _label) in pattern_ranges {
        let start = range.start.min(nodes.len());
        let end = range.end.min(nodes.len());
        if end <= start || end - start < 2 {
            continue; // skip singletons
        }
        let slice = &nodes[start..end];
        let metrics = compute_metrics(slice, diameter);
        timeline.push(metrics);
    }

    // Sort by time (should already be in order, but defensive)
    timeline.sort_by(|a, b| a.time_ms.partial_cmp(&b.time_ms).unwrap_or(std::cmp::Ordering::Equal));

    let summary_mpa = summarize(&collect_mpa(&timeline));
    let summary_mm = summarize(&collect_mm(&timeline));
    let summary_sc = summarize(&collect_sc(&timeline));

    SequenceMotorOutput {
        timeline,
        summary_mpa,
        summary_mm,
        summary_sc,
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn node(time: f64, x: f64, y: f64) -> VisualNode {
        VisualNode {
            start_time: time,
            end_time: time,
            fade_in_time: time - 1000.0,
            x,
            y,
            is_slider: false,
        }
    }

    #[test]
    fn two_notes_constant_spacing() {
        // Two notes at (0,0) and (100,0), diameter=100 → spacing = 1.0
        let nodes = vec![node(0.0, 0.0, 0.0), node(100.0, 100.0, 0.0)];
        let m = compute_metrics(&nodes, 100.0);
        assert_eq!(m.note_count, 2);
        assert_eq!(m.mpa, 0.0);
        assert!((m.mm - 1.0).abs() < 1e-9);
        assert_eq!(m.sc, 0.0);
    }

    #[test]
    fn three_notes_linear() {
        // Three notes equally spaced in a line, diameter=50
        let nodes = vec![
            node(0.0, 0.0, 0.0),
            node(100.0, 50.0, 0.0),
            node(200.0, 100.0, 0.0),
        ];
        let m = compute_metrics(&nodes, 50.0);
        assert_eq!(m.note_count, 3);
        assert_eq!(m.mpa, 0.0); // < 4 notes
        assert!((m.mm - 1.0).abs() < 1e-9); // 50/50 = 1.0
        assert_eq!(m.sc, 0.0); // perfectly uniform
    }

    #[test]
    fn four_notes_square() {
        // 4 notes forming a square with side 50, diameter=50 → spacing = 1.0 each
        let nodes = vec![
            node(0.0, 0.0, 0.0),
            node(100.0, 50.0, 0.0),
            node(200.0, 50.0, 50.0),
            node(300.0, 0.0, 50.0),
        ];
        // spacings: 50/50, 50/50, sqrt(50^2+0^2)/50
        // = 1.0, 1.0, 1.0
        let m = compute_metrics(&nodes, 50.0);
        assert_eq!(m.note_count, 4);
        assert_eq!(m.mpa, 0.0); // uniform spacing → all Δ² = 0
        assert!((m.mm - 1.0).abs() < 1e-9);
        assert_eq!(m.sc, 0.0);
    }

    #[test]
    fn mpa_detects_instability() {
        // 5 notes with oscillating spacing: 1.0, 3.0, 1.0, 3.0 diameters
        let nodes = vec![
            node(0.0, 0.0, 0.0),
            node(100.0, 100.0, 0.0),
            node(200.0, 400.0, 0.0),
            node(300.0, 500.0, 0.0),
            node(400.0, 800.0, 0.0),
        ];
        let m = compute_metrics(&nodes, 100.0);
        assert_eq!(m.note_count, 5);
        assert!(m.mpa > 0.0, "Oscillating spacing should give non-zero MPA");
    }

    #[test]
    fn all_metrics_zero_for_empty_pattern() {
        let nodes: Vec<VisualNode> = vec![];
        let m = compute_metrics(&nodes, 100.0);
        assert_eq!(m.mpa, 0.0);
        assert_eq!(m.mm, 0.0);
        assert_eq!(m.sc, 0.0);
    }

    #[test]
    fn analyze_patterns_skips_singletons() {
        let nodes = vec![
            node(0.0, 0.0, 0.0),
            node(100.0, 100.0, 0.0),
            node(200.0, 200.0, 0.0),
            node(300.0, 300.0, 0.0),
        ];
        // ranges: singleton [0..1), pair [1..3), singleton [3..4)
        let ranges = vec![(0..1, "Jump"), (1..3, "2n Burst"), (3..4, "Jump")];
        let output = analyze_patterns(&nodes, &ranges, 100.0);
        assert_eq!(output.timeline.len(), 1); // only the pair survives
        assert_eq!(output.timeline[0].note_count, 2);
    }
}
