use serde::{Deserialize, Serialize};
use super::vectors::AimVector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACCVState {
    pub start_time: f64,
    pub spatial_cv: f64,    // V_d: Variance in spacing
    pub temporal_cv: f64,   // V_t: Variance in rhythm
    pub kinetic_var: f64,   // V_theta: Variance in angles
    pub total_complexity: f64, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACCVMetrics {
    pub peak_complexity: f64,         // 95th percentile: The hardest spikes
    pub sustained_complexity: f64,    // 50th percentile: The general map difficulty
    pub peak_spatial_cv: f64,         // 95th percentile spacing variance
    pub peak_temporal_cv: f64,        // 95th percentile rhythm variance
    pub peak_kinetic_var: f64,        // 95th percentile angle variance
}

// Helper function to calculate mean and standard deviation
fn calc_mean_and_std(data: &[f64]) -> (f64, f64) {
    if data.is_empty() { return (0.0, 0.0); }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    (mean, variance.sqrt())
}

pub fn calculate_accv(vectors: &[AimVector]) -> Vec<ACCVState> {
    let mut accv_states: Vec<ACCVState> = Vec::new();
    let window_size = 4; // Our standard sequence length

    if vectors.len() < window_size {
        return accv_states;
    }

    // Slide the window across the vector array
    for window in vectors.windows(window_size) {
        let mut is_valid = true;
        let mut contains_slider = false;
        
        // Anti-Symmetry & Geometry trackers
        let mut chirps = 0;
        let mut last_sign = 0.0;
        let mut high_deflection_count = 0;

        // 1. Evaluate Boundary Rules & Local Geometry
        for i in 0..window_size {
            if i > 0 {
                // Break threshold
                if window[i].dt_break > 1000.0 {
                    is_valid = false;
                    break;
                }

                // Calculate Chirps (Anti-Symmetry / Zig-Zags)
                let prev = &window[i - 1];
                let curr = &window[i];
                let cross = (prev.dx * curr.dy) - (prev.dy * curr.dx);
                let sign = cross.signum();
                if last_sign != 0.0 && sign != 0.0 && sign != last_sign {
                    chirps += 1; // Rotation direction inverted
                }
                if sign != 0.0 {
                    last_sign = sign;
                }
            }
            
            if window[i].is_slider { contains_slider = true; }
            
            // Track sharp turns (Deflection > 90 degrees means reversing direction)
            if let Some(angle) = window[i].deflection_angle {
                if angle > 90.0 { high_deflection_count += 1; }
            }
        }

        if !is_valid { continue; }

        let distances: Vec<f64> = window.iter().map(|v| v.norm_distance).collect();
        let times: Vec<f64> = window.iter().map(|v| v.dt).collect();
        let angles: Vec<f64> = window.iter().filter_map(|v| v.deflection_angle).collect();
        let velocities: Vec<f64> = window.iter().map(|v| v.velocity).collect();

        // 2. Base Variances
        let (mu_d, std_d) = calc_mean_and_std(&distances);
        let spatial_cv = if mu_d > 5.0 { std_d / mu_d } else { 0.0 };

        let (mu_t, std_t) = calc_mean_and_std(&times);
        let temporal_cv = if mu_t > 10.0 { std_t / mu_t } else { 0.0 };

        let (_, kinetic_var) = calc_mean_and_std(&angles);

        // 3. The Geometric Multiplier (Solving the Bookmaker Problem)
        // Escalates difficulty based on your defined progression of burst awkwardness
        let mut geometric_multiplier = 1.0;
        if high_deflection_count > 0 { 
            // Penalize sharp turning points in flow aim
            geometric_multiplier += 0.20 * high_deflection_count as f64; 
        }
        if chirps > 0 { 
            // Heavily penalize anti-symmetry / alternating rotation
            geometric_multiplier += 0.35 * chirps as f64; 
        }

        let base_complexity = spatial_cv + (temporal_cv * 1.5) + ((kinetic_var / 90.0) * geometric_multiplier);

        // 4. The Non-Linear Magnitude Scaler (Solving the Star Rating Compression)
        // An exponent of 1.8 exponentially separates high-velocity 9* aim from 6* aim
        let (mu_v, _) = calc_mean_and_std(&velocities);
        let magnitude_multiplier = 1.0 + mu_v.powf(1.8); 

        let mut total_complexity = base_complexity * magnitude_multiplier;

        // Apply Slider Leniency Boundary Rule
        if contains_slider {
            total_complexity *= 0.7;
        }

        accv_states.push(ACCVState {
            start_time: window[0].start_time,
            spatial_cv,
            temporal_cv,
            kinetic_var,
            total_complexity,
        });
    }

    accv_states
}

// Helper function to calculate the N-th percentile of a dataset
fn calculate_percentile(data: &[f64], percentile: f64) -> f64 {
    if data.is_empty() { return 0.0; }
    
    // Create a mutable copy to sort
    let mut sorted_data = data.to_vec();
    // Safely handle any potential NaNs or Infs that might have snuck through
    sorted_data.retain(|x| x.is_finite()); 
    
    if sorted_data.is_empty() { return 0.0; }

    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let index = ((percentile / 100.0) * (sorted_data.len() - 1) as f64).round() as usize;
    sorted_data[index]
}

pub fn aggregate_accv(states: &[ACCVState]) -> ACCVMetrics {
    if states.is_empty() {
        return ACCVMetrics {
            peak_complexity: 0.0,
            sustained_complexity: 0.0,
            peak_spatial_cv: 0.0,
            peak_temporal_cv: 0.0,
            peak_kinetic_var: 0.0,
        };
    }

    let complexities: Vec<f64> = states.iter().map(|s| s.total_complexity).collect();
    let spatial_cvs: Vec<f64> = states.iter().map(|s| s.spatial_cv).collect();
    let temporal_cvs: Vec<f64> = states.iter().map(|s| s.temporal_cv).collect();
    let kinetic_vars: Vec<f64> = states.iter().map(|s| s.kinetic_var).collect();

    ACCVMetrics {
        peak_complexity: calculate_percentile(&complexities, 95.0),
        sustained_complexity: calculate_percentile(&complexities, 50.0),
        peak_spatial_cv: calculate_percentile(&spatial_cvs, 95.0),
        peak_temporal_cv: calculate_percentile(&temporal_cvs, 95.0),
        peak_kinetic_var: calculate_percentile(&kinetic_vars, 95.0),
    }
}