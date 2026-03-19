use serde::Serialize;
use crate::analysis::aim_control::spatial::AimVector;
use crate::analysis::finger_control::patterns::{Pattern, PatternType};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BurstAimDistribution {
    pub low: f64,    // Stacked / Constant / Flat
    pub mid: f64,    // Flow / Adaptive / Accented
    pub high: f64,   // Jump / Erratic / Kick
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BurstAimAnalysis {
    pub avg_spacing: BurstAimDistribution,
    pub variance: BurstAimDistribution,
    pub spikes: BurstAimDistribution,
}

/// Helper to calculate the median of a small slice
fn calculate_median(data: &mut [f64]) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        (data[mid - 1] + data[mid]) / 2.0
    } else {
        data[mid]
    }
}

/// Processes patterns identified by finger_control and extracts burst-aim metrics.
pub fn analyze_burst_aim(patterns: &[Pattern], aim_vectors: &[AimVector]) -> BurstAimAnalysis {
    let mut avg_counts = [0; 3];
    let mut var_counts = [0; 3];
    let mut spike_counts = [0; 3];
    let mut total_bursts = 0;

    for pattern in patterns {
        // Only process bursts between 2 and 6 notes
        if let PatternType::Burst(n) = pattern.p_type {
            if n < 2 || n > 6 { continue; }

            // Your Pattern struct uses 'time'. 
            // We find the vectors that start at or after the burst's start time.
            // Since it's an N-note burst, it contains N-1 aim vectors.
            let burst_vectors: Vec<&AimVector> = aim_vectors.iter()
                .filter(|v| v.start_time >= pattern.time)
                .take((n - 1) as usize) // A 3-note burst has 2 movement vectors
                .collect();

            if burst_vectors.is_empty() { continue; }

            let distances: Vec<f64> = burst_vectors.iter().map(|v| v.norm_distance).collect();
            let count = distances.len() as f64;
            let mean: f64 = distances.iter().sum::<f64>() / count;
            let variance: f64 = (distances.iter().map(|d| (mean - d).powi(2)).sum::<f64>() / count).sqrt();
            
            // Spike detection (Max / Median)
            let mut dist_copy = distances.clone();
            let median = calculate_median(&mut dist_copy);
            let max = distances.iter().cloned().fold(0.0, f64::max);
            let spike_ratio = if median > 0.0 { max / median } else { 1.0 };

            // 1. Categorize Avg Spacing
            if mean < 0.5 { avg_counts[0] += 1; }
            else if mean <= 2.0 { avg_counts[1] += 1; }
            else { avg_counts[2] += 1; }

            // 2. Categorize Variance
            if variance < 0.1 { var_counts[0] += 1; }
            else if variance <= 0.4 { var_counts[1] += 1; }
            else { var_counts[2] += 1; }

            // 3. Categorize Spikes
            if spike_ratio < 1.2 { spike_counts[0] += 1; }
            else if spike_ratio <= 2.0 { spike_counts[1] += 1; }
            else { spike_counts[2] += 1; }

            total_bursts += 1;
        }
    }

    let to_dist = |counts: [usize; 3]| -> BurstAimDistribution {
        let t = total_bursts as f64;
        if t == 0.0 { return BurstAimDistribution { low: 0.0, mid: 0.0, high: 0.0 }; }
        BurstAimDistribution {
            low: (counts[0] as f64 / t) * 100.0,
            mid: (counts[1] as f64 / t) * 100.0,
            high: (counts[2] as f64 / t) * 100.0,
        }
    };

    BurstAimAnalysis {
        avg_spacing: to_dist(avg_counts),
        variance: to_dist(var_counts),
        spikes: to_dist(spike_counts),
    }
}