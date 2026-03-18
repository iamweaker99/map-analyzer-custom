use super::spatial::AimVector;
use super::buckets::{
    get_relative_velocity_bucket, get_angle_bucket, get_alignment_bucket,
    RelativeVelocityBucket, AngleBucket, AlignmentBucket,
};

#[derive(Debug, Clone)]
pub struct WindowVolatility {
    pub velocity_switches: u8,
    pub angle_switches: u8,
    pub alignment_switches: u8,
}

/// Helper function to calculate mean and standard deviation of a slice of f64.
fn calculate_mean_and_std_dev(data: &[f64]) -> (f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0);
    }
    
    let count = data.len() as f64;
    let mean = data.iter().sum::<f64>() / count;
    
    let variance = data.iter()
        .map(|value| {
            let diff = mean - *value;
            diff * diff
        })
        .sum::<f64>() / count;
        
    (mean, variance.sqrt())
}

/// Slides an 8-note window across the vectors to calculate volatility (switch counts).
pub fn calculate_volatilities(vectors: &[AimVector]) -> Vec<WindowVolatility> {
    let mut volatilities = Vec::new();
    let window_size = 8;

    // If the map has fewer than 8 vectors, we can't form a valid window.
    if vectors.len() < window_size {
        return volatilities;
    }

    // Slide window with stride = 1
    for window in vectors.windows(window_size) {
        // 1. Calculate velocities (norm_distance / dt) for this specific window
        let velocities: Vec<f64> = window.iter()
            .map(|v| if v.dt > 0.0 { v.norm_distance / v.dt } else { 0.0 })
            .collect();

        // 2. Get local Mean and Sigma
        let (mean, std_dev) = calculate_mean_and_std_dev(&velocities);

        // 3. Map vectors to their respective buckets
        let vel_buckets: Vec<RelativeVelocityBucket> = velocities.iter()
            .map(|&v| get_relative_velocity_bucket(v, mean, std_dev))
            .collect();
            
        let ang_buckets: Vec<AngleBucket> = window.iter()
            .map(|v| get_angle_bucket(v.deflection_angle))
            .collect();
            
        let align_buckets: Vec<AlignmentBucket> = window.iter()
            .map(|v| get_alignment_bucket(v.deflection_angle))
            .collect();

        // 4. Count the switches (transitions) within this 8-note window
        let mut vel_switches = 0;
        let mut ang_switches = 0;
        let mut align_switches = 0;

        // An 8-note window has 7 potential transitions (index 0->1, 1->2, ..., 6->7)
        for i in 0..(window_size - 1) {
            // Velocity transition
            if vel_buckets[i] != vel_buckets[i + 1] {
                vel_switches += 1;
            }

            // Angle transition (ignore transitions to/from `None` at the very start/end of the map)
            if ang_buckets[i] != ang_buckets[i + 1] 
                && ang_buckets[i] != AngleBucket::None 
                && ang_buckets[i + 1] != AngleBucket::None 
            {
                ang_switches += 1;
            }

            // Alignment transition (ignore `None`)
            if align_buckets[i] != align_buckets[i + 1] 
                && align_buckets[i] != AlignmentBucket::None 
                && align_buckets[i + 1] != AlignmentBucket::None 
            {
                align_switches += 1;
            }
        }

        // 5. Store the result for this window
        volatilities.push(WindowVolatility {
            velocity_switches: vel_switches,
            angle_switches: ang_switches,
            alignment_switches: align_switches,
        });
    }

    volatilities
}