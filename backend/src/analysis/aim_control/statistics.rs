use serde::Serialize;
use super::spatial::AimVector;
use super::volatility::{calculate_volatilities, WindowVolatility};
use super::buckets::{get_relative_velocity_bucket};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolatilityDistribution {
    pub switches_0: f64,
    pub switches_1_2: f64,
    pub switches_3_4: f64,
    pub switches_5_6: f64,
    pub switches_7: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VelocityIntensityDistribution {
    pub major_adjustment: f64,
    pub minor_adjustment: f64,
    pub steady: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapFlowRatio {
    pub snap_aim: f64,
    pub flow_aim: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureMatrix {
    pub consistent: f64,
    pub flow_tech: f64,
    pub rhythmic_tech: f64,
    pub chaotic_tech: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AimVolatilitySummary {
    pub velocity_buckets: VolatilityDistribution, 
    pub velocity_intensity: VelocityIntensityDistribution,
    pub angle: VolatilityDistribution,
    pub direction: VolatilityDistribution,
    pub texture_matrix: TextureMatrix,
    pub snap_flow: SnapFlowRatio,
}

fn to_distribution(counts: &[usize; 5], total_windows: f64) -> VolatilityDistribution {
    VolatilityDistribution {
        switches_0: (counts[0] as f64 / total_windows) * 100.0,
        switches_1_2: (counts[1] as f64 / total_windows) * 100.0,
        switches_3_4: (counts[2] as f64 / total_windows) * 100.0,
        switches_5_6: (counts[3] as f64 / total_windows) * 100.0,
        switches_7: (counts[4] as f64 / total_windows) * 100.0,
    }
}

pub fn calculate_distributions(volatilities: &[WindowVolatility], vectors: &[AimVector]) -> AimVolatilitySummary {
    let total = volatilities.len() as f64;

    if total == 0.0 {
        let empty_dist = VolatilityDistribution { switches_0: 0.0, switches_1_2: 0.0, switches_3_4: 0.0, switches_5_6: 0.0, switches_7: 0.0 };
        return AimVolatilitySummary {
            velocity_buckets: empty_dist.clone(),
            velocity_intensity: VelocityIntensityDistribution { major_adjustment: 0.0, minor_adjustment: 0.0, steady: 0.0 },
            angle: empty_dist.clone(),
            direction: empty_dist,
            texture_matrix: TextureMatrix { consistent: 0.0, flow_tech: 0.0, rhythmic_tech: 0.0, chaotic_tech: 0.0 },
            snap_flow: SnapFlowRatio { snap_aim: 0.0, flow_aim: 0.0 },
        };
    }

    // 1. Calculate Global Velocity Distribution (Per Note)
    let mut vel_bucket_counts = [0; 5];
    let v_all: Vec<f64> = vectors.iter().map(|v| if v.dt > 0.0 { v.norm_distance / v.dt } else { 0.0 }).collect();
    let v_mean = v_all.iter().sum::<f64>() / v_all.len() as f64;
    let v_std = (v_all.iter().map(|v| (v_mean - v).powi(2)).sum::<f64>() / v_all.len() as f64).sqrt();

    for v in v_all {
        let bucket = get_relative_velocity_bucket(v, v_mean, v_std);
        vel_bucket_counts[bucket as usize] += 1;
    }

    // 2. Window-based calculations
    let mut ang_counts = [0; 5];
    let mut dir_counts = [0; 5];
    let mut major = 0; let mut minor = 0; let mut steady = 0;
    let mut snap_count = 0; let mut flow_count = 0;
    let mut consistent = 0; let mut flow_tech = 0; let mut rhythmic_tech = 0; let mut chaotic_tech = 0;

    let map_to_index = |switches: u8| -> usize {
        match switches { 0 => 0, 1..=2 => 1, 3..=4 => 2, 5..=6 => 3, _ => 4 }
    };

    for w in volatilities {
        ang_counts[map_to_index(w.angle_switches)] += 1;
        dir_counts[map_to_index(w.alignment_switches)] += 1;

        // Intensity Logic (simplified to window average)
        if w.velocity_switches == 0 { steady += 1; }
        else if w.velocity_switches <= 3 { minor += 1; }
        else { major += 1; }

        if w.velocity_switches >= 4 || w.angle_switches >= 5 { snap_count += 1; } 
        else { flow_count += 1; }

        let v_high = w.velocity_switches >= 5;
        let a_high = w.angle_switches >= 5;
        match (v_high, a_high) {
            (false, false) => consistent += 1,
            (false, true)  => flow_tech += 1,
            (true, false)  => rhythmic_tech += 1,
            (true, true)   => chaotic_tech += 1,
        }
    }

    AimVolatilitySummary {
        velocity_buckets: to_distribution(&vel_bucket_counts, vectors.len() as f64),
        velocity_intensity: VelocityIntensityDistribution {
            major_adjustment: (major as f64 / total) * 100.0,
            minor_adjustment: (minor as f64 / total) * 100.0,
            steady: (steady as f64 / total) * 100.0,
        },
        angle: to_distribution(&ang_counts, total),
        direction: to_distribution(&dir_counts, total),
        texture_matrix: TextureMatrix {
            consistent: (consistent as f64 / total) * 100.0,
            flow_tech: (flow_tech as f64 / total) * 100.0,
            rhythmic_tech: (rhythmic_tech as f64 / total) * 100.0,
            chaotic_tech: (chaotic_tech as f64 / total) * 100.0,
        },
        snap_flow: SnapFlowRatio {
            snap_aim: (snap_count as f64 / total) * 100.0,
            flow_aim: (flow_count as f64 / total) * 100.0,
        },
    }
}

pub fn generate_aim_complexity_report(vectors: &[AimVector]) -> AimVolatilitySummary {
    let volatilities = calculate_volatilities(vectors);
    calculate_distributions(&volatilities, vectors)
}