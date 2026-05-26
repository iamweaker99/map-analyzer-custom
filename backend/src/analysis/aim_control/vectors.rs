use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AimVector {
    pub start_time: f64,
    pub end_time: f64,             // True end time (accounts for slider duration)
    pub norm_distance: f64,        // Consistent naming for all files
    pub dx: f64,
    pub dy: f64,
    pub dt: f64,                   
    pub dt_break: f64,             
    pub velocity: f64,             
    pub deflection_angle: Option<f64>, 
    pub is_slider: bool,           
}

#[derive(Clone, Debug, Default)]
pub struct VectorData {
    pub flips: usize,
    pub chirps: usize,
    pub alignment_parallel: usize,
    pub alignment_orthogonal: usize,
    pub alignment_anti_symmetric: usize,
}

pub fn calculate_vector_mechanics(vectors: &[AimVector]) -> VectorData {
    let mut data = VectorData::default();
    let mut last_cross_sign = 0.0;

    for i in 0..vectors.len() {
        let curr = &vectors[i];

        if i > 0 {
            let prev = &vectors[i - 1];

            // FIXED: Changed .distance to .norm_distance
            if prev.norm_distance > 0.0 && curr.norm_distance > 0.0 {
                let prev_norm_x = prev.dx / prev.norm_distance;
                let prev_norm_y = prev.dy / prev.norm_distance;
                let curr_norm_x = curr.dx / curr.norm_distance;
                let curr_norm_y = curr.dy / curr.norm_distance;

                let dot_consecutive = (prev_norm_x * curr_norm_x) + (prev_norm_y * curr_norm_y);
                if dot_consecutive < -0.5 {
                    data.flips += 1;
                }

                let cross = (prev.dx * curr.dy) - (prev.dy * curr.dx);
                let sign = cross.signum();

                if last_cross_sign != 0.0 && sign != 0.0 && sign != last_cross_sign {
                    data.chirps += 1; 
                }
                
                if sign != 0.0 {
                    last_cross_sign = sign;
                }
            }
        }

        if i > 1 {
            let prev2 = &vectors[i - 2];

            // FIXED: Changed .distance to .norm_distance
            if prev2.norm_distance > 0.0 && curr.norm_distance > 0.0 {
                let prev2_norm_x = prev2.dx / prev2.norm_distance;
                let prev2_norm_y = prev2.dy / prev2.norm_distance;
                let curr_norm_x = curr.dx / curr.norm_distance;
                let curr_norm_y = curr.dy / curr.norm_distance;

                let dot_alignment = (prev2_norm_x * curr_norm_x) + (prev2_norm_y * curr_norm_y);

                if dot_alignment > 0.8 {
                    data.alignment_parallel += 1;
                } else if dot_alignment < -0.8 {
                    data.alignment_anti_symmetric += 1;
                } else if dot_alignment >= -0.3 && dot_alignment <= 0.3 {
                    data.alignment_orthogonal += 1;
                }
            }
        }
    }

    data
}