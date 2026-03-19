use super::{kinematics::KinematicData, vectors::AimVector};
use std::f64::consts::LN_2;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct StrainPoint {
    pub time: f64,
    pub strain: f64,
}

#[derive(Clone, Debug)]
pub struct EnduranceData {
    pub ema_strain: Vec<StrainPoint>,
    pub peak_strain: f64,
    pub time_under_tension: f64, 
}

pub fn calculate_endurance(vectors: &[AimVector], kinematics: &[KinematicData]) -> EnduranceData {
    let mut ema_strain = Vec::with_capacity(vectors.len());
    let mut current_strain = 0.0;
    let mut peak_strain = 0.0;
    
    let half_life_ms = 500.0;
    let lambda = LN_2 / half_life_ms;

    for i in 0..vectors.len() {
        let vec = &vectors[i];
        let kin = &kinematics[i];

        let angle_penalty = vec.deflection_angle.unwrap_or(0.0) / 180.0;
        let mechanical_cost = kin.velocity * (1.0 + angle_penalty);

        current_strain = current_strain * (-lambda * vec.dt).exp() + mechanical_cost;
        
        ema_strain.push(StrainPoint {
            time: vec.end_time,
            strain: current_strain,
        });

        if current_strain > peak_strain {
            peak_strain = current_strain;
        }
    }

    let threshold = peak_strain * 0.5;
    let mut time_under_tension = 0.0;

    for i in 0..vectors.len() {
        if ema_strain[i].strain > threshold {
            time_under_tension += vectors[i].dt;
        }
    }

    EnduranceData {
        ema_strain,
        peak_strain,
        time_under_tension,
    }
}