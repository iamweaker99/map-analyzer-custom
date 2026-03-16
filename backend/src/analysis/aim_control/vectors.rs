use super::spatial::AimVector;

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

        // Flips & Chirps (Requires n and n-1)
        if i > 0 {
            let prev = &vectors[i - 1];

            if prev.distance > 0.0 && curr.distance > 0.0 {
                let prev_norm_x = prev.dx / prev.distance;
                let prev_norm_y = prev.dy / prev.distance;
                let curr_norm_x = curr.dx / curr.distance;
                let curr_norm_y = curr.dy / curr.distance;

                // 1. Directional Flip (Dot Product of consecutive normalized vectors)
                let dot_consecutive = (prev_norm_x * curr_norm_x) + (prev_norm_y * curr_norm_y);
                if dot_consecutive < -0.5 {
                    data.flips += 1;
                }

                // 2. Directional Chirp (Cross Product Sign Inversion)
                // Positive = Clockwise, Negative = Anticlockwise
                let cross = (prev.dx * curr.dy) - (prev.dy * curr.dx);
                let sign = cross.signum();

                if last_cross_sign != 0.0 && sign != 0.0 && sign != last_cross_sign {
                    data.chirps += 1; // Rotation direction inverted
                }
                
                if sign != 0.0 {
                    last_cross_sign = sign;
                }
            }
        }

        // Vector Alignment / Autocorrelation (Requires n and n-2)
        if i > 1 {
            let prev2 = &vectors[i - 2];

            if prev2.distance > 0.0 && curr.distance > 0.0 {
                let prev2_norm_x = prev2.dx / prev2.distance;
                let prev2_norm_y = prev2.dy / prev2.distance;
                let curr_norm_x = curr.dx / curr.distance;
                let curr_norm_y = curr.dy / curr.distance;

                // Dot product skipping one note to detect symmetry/repeating patterns
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