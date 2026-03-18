use super::spatial::AimVector;
use super::volatility::{calculate_volatilities, WindowVolatility};

#[derive(Debug, Clone)]
pub struct VolatilityDistribution {
    pub switches_0: f64,
    pub switches_1: f64,
    pub switches_2: f64,
    pub switches_3: f64,
    pub switches_more_than_3: f64, // Represents 4, 5, 6, 7
}

#[derive(Debug, Clone)]
pub struct AimVolatilitySummary {
    pub relative_velocity: VolatilityDistribution,
    pub angle: VolatilityDistribution,
    pub direction: VolatilityDistribution,
}

/// Helper function to convert raw counts into percentages.
fn to_distribution(counts: &[usize; 5], total_windows: f64) -> VolatilityDistribution {
    VolatilityDistribution {
        switches_0: (counts[0] as f64 / total_windows) * 100.0,
        switches_1: (counts[1] as f64 / total_windows) * 100.0,
        switches_2: (counts[2] as f64 / total_windows) * 100.0,
        switches_3: (counts[3] as f64 / total_windows) * 100.0,
        switches_more_than_3: (counts[4] as f64 / total_windows) * 100.0,
    }
}

/// Takes the raw volatilities and calculates the distribution ratios (0 to 100%).
pub fn calculate_distributions(volatilities: &[WindowVolatility]) -> AimVolatilitySummary {
    let total = volatilities.len() as f64;

    // Safety check: if map is too short to have any windows
    if total == 0.0 {
        let empty_dist = VolatilityDistribution {
            switches_0: 0.0, switches_1: 0.0, switches_2: 0.0,
            switches_3: 0.0, switches_more_than_3: 0.0,
        };
        return AimVolatilitySummary {
            relative_velocity: empty_dist.clone(),
            angle: empty_dist.clone(),
            direction: empty_dist,
        };
    }

    // Indices: 0, 1, 2, 3, 4 (where 4 represents > 3)
    let mut vel_counts = [0; 5];
    let mut ang_counts = [0; 5];
    let mut dir_counts = [0; 5];

    for w in volatilities {
        // .min(4) caps the value at 4, so 4, 5, 6, 7 all go into index 4
        vel_counts[(w.velocity_switches as usize).min(4)] += 1;
        ang_counts[(w.angle_switches as usize).min(4)] += 1;
        dir_counts[(w.alignment_switches as usize).min(4)] += 1;
    }

    AimVolatilitySummary {
        relative_velocity: to_distribution(&vel_counts, total),
        angle: to_distribution(&ang_counts, total),
        direction: to_distribution(&dir_counts, total),
    }
}

/// Orchestrator function: Pass in the raw vectors, get the final summary.
/// This is the function you will call from your main map analysis pipeline.
pub fn generate_aim_complexity_report(vectors: &[AimVector]) -> AimVolatilitySummary {
    // Stage 2: Get raw window statistics
    let volatilities = calculate_volatilities(vectors);
    
    // Stage 3: Aggregate into final distributions
    calculate_distributions(&volatilities)
}