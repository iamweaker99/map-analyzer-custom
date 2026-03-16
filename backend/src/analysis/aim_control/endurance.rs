use super::{kinematics::KinematicData, spatial::AimVector};
use std::f64::consts::LN_2;

#[derive(Clone, Debug)]
pub struct EnduranceData {
    pub ema_strain: Vec<f64>,
    pub peak_strain: f64,
    pub time_under_tension: f64, // ms spent above 80% peak
}

pub fn calculate_endurance(vectors: &[AimVector], kinematics: &[KinematicData]) -> EnduranceData {
    let mut ema_strain = Vec::with_capacity(vectors.len());
    let mut current_strain = 0.0;
    let mut peak_strain = 0.0;
    
    // Decay constant (lambda) based on a 500ms half-life
    let half_life_ms = 500.0;
    let lambda = LN_2 / half_life_ms;

    for i in 0..vectors.len() {
        let vec = &vectors[i];
        let kin = &kinematics[i];

        // Base mechanical cost: Velocity combined with an angle penalty.
        // Sharper angles (closer to 180 degrees) increase strain up to 2x.
        let angle_penalty = vec.deflection_angle.unwrap_or(0.0) / 180.0;
        let mechanical_cost = kin.velocity * (1.0 + angle_penalty);

        // Apply exponential decay over the time gap (dt), then add new cost
        current_strain = current_strain * (-lambda * vec.dt).exp() + mechanical_cost;
        ema_strain.push(current_strain);

        if current_strain > peak_strain {
            peak_strain = current_strain;
        }
    }

    // Calculate Time Under Tension (TUT)
    let threshold = peak_strain * 0.8;
    let mut time_under_tension = 0.0;

    for i in 0..vectors.len() {
        if ema_strain[i] > threshold {
            time_under_tension += vectors[i].dt;
        }
    }

    EnduranceData {
        ema_strain,
        peak_strain,
        time_under_tension,
    }
}