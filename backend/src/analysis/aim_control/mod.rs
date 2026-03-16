pub mod spatial;
pub mod kinematics;
pub mod vectors;
pub mod endurance;

use rosu_pp::Beatmap;
use serde_json::{json, Value};

pub fn analyze(map: &Beatmap) -> Value {
    // 1. Spatial Geometry
    let spatial_vectors = spatial::calculate_spatial_vectors(map);
    
    if spatial_vectors.is_empty() {
        return json!({ "error": "Not enough objects for aim analysis" });
    }

    // 2. Base Kinematics
    let kinematics = kinematics::calculate_kinematics(&spatial_vectors);

    // 3. Advanced Vector Mechanics (Flips, Chirps, Alignment)
    let vector_data = vectors::calculate_vector_mechanics(&spatial_vectors);

    // 4. Endurance & Sustained Strain (EMA)
    let endurance_data = endurance::calculate_endurance(&spatial_vectors, &kinematics);

    // Prepare arrays for distribution grouping (to be handled in stage 3/4)
    let mut spacing_array: Vec<f64> = spatial_vectors.iter().map(|v| v.norm_distance).collect();
    let mut angle_array: Vec<f64> = spatial_vectors.iter().filter_map(|v| v.deflection_angle).collect();
    let mut velocity_array: Vec<f64> = kinematics.iter().map(|k| k.velocity).collect();

    // Sort for safe basic stat extraction (medians/percentiles)
    spacing_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angle_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
    velocity_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Assemble Stage 2 finalized payload
    json!({
        "spatial": {
            "total_movements": spatial_vectors.len(),
            "avg_spacing_d": spacing_array.iter().sum::<f64>() / spacing_array.len() as f64,
            "avg_angle": if !angle_array.is_empty() { angle_array.iter().sum::<f64>() / angle_array.len() as f64 } else { 0.0 },
        },
        "kinematics": {
            "avg_velocity": velocity_array.iter().sum::<f64>() / velocity_array.len() as f64,
        },
        "vectors": {
            "directional_flips": vector_data.flips,
            "directional_chirps": vector_data.chirps,
            "alignment": {
                "parallel": vector_data.alignment_parallel,
                "orthogonal": vector_data.alignment_orthogonal,
                "anti_symmetric": vector_data.alignment_anti_symmetric,
            }
        },
        "endurance": {
            "peak_strain": endurance_data.peak_strain,
            "time_under_tension_ms": endurance_data.time_under_tension,
            // We pass the raw EMA curve to the frontend to plot a line chart
            "strain_curve": endurance_data.ema_strain,
        }
    })
}