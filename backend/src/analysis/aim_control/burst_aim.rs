use serde::Serialize;
use crate::analysis::aim_control::spatial::AimVector;
use crate::analysis::finger_control::patterns::{Pattern, PatternType};
use crate::analysis::aim_control::buckets::{get_alignment_bucket, AlignmentBucket};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BurstAimDistribution {
    pub low: f64,    // Constant / Stable
    pub mid: f64,    // Adaptive / Transitionary
    pub high: f64,   // Erratic / Chaotic
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BurstChirpBias {
    pub no_chirp_bursts: f64,
    pub chirp_bursts: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BurstAimAnalysis {
    pub avg_spacing: BurstAimDistribution,
    pub variance: BurstAimDistribution,
    pub spikes: BurstAimDistribution,
    pub angle_variance: BurstAimDistribution,     // NEW
    pub alignment_stability: BurstAimDistribution, // NEW
    pub chirp_bias: BurstChirpBias,                // NEW
}

fn calculate_median(data: &mut [f64]) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    if data.len() % 2 == 0 { (data[mid - 1] + data[mid]) / 2.0 } else { data[mid] }
}

pub fn analyze_burst_aim(patterns: &[Pattern], aim_vectors: &[AimVector]) -> BurstAimAnalysis {
    let mut avg_counts = [0; 3];
    let mut var_counts = [0; 3];
    let mut spike_counts = [0; 3];
    let mut ang_var_counts = [0; 3];
    let mut align_stab_counts = [0; 3];
    let mut chirp_burst_count = 0;
    let mut total_bursts = 0;

    for pattern in patterns {
        if let PatternType::Burst(n) = pattern.p_type {
            if n < 2 || n > 6 { continue; }

            let burst_vectors: Vec<&AimVector> = aim_vectors.iter()
                .filter(|v| v.start_time >= pattern.time)
                .take((n - 1) as usize)
                .collect();

            if burst_vectors.is_empty() { continue; }

            // --- 1. Spacing Calculations ---
            let distances: Vec<f64> = burst_vectors.iter().map(|v| v.norm_distance).collect();
            let count = distances.len() as f64;
            let mean: f64 = distances.iter().sum::<f64>() / count;
            let variance: f64 = (distances.iter().map(|d| (mean - d).powi(2)).sum::<f64>() / count).sqrt();
            let mut dist_copy = distances.clone();
            let spike_ratio = if calculate_median(&mut dist_copy) > 0.0 { distances.iter().cloned().fold(0.0, f64::max) / calculate_median(&mut dist_copy) } else { 1.0 };

            // --- 2. Angle Variance ---
            let angles: Vec<f64> = burst_vectors.iter().filter_map(|v| v.deflection_angle).collect();
            let ang_var = if angles.len() > 1 {
                let ang_mean = angles.iter().sum::<f64>() / angles.len() as f64;
                (angles.iter().map(|a| (ang_mean - a).powi(2)).sum::<f64>() / angles.len() as f64).sqrt()
            } else { 0.0 };

            // --- 3. Alignment Stability ---
            let align_buckets: Vec<AlignmentBucket> = burst_vectors.iter().map(|v| get_alignment_bucket(v.deflection_angle)).collect();
            let mut align_switches = 0;
            for i in 0..align_buckets.len().saturating_sub(1) {
                if align_buckets[i] != align_buckets[i+1] && align_buckets[i] != AlignmentBucket::None && align_buckets[i+1] != AlignmentBucket::None {
                    align_switches += 1;
                }
            }

            // --- 4. Chirp Detection (Rotation Flip) ---
            let mut has_chirp = false;
            let cross_products: Vec<f64> = burst_vectors.windows(2).map(|v| v[0].dx * v[1].dy - v[0].dy * v[1].dx).collect();
            for i in 0..cross_products.len().saturating_sub(1) {
                if cross_products[i] * cross_products[i+1] < -1e-5 { // Sign change detected
                    has_chirp = true;
                    break;
                }
            }

            // --- CATEGORIZATION ---
            // Spacing
            if mean < 0.5 { avg_counts[0] += 1; } else if mean <= 2.0 { avg_counts[1] += 1; } else { avg_counts[2] += 1; }
            if variance < 0.1 { var_counts[0] += 1; } else if variance <= 0.4 { var_counts[1] += 1; } else { var_counts[2] += 1; }
            if spike_ratio < 1.2 { spike_counts[0] += 1; } else if spike_ratio <= 2.0 { spike_counts[1] += 1; } else { spike_counts[2] += 1; }

            // Angle Variance (Constant < 5, Adaptive < 15)
            if ang_var < 5.0 { ang_var_counts[0] += 1; } else if ang_var <= 15.0 { ang_var_counts[1] += 1; } else { ang_var_counts[2] += 1; }

            // Alignment Stability (Stable: 0, Transitionary: 1, Chaotic: 2+)
            if align_switches == 0 { align_stab_counts[0] += 1; } else if align_switches == 1 { align_stab_counts[1] += 1; } else { align_stab_counts[2] += 1; }

            if has_chirp { chirp_burst_count += 1; }
            total_bursts += 1;
        }
    }

    let to_dist = |counts: [usize; 3]| -> BurstAimDistribution {
        let t = total_bursts as f64;
        if t == 0.0 { return BurstAimDistribution { low: 0.0, mid: 0.0, high: 0.0 }; }
        BurstAimDistribution { low: (counts[0] as f64 / t) * 100.0, mid: (counts[1] as f64 / t) * 100.0, high: (counts[2] as f64 / t) * 100.0 }
    };

    BurstAimAnalysis {
        avg_spacing: to_dist(avg_counts),
        variance: to_dist(var_counts),
        spikes: to_dist(spike_counts),
        angle_variance: to_dist(ang_var_counts),
        alignment_stability: to_dist(align_stab_counts),
        chirp_bias: BurstChirpBias {
            no_chirp_bursts: if total_bursts > 0 { ((total_bursts - chirp_burst_count) as f64 / total_bursts as f64) * 100.0 } else { 0.0 },
            chirp_bursts: if total_bursts > 0 { (chirp_burst_count as f64 / total_bursts as f64) * 100.0 } else { 0.0 },
        },
    }
}