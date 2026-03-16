use super::spatial::AimVector;

#[derive(Clone, Debug)]
pub struct KinematicData {
    pub velocity: f64, // Pixels per ms
    pub momentum_retention: Option<f64>, // |V2| / |V1|
}

pub fn calculate_kinematics(vectors: &[AimVector]) -> Vec<KinematicData> {
    let mut kinematics = Vec::new();

    for i in 0..vectors.len() {
        let curr_vec = &vectors[i];
        let velocity = curr_vec.distance / curr_vec.dt;

        let mut momentum_retention = None;

        if i > 0 {
            let prev_vec = &vectors[i - 1];
            let prev_velocity = prev_vec.distance / prev_vec.dt;
            
            if prev_velocity > 0.0 {
                momentum_retention = Some(velocity / prev_velocity);
            }
        }

        kinematics.push(KinematicData {
            velocity,
            momentum_retention,
        });
    }

    kinematics
}