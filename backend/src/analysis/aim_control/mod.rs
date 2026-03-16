pub mod spatial;
pub mod kinematics;

use rosu_pp::Beatmap;
use serde_json::{json, Value};

pub fn analyze(map: &Beatmap) -> Value {
    // 1. Extract Spatial Vectors
    let vectors = spatial::calculate_spatial_vectors(map);
    
    // 2. Extract Base Kinematics
    let kinematics = kinematics::calculate_kinematics(&vectors);

    // TODO: Advanced Vectors (Flips, Chirps, Alignment)
    // TODO: Endurance EMA Strain

    // Return Stage 1 raw data payload for verification
    json!({
        "total_vectors": vectors.len(),
        "sample_vector_data": {
            "norm_distance_d": vectors.get(10).map(|v| v.norm_distance),
            "deflection_angle": vectors.get(10).and_then(|v| v.deflection_angle),
            "velocity": kinematics.get(10).map(|k| k.velocity),
            "momentum_retention": kinematics.get(10).and_then(|k| k.momentum_retention),
        }
    })
}