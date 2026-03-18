use serde::Serialize;
use super::spatial::AimVector;
use super::volatility::{calculate_volatilities, WindowVolatility};

#[derive(Debug, Clone, Serialize)] // Added Serialize here
#[serde(rename_all = "camelCase")] // Matches your project's naming convention
pub struct VolatilityDistribution {
    pub switches_0: f64,
    pub switches_1_2: f64,
    pub switches_3_4: f64,
    pub switches_5_6: f64,
    pub switches_7: f64,
}

#[derive(Debug, Clone, Serialize)] // Added Serialize here
#[serde(rename_all = "camelCase")] // Matches your project's naming convention
pub struct AimVolatilitySummary {
    pub relative_velocity: VolatilityDistribution,
    pub angle: VolatilityDistribution,
    pub direction: VolatilityDistribution,
}

/// Helper function to convert raw counts into percentages.
fn to_distribution(counts: &[usize; 5], total_windows: f64) -> VolatilityDistribution {
    VolatilityDistribution {
        switches_0: (counts[0] as f64 / total_windows) * 100.0,
        switches_1_2: (counts[1] as f64 / total_windows) * 100.0,
        switches_3_4: (counts[2] as f64 / total_windows) * 100.0,
        switches_5_6: (counts[3] as f64 / total_windows) * 100.0,
        switches_7: (counts[4] as f64 / total_windows) * 100.0,
    }
}

/// Takes the raw volatilities and calculates the distribution ratios (0 to 100%).
pub fn calculate_distributions(volatilities: &[WindowVolatility]) -> AimVolatilitySummary {
    let total = volatilities.len() as f64;
    // ... same safety check as before ...

    let mut vel_counts = [0; 5];
    let mut ang_counts = [0; 5];
    let mut dir_counts = [0; 5];

    let map_to_index = |switches: u8| -> usize {
        match switches {
            0 => 0,
            1..=2 => 1,
            3..=4 => 2,
            5..=6 => 3,
            7 => 4,
            _ => 4, // Safety fallback
        }
    };

    for w in volatilities {
        vel_counts[map_to_index(w.velocity_switches)] += 1;
        ang_counts[map_to_index(w.angle_switches)] += 1;
        dir_counts[map_to_index(w.alignment_switches)] += 1;
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