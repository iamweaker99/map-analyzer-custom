pub mod spatial;
pub mod kinematics;
pub mod vectors;
pub mod endurance;
pub mod buckets;
pub mod volatility;
pub mod statistics;

use rosu_pp::Beatmap;
use serde_json::{json, Value};

fn calculate_std_dev(data: &[f64], mean: f64) -> f64 {
    if data.is_empty() { return 0.0; }
    let variance: f64 = data.iter().map(|value| {
        let diff = mean - value;
        diff * diff
    }).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

pub fn analyze(map: &Beatmap) -> Value {
    let spatial_vectors = spatial::calculate_spatial_vectors(map);
    
    if spatial_vectors.is_empty() {
        return json!({ "error": "Not enough objects for aim analysis" });
    }

    let kinematics = kinematics::calculate_kinematics(&spatial_vectors);
    let vector_data = vectors::calculate_vector_mechanics(&spatial_vectors);
    let endurance_data = endurance::calculate_endurance(&spatial_vectors, &kinematics);

    let spacing_array: Vec<f64> = spatial_vectors.iter().map(|v| v.norm_distance).collect();
    let angle_array: Vec<f64> = spatial_vectors.iter().filter_map(|v| v.deflection_angle).collect();
    let velocity_array: Vec<f64> = kinematics.iter().map(|k| k.velocity).collect();

    let avg_spacing = spacing_array.iter().sum::<f64>() / spacing_array.len() as f64;
    let avg_angle = if !angle_array.is_empty() { angle_array.iter().sum::<f64>() / angle_array.len() as f64 } else { 0.0 };
    let avg_velocity = velocity_array.iter().sum::<f64>() / velocity_array.len() as f64;

    let velocity_std_dev = calculate_std_dev(&velocity_array, avg_velocity);

    let mut stacked = 0; let mut micro = 0; let mut flow = 0; let mut standard = 0; let mut large = 0;
    for &d in &spacing_array {
        if d <= 0.5 { stacked += 1; }
        else if d <= 1.25 { micro += 1; }
        else if d <= 2.5 { flow += 1; }
        else if d <= 4.5 { standard += 1; }
        else { large += 1; }
    }

    let mut linear = 0; let mut wide = 0; let mut acute = 0; let mut snap_backs = 0;
    for &a in &angle_array {
        if a <= 45.0 { linear += 1; } // 0-45 deg: Straight lines/gentle curves
        else if a <= 90.0 { wide += 1; } // 45-90 deg: Wide flow aim
        else if a <= 135.0 { acute += 1; } // 90-135 deg: Sharp tech angles
        else { snap_backs += 1; } // 135-180 deg: Reversing direction/1-2s
    }

    let mut v_sig_slower = 0; let mut v_slower = 0; let mut v_mean = 0; let mut v_faster = 0; let mut v_sig_faster = 0;
    for &v in &velocity_array {
        if v < avg_velocity - (1.5 * velocity_std_dev) { v_sig_slower += 1; }
        else if v < avg_velocity - (0.5 * velocity_std_dev) { v_slower += 1; }
        else if v <= avg_velocity + (0.5 * velocity_std_dev) { v_mean += 1; }
        else if v <= avg_velocity + (1.5 * velocity_std_dev) { v_faster += 1; }
        else { v_sig_faster += 1; }
    }

    json!({
        "spatial": {
            "total_movements": spatial_vectors.len(),
            "avg_spacing_d": avg_spacing,
            "avg_angle": avg_angle,
            "spacing_distribution": {
                "stacked": stacked,
                "micro": micro,
                "flow": flow,
                "standard": standard,
                "large": large
            },
            "angle_distribution": {
                "snap_backs": snap_backs,
                "acute": acute,
                "wide": wide,
                "linear": linear
            }
        },
        "kinematics": {
            "avg_velocity": avg_velocity,
            "velocity_std_dev": velocity_std_dev,
            "velocity_distribution": {
                "significantly_slower": v_sig_slower,
                "slower": v_slower,
                "mean": v_mean,
                "faster": v_faster,
                "significantly_faster": v_sig_faster
            }
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
            "strain_curve": endurance_data.ema_strain,
        }
    })
}